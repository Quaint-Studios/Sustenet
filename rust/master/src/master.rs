//! Handles connections and logic specific to the Master Server.
//! This module is intended for managing cluster/server registration, diagnostics, and authentication.

use bytes::Bytes;
use dashmap::DashMap;
use std::io::{ Error, ErrorKind };
use std::sync::LazyLock;
use sustenet_shared::logging::{ LogType, Logger };
use sustenet_shared::packets::{ ClusterSetup, Connection, Diagnostics };
use sustenet_shared::utils::constants;
use tokio::io::{ self, AsyncReadExt, AsyncWriteExt };
use tokio::net::{ TcpListener, TcpStream };
use tokio::sync::{ broadcast, mpsc };

use crate::server_client::ServerClient;

/// Global logger for the master module.
pub static LOGGER: LazyLock<Logger> = LazyLock::new(|| Logger::new(LogType::Master));

/// Events emitted by the master server to notify listeners.
#[derive(Debug, Clone)]
pub enum MasterEvent {
    Connected,
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
	connections: DashMap<u32, ServerClient>,

}

impl MasterServer {
    pub async fn new(/* config: Config */) -> io::Result<Self> {
		// Load the configuration from a file or environment variables
		// For now, we'll use a default port
		let port = constants::MASTER_PORT;

		// let (sender, _) = mpsc::channel::<Bytes>(16);
		let (event_tx, event_rx) = broadcast::channel::<MasterEvent>(16);

		// Return the new MasterServer instance
		Ok(Self {
			port,
			// sender,
			event_tx,
			event_rx,
			connections: DashMap::new(),
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
                return Err(
                    Error::new(e.kind(), format!("Failed to bind to ({addr}): {e}"))
                );
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
			let connection = match ServerClient::new(id, stream, self.event_tx.clone()).await {
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
