//! The cluster is a server that hosts many worlds and also knows about other clusters.
//! The cluster gets this information from the master server.

use sustenet_shared::logging::{ LogType, Logger };
use sustenet_shared::network::ClusterInfo;
use sustenet_shared::packets::{ ClusterSetup, Connection, Diagnostics };
use sustenet_shared::utils::constants;

use std::collections::BTreeSet;
use std::io::Error;
use std::sync::LazyLock;

use bytes::Bytes;
use dashmap::DashMap;
use tokio::io::{ self, AsyncReadExt, AsyncWriteExt };
use tokio::net::{ TcpListener, TcpStream };
use tokio::sync::{ broadcast, mpsc };

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
    Connected(u32),
    /// When a connection is closed with a client or server.
    Disconnected(u32),
    DiagnosticsReceived(Diagnostics, Bytes),
    Error(String),
}

/// Handles connections and interactions with Cluster Servers and Clients.
pub struct ClusterServer {
    port: u16,
    // sender: mpsc::Sender<Bytes>,
    event_tx: broadcast::Sender<ClusterEvent>,
    event_rx: broadcast::Receiver<ClusterEvent>,
    connections: DashMap<u32, ClusterClient>,
    // Only used to store cluster servers so clients can switch between them.
    cluster_servers: BTreeSet<ClusterInfo>,
    released_ids: BTreeSet<u32>,
    master_connection: MasterConnection,
}

impl ClusterServer {
    pub async fn new(/* config: Config */) -> io::Result<Self> {
        // Load the configuration from a file or environment variables
        // For now, we'll use a default port
        let port = constants::MASTER_PORT;

        // let (sender, _) = mpsc::channel::<Bytes>(16);
        let (event_tx, event_rx) = broadcast::channel::<ClusterEvent>(16);

        let master_connection = MasterConnection::connect(constants::DEFAULT_IP, port).await?;

        Ok(Self {
            port,
            event_tx,
            event_rx,
            connections: DashMap::new(),
            cluster_servers: BTreeSet::new(),
            released_ids: BTreeSet::new(),
            master_connection,
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
    pub async fn start(&self) -> io::Result<()> {
        let addr = format!("{}:{}", constants::DEFAULT_IP, self.port);

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

        loop {
            let (stream, peer) = match listener.accept().await {
                Ok(pair) => pair,
                Err(e) => {
                    LOGGER.error(&format!("Failed to accept connection: {e}")).await;
                    continue;
                }
            };
            LOGGER.debug(&format!("Accepted connection from {peer}")).await;

            // TODO: This is one the right path but AI did this. Move it to a struct.

            // Create a new ConnectionInfo instance
            let id = 0;
            let connection = match ClusterClient::new(id, stream, self.event_tx.clone()).await {
                Ok(c) => c,
                Err(e) => {
                    LOGGER.error(&format!("Failed to create connection: {e}")).await;
                    continue;
                }
            };

            // Store the connection in the connections map
            self.connections.insert(id, connection);
        }
    }
}
