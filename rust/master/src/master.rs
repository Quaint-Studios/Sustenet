//! The master serve acts as a load balancer.
//! When a client connects to it, it will redirect them to a registered cluster.

use crate::master_client::MasterClient;
use sustenet_shared::logging::{ LogType, Logger };
use sustenet_shared::lselect;
use sustenet_shared::network::ClusterInfo;
use sustenet_shared::packets::Diagnostics;
use sustenet_shared::utils::constants::{ self, DEFAULT_IP };

use std::collections::HashMap;
use std::io::{ Error, ErrorKind };
use std::net::SocketAddr;
use std::sync::LazyLock;

use bytes::Bytes;
use tokio::io;
use tokio::net::{ TcpListener, TcpStream };
use tokio::sync::broadcast;

/// Global logger for the master module.
pub static LOGGER: LazyLock<Logger> = LazyLock::new(|| Logger::new(LogType::Master));

/// Events emitted by the master server to notify listeners.
#[derive(Debug, Clone)]
pub enum MasterEvent {
    /// When a connection is established with a client or server.
    Connected(u64),
    /// When a connection is closed with a client or server.
    Disconnected(u64),
    ClusterRegistered(u64, String),
    ClusterRegistrationFailed(u64),
    DiagnosticsReceived(Diagnostics, Bytes),
    Shutdown,
    Error(String),
}

/// Handles connections and interactions with Cluster Servers and Clients.
pub struct MasterServer {
    max_connections: u32,
    port: u16,

    // sender: mpsc::Sender<Bytes>,
    event_tx: broadcast::Sender<MasterEvent>,
    event_rx: broadcast::Receiver<MasterEvent>,

    connections: HashMap<u64, MasterClient>,
    cluster_servers: HashMap<u64, ClusterInfo>,
    next_id: u64,
}

impl MasterServer {
    pub async fn new(/* config: Config */) -> io::Result<Self> {
        // Load the configuration from a file or environment variables
        // For now, we'll use a default port
        let max_connections = 0; // TODO: Load from config
        let port = constants::MASTER_PORT; // TODO: Load from config

        let (event_tx, event_rx) = broadcast::channel::<MasterEvent>(16);

        Ok(Self {
            max_connections,
            port,

            event_tx,
            event_rx,

            connections: HashMap::new(),
            cluster_servers: HashMap::new(),
            next_id: 0,
        })
    }

    pub async fn new_from_cli() -> io::Result<Self> {
        // TODO (low priority): Load the configuration from CLI arguments
        todo!()
    }

    pub async fn new_from_config() -> io::Result<Self> {
        // TODO: Load the configuration from a file
        Self::new().await
    }

    ///
    pub async fn start(&mut self) -> io::Result<()> {
        // Create Listener
        let addr = format!("{}:{}", DEFAULT_IP, self.port);
        let listener = match TcpListener::bind(&addr).await {
            Ok(l) => {
                LOGGER.success(&format!("Master server started on {addr}")).await;
                l
            }
            Err(e) => {
                LOGGER.error(&format!("Failed to bind to {addr}")).await;
                return Err(Error::new(e.kind(), format!("Failed to bind to ({addr}): {e}")));
            }
        };

        lselect!(
            event = self.event_rx.recv() => {
                if let Ok(event) = event {
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
                LOGGER.debug(&format!("Client #{id} connected")).await;
            }
            MasterEvent::Disconnected(id) => {
                // The connection is already scheduled to close, so no need
                // to call close() on the MasterClient.
                if self.connections.remove(&id).is_none() {
                    LOGGER.warning(&format!("Disconnected client #{id} not found")).await;
                    return Ok(true);
                }
                LOGGER.debug(&format!("Client #{id} disconnected")).await;
            }
            MasterEvent::ClusterRegistered(id, name) => {
                LOGGER.success(&format!("Cluster ({name}) registered with ID #{id}")).await;
            }
            MasterEvent::ClusterRegistrationFailed(id) => {
                LOGGER.error(&format!("Cluster registration failed for ID {id}")).await;
            }
            MasterEvent::DiagnosticsReceived(diagnostics, _bytes) => {
                LOGGER.debug(&format!("Diagnostics received: {diagnostics:?}")).await;
            }
            MasterEvent::Error(msg) => {
                LOGGER.error(&format!("Error: {msg}")).await;
            }
            MasterEvent::Shutdown => {
                LOGGER.info("Received shutdown event, cleaning up...").await;
                return Ok(false);
            }
        }
        Ok(true)
    }

    pub async fn handle_listener(
        &mut self,
        res: io::Result<(TcpStream, SocketAddr)>
    ) -> io::Result<()> {
        if self.max_connections != 0 && (self.connections.len() as u32) >= self.max_connections {
            LOGGER.warning("Max connections reached, rejecting new connection").await;
            return Ok(());
        }

        let (stream, peer) = match res {
            Ok(pair) => pair,
            Err(e) => {
                LOGGER.error(&format!("Failed to accept connection: {e}")).await;
                return Err(Error::new(e.kind(), format!("Failed to accept connection: {e}")));
            }
        };

        // Add connection
        let connection = MasterClient::new(self.next_id, stream, self.event_tx.clone()).await?;
        self.connections.insert(self.next_id, connection);

        LOGGER.debug(&format!("Accepted connection from {peer}")).await;
        let _ = self.event_tx.send(MasterEvent::Connected(self.next_id));
        self.next_id += 1;

        Ok(())
    }

    /// Sends a message to a specific client.
    pub async fn send(client: &MasterClient, bytes: Bytes) -> io::Result<()> {
        if let Err(e) = client.send(bytes).await {
            LOGGER.error(&format!("Failed to send message to client: {e}")).await;
            return Err(
                Error::new(ErrorKind::Other, format!("Failed to send message to client: {e}"))
            );
        }
        Ok(())
    }

    /// Sends a message to a specific client ID.
    pub async fn send_to(&self, id: &u64, bytes: Bytes) -> io::Result<()> {
        if let Some(client) = self.connections.get(id) {
            Self::send(&client, bytes).await?;
        } else {
            LOGGER.warning(&format!("Client {id} not found")).await;
            return Err(Error::new(std::io::ErrorKind::NotFound, format!("Client {id} not found")));
        }
        Ok(())
    }

    /// Sends a message to all connections.
    pub async fn send_to_all(&self, bytes: Bytes) -> io::Result<()> {
        for client in self.connections.values() {
            Self::send(client, bytes.clone()).await?;
        }
        Ok(())
    }

    /// Sends a message to all cluster servers.
    pub async fn send_to_clusters(&self, bytes: Bytes) -> io::Result<()> {
        for cluster in self.cluster_servers.values() {
            if let Err(e) = self.send_to(&cluster.id, bytes.clone()).await {
                LOGGER.error(
                    &format!("Failed to send message to cluster {}: {e}", cluster.name)
                ).await;
            }
        }
        Ok(())
    }

    pub async fn cleanup(&mut self) {
        LOGGER.info("Shutting down master server, closing all connections...").await;
        // Stop listening for new connections.
        // TODO: This might be doing nothing...
        if let Err(e) = self.event_tx.send(MasterEvent::Shutdown) {
            LOGGER.error(&format!("Failed to send shutdown event: {e}")).await;
        }

        // Close all connections for shutdown.
        for (id, client) in self.connections.drain() {
            if let Err(e) = client.close().await {
                LOGGER.error(&format!("Failed to close connection #{id}: {e}")).await;
            }
        }

        LOGGER.cleanup().await;
    }
}
