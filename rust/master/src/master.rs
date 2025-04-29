//! Handles connections and logic specific to the Master Server.
//! This module is intended for managing cluster/server registration, diagnostics, and authentication.
use crate::master_client::MasterClient;
use sustenet_shared::logging::{ LogType, Logger };
use sustenet_shared::network::ClusterInfo;
use sustenet_shared::packets::{ ClusterSetup, Connection, Diagnostics };
use sustenet_shared::utils::constants;

use std::collections::BTreeSet;
use std::io::{Error, ErrorKind};
use std::sync::LazyLock;

use bytes::Bytes;
use dashmap::DashMap;
use tokio::io;
use tokio::net::{ TcpListener, TcpStream };
use tokio::sync::{ broadcast, mpsc };

/// Global logger for the master module.
pub static LOGGER: LazyLock<Logger> = LazyLock::new(|| Logger::new(LogType::Master));

/// Events emitted by the master server to notify listeners.
#[derive(Debug, Clone)]
pub enum MasterEvent {
    /// When a connection is established with a client or server.
    Connected(u32),
    /// When a connection is closed with a client or server.
    Disconnected(u32),
    ClusterRegistered(u32, String),
    ClusterRegistrationFailed(u32),
    DiagnosticsReceived(Diagnostics, Bytes),
    Error(String),
}

/// Handles connections and interactions with Cluster Servers and Clients.
pub struct MasterServer {
    port: u16,
    // sender: mpsc::Sender<Bytes>,
    event_tx: broadcast::Sender<MasterEvent>,
    event_rx: broadcast::Receiver<MasterEvent>,
    connections: DashMap<u32, MasterClient>,
    cluster_servers: BTreeSet<ClusterInfo>,
    released_ids: BTreeSet<u32>,
}

impl MasterServer {
    pub async fn new(/* config: Config */) -> io::Result<Self> {
        // Load the configuration from a file or environment variables
        // For now, we'll use a default port
        let port = constants::MASTER_PORT;

        let (event_tx, event_rx) = broadcast::channel::<MasterEvent>(16);

        Ok(Self {
            port,
            event_tx,
            event_rx,
            connections: DashMap::new(),
            cluster_servers: BTreeSet::new(),
            released_ids: BTreeSet::new(),
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
            let connection = match MasterClient::new(id, stream, self.event_tx.clone()).await {
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

	/// Sends a message to a specific client.
	pub async fn send(client: &MasterClient, bytes: Bytes) -> io::Result<()> {
		if let Err(e) = client.send(bytes).await {
			LOGGER.error(&format!("Failed to send message to client: {e}")).await;
			return Err(Error::new(ErrorKind::Other, format!("Failed to send message to client: {e}")));
		}
		Ok(())
	}

	/// Sends a message to a specific client ID.
	pub async fn send_to(&self, id: &u32, bytes: Bytes) -> io::Result<()> {
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
		for client in self.connections.iter() {
			Self::send(&client, bytes.clone()).await?;
		}
		Ok(())
	}

	/// Sends a message to all cluster servers.
	pub async fn send_to_clusters(&self, bytes: Bytes) -> io::Result<()> {
		for cluster in self.cluster_servers.iter() {
			if let Err(e) = self.send_to(&cluster.id, bytes.clone()).await {
				LOGGER.error(&format!("Failed to send message to cluster {}: {e}", cluster.name)).await;
			}
		}
		Ok(())
	}
}
