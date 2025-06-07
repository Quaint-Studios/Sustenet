//! The master serve acts as a load balancer.
//!
//! When a client connects to it, it will redirect them to a registered cluster.
//!
//! The focus for the Master Server is to accept connections fast. So it should
//! stray away from doing too much work and it should distribute users to other
//! servers as fast as possible.

use crate::master_client::MasterClient;
use sustenet_shared::config::master::{ Settings, read };
use sustenet_shared::logging::{ LogType, Logger };
use sustenet_shared::lselect;
use sustenet_shared::network::ClusterInfo;
use sustenet_shared::packets::Diagnostics;

use std::collections::HashMap;
use std::io::Error;
use std::net::SocketAddr;
use std::sync::LazyLock;

use bytes::Bytes;
use tokio::io;
use tokio::net::{ TcpListener, TcpStream };
use tokio::sync::mpsc;

/// Global logger for the master module.
pub static LOGGER: LazyLock<Logger> = LazyLock::new(|| Logger::new(LogType::Master));

#[derive(Debug, Clone)]
pub enum MasterEvent {
    /// When a connection is established with a client or server.
    Connected(u64),
    /// When a connection is closed with a client or server.
    Disconnected(u64),

    /// When a cluster server is initialized with a passphrase.
    ClusterInit(u64, [u8; 20]),
    /// When a cluster server answer the passphrase correctly.
    ClusterRegistered(u64, String),
    /// When a cluster server fails to register with the master server.
    /// This is usually due to a wrong passphrase. But it can also be due to a timeout.
    ClusterRegistrationFailed(u64),
    
    DiagnosticsReceived(Diagnostics, Bytes),
    Shutdown,
    Error(String),
}

/// Handles connections and interactions with Cluster Servers and Clients.
pub struct MasterServer {
    max_connections: u32,
    bind: String,
    port: u16,

    // sender: mpsc::Sender<Bytes>,
    event_tx: mpsc::Sender<MasterEvent>,
    event_rx: mpsc::Receiver<MasterEvent>,

    connections: HashMap<u64, MasterClient>,
    cluster_servers: HashMap<u64, ClusterInfo>,
    cluster_passphrases: HashMap<u64, [u8; 20]>,
    next_id: u64,
}

impl MasterServer {
    pub async fn new(settings: Settings) -> io::Result<Self> {
        let (event_tx, event_rx) = mpsc::channel::<MasterEvent>(10000);

        Ok(Self {
            max_connections: settings.max_connections,
            bind: settings.bind,
            port: settings.port,

            event_tx,
            event_rx,

            connections: HashMap::new(),
            cluster_servers: HashMap::new(),
            cluster_passphrases: HashMap::new(),
            next_id: 0,
        })
    }

    pub async fn new_from_cli() -> io::Result<Self> {
        // TODO (low priority): Load the configuration from CLI arguments
        todo!()
    }

    pub async fn new_from_config() -> io::Result<Self> {
        let settings = read();

        Self::new(settings).await
    }

    ///
    pub async fn start(&mut self) -> io::Result<()> {
        // Create Listener
        let addr = format!("{}:{}", self.bind, self.port);
        let listener = match TcpListener::bind(&addr).await {
            Ok(l) => {
                LOGGER.success(&format!("Master server started on {addr}"));
                l
            }
            Err(e) => {
                LOGGER.error(&format!("Failed to bind to {addr}"));
                return Err(Error::new(e.kind(), format!("Failed to bind to ({addr}): {e}")));
            }
        };

        lselect!(
            event = self.event_rx.recv() => {
                if let Some(event) = event {
                    if !self.handle_events(event).await? {
                        println!("Cleaning up master server...");
                        break;
                    };
                }
            }
            res = listener.accept() => self.handle_listener(res).await?
        );

        Ok(())
    }

    /// TODO
    // async fn tick() {}
    pub async fn handle_events(&mut self, event: MasterEvent) -> io::Result<bool> {
        match event {
            MasterEvent::Connected(id) => {
                LOGGER.debug(&format!("Client #{id} connected"));
            }
            MasterEvent::Disconnected(id) => {
                // The connection is already scheduled to close, so no need
                // to call close() on the MasterClient.
                if self.connections.remove(&id).is_none() {
                    LOGGER.warning(&format!("Disconnected client #{id} not found"));
                    return Ok(true);
                }
                LOGGER.debug(&format!("Client #{id} disconnected"));
            }
            MasterEvent::ClusterInit(id, passphrase) => {
                LOGGER.debug(&format!("Cluster #{id} initialized with passphrase: {passphrase:?}"));
                self.cluster_passphrases.insert(id, passphrase);
            }
            MasterEvent::ClusterRegistered(id, name) => {
                LOGGER.success(&format!("Cluster ({name}) registered with ID #{id}"));
            }
            MasterEvent::ClusterRegistrationFailed(id) => {
                LOGGER.error(&format!("Cluster registration failed for ID {id}"));
            }
            MasterEvent::DiagnosticsReceived(diagnostics, _bytes) => {
                LOGGER.debug(&format!("Diagnostics received: {diagnostics:?}"));
            }
            MasterEvent::Error(msg) => {
                LOGGER.error(&format!("Error: {msg}"));
            }
            MasterEvent::Shutdown => {
                LOGGER.info("Received shutdown event, cleaning up...");
                return Ok(false);
            }
        }
        Ok(true)
    }

    pub async fn handle_listener(
        &mut self,
        res: io::Result<(TcpStream, SocketAddr)>
    ) -> io::Result<()> {
        let (mut stream, peer) = match res {
            Ok(pair) => pair,
            Err(e) => {
                LOGGER.error(&format!("Failed to accept connection: {e}"));
                return Err(Error::new(e.kind(), format!("Failed to accept connection: {e}")));
            }
        };

        if self.max_connections != 0 && (self.connections.len() as u32) >= self.max_connections {
            LOGGER.warning("Max connections reached, rejecting new connection");
            let _ = io::AsyncWriteExt::shutdown(&mut stream).await;
            return Ok(());
        }

        // Add connection
        let connection = MasterClient::new(self.next_id, stream, self.event_tx.clone()).await?;
        self.connections.insert(self.next_id, connection);

        LOGGER.debug(&format!("Accepted connection from {peer}"));
        let _ = self.event_tx.send(MasterEvent::Connected(self.next_id));
        self.next_id += 1;

        Ok(())
    }

    /// Sends a message to a specific client ID.
    pub async fn send_to(&self, id: &u64, bytes: Bytes) -> io::Result<()> {
        if let Some(client) = self.connections.get(id) {
            client.send(bytes).await?;
        } else {
            LOGGER.warning(&format!("Client {id} not found"));
            return Err(Error::new(std::io::ErrorKind::NotFound, format!("Client {id} not found")));
        }
        Ok(())
    }

    /// Sends a message to all connections.
    pub async fn send_to_all(&self, bytes: Bytes) -> io::Result<()> {
        for client in self.connections.values() {
            client.send(bytes.clone()).await?;
        }
        Ok(())
    }

    /// Sends a message to all cluster servers.
    pub async fn send_to_clusters(&self, bytes: Bytes) -> io::Result<()> {
        for cluster in self.cluster_servers.values() {
            if let Err(e) = self.send_to(&cluster.id, bytes.clone()).await {
                LOGGER.error(&format!("Failed to send message to cluster {}: {e}", cluster.name));
            }
        }
        Ok(())
    }

    pub async fn cleanup(&mut self) {
        LOGGER.info("Shutting down master server, closing all connections...");
        // Stop listening for new connections.
        // TODO: This might be doing nothing...
        if let Err(e) = self.event_tx.send(MasterEvent::Shutdown).await {
            LOGGER.error(&format!("Failed to send shutdown event: {e}"));
        }

        // Close all connections for shutdown.
        for (id, client) in self.connections.drain() {
            if let Err(e) = client.close().await {
                LOGGER.error(&format!("Failed to close connection #{id}: {e}"));
            }
        }

        LOGGER.cleanup();
    }
}
