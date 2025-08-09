//! The cluster is a server that hosts many worlds and also knows about other clusters.
//! The cluster gets this information from the master server.

use sustenet_shared::ServerPlugin;
use sustenet_shared::config::cluster::{ Settings, read };
use sustenet_shared::logging::{ LogType, Logger };
use sustenet_shared::network::ClusterInfo;
use sustenet_shared::packets::Diagnostics;

use std::collections::HashMap;
use std::io::Error;
use std::sync::LazyLock;

use bytes::Bytes;
use tokio::io;
use tokio::net::TcpListener;
use tokio::sync::mpsc;

use crate::cluster_client::ClusterClient;
use crate::master_connection::MasterConnection;

/// Global logger for the cluster module.
pub static LOGGER: LazyLock<Logger> = LazyLock::new(|| Logger::new(LogType::Cluster));

/// Events emitted by the cluster server to notify listeners.
#[derive(Debug, Clone)]
pub enum ClusterEvent {
    MasterConnected,
    MasterDisconnected,
    MasterCommandSent(u8),
    MasterMessageSent(Bytes),
    MasterCommandReceived(u8),
    MasterMessageReceived(Bytes),

    /// When a connection is established with a client or server.
    Connected(u64),
    /// When a connection is closed with a client or server.
    Disconnected(u64),

    DiagnosticsReceived(Diagnostics, Bytes),
    Shutdown,
    Error(String),
}

/// Handles connections and interactions with Cluster Servers and Clients.
pub struct ClusterServer<P: ServerPlugin + Send + Sync> {
    _plugin: P,

    _max_connections: u32,
    bind: String,
    port: u16,

    // sender: mpsc::Sender<Bytes>,
    event_tx: mpsc::Sender<ClusterEvent>,
    _event_rx: mpsc::Receiver<ClusterEvent>,

    connections: HashMap<u64, ClusterClient>,
    /// Only used to store cluster servers so clients can switch between them.
    _cluster_servers: Vec<ClusterInfo>,
    _master_connection: MasterConnection,
    _next_id: u64,
}

impl<P: ServerPlugin + Send + Sync> ClusterServer<P> {
    pub async fn new(settings: Settings, plugin: P) -> io::Result<Self> {
        let (event_tx, event_rx) = mpsc::channel::<ClusterEvent>(16);

        let port = settings.port;
        let master_connection = MasterConnection::connect(
            &settings.master_ip,
            settings.master_port
        ).await?;

        Ok(Self {
            _plugin: plugin,

            _max_connections: settings.max_connections,
            bind: settings.bind,
            port,

            event_tx,
            _event_rx: event_rx,

            connections: HashMap::new(),
            _cluster_servers: Vec::new(),
            _master_connection: master_connection,
            _next_id: 0,
        })
    }

    pub async fn new_from_cli() -> io::Result<Self> {
        // TODO (low priority): Load the configuration from CLI arguments
        todo!()
    }

    pub async fn new_from_config(plugin: P) -> io::Result<Self> {
        let settings = read();

        Self::new(settings, plugin).await
    }

    ///
    pub async fn start(&mut self) -> io::Result<()> {
        // Create Listener
        let addr = format!("{}:{}", self.bind, self.port);
        let listener = match TcpListener::bind(&addr).await {
            Ok(l) => {
                LOGGER.success(&format!("Cluster server started on {addr}"));
                l
            }
            Err(e) => {
                LOGGER.error(&format!("Failed to bind to {addr}"));
                return Err(Error::new(e.kind(), format!("Failed to bind to ({addr}): {e}")));
            }
        };

        // TODO: Improve starting here.
        loop {
            let (stream, peer) = match listener.accept().await {
                Ok(pair) => pair,
                Err(e) => {
                    LOGGER.error(&format!("Failed to accept connection: {e}"));
                    continue;
                }
            };
            LOGGER.debug(&format!("Accepted connection from {peer}"));

            // TODO: This is one the right path but AI did this. Move it to a struct.

            // Create a new ConnectionInfo instance
            let id = 0;
            let connection = match ClusterClient::new(id, stream, self.event_tx.clone()).await {
                Ok(c) => c,
                Err(e) => {
                    LOGGER.error(&format!("Failed to create connection: {e}"));
                    continue;
                }
            };

            // Store the connection in the connections map
            self.connections.insert(id, connection);
        }
    }

    // TODO: Add a tick
    // async fn tick(&mut self) -> io::Result<()> {
    //     LOGGER.debug("Ticking cluster server...");
    //     Ok(())
    // }
}
