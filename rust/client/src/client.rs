//! Handles connections to a server and sending messages.
use sustenet_shared::logging::{ LogType, Logger };
use sustenet_shared::lselect;
use sustenet_shared::packets::{ Connection, Diagnostics, Messaging };

use std::io::Error;
use std::sync::LazyLock;

use bytes::Bytes;
use tokio::io::AsyncReadExt;
use tokio::io::{ self, AsyncWriteExt };
use tokio::net::TcpStream;
use tokio::sync::{ broadcast, mpsc };

/// Global logger for the client module.
pub static LOGGER: LazyLock<Logger> = LazyLock::new(|| Logger::new(LogType::Client));

#[derive(Debug, Clone, PartialEq)]
pub struct ClusterInfo {
    pub name: String,
    pub ip: String,
    pub port: u16,
    pub max_connections: u32,
}



/// Events emitted by the client to notify listeners.
///
/// Should be handled with `event_receiver` or `next_event` externally.
#[derive(Debug, Clone)]
pub enum ClientEvent {
    Connected,
    Disconnected,
    CommandSent(u8),
    MessageSent(Bytes),
    CommandReceived(u8),
    MessageReceived(Bytes),
    Error(String),
}

/// Handles connection to a master or cluster server, and provides async channels for interaction.
pub struct Client {
    /// Sends messages to the server.
    sender: mpsc::Sender<Bytes>,
    /// Sends events to listeners.
    event_tx: broadcast::Sender<ClientEvent>,
    /// Receives events about connection state and activity.
    event_rx: broadcast::Receiver<ClientEvent>,
    /// Cluster servers this client knows about.
    pub cluster_servers: Vec<ClusterInfo>,
}

impl Client {
    /// Attempts to connect to a server at the specified address and port and returns a `ClientHandle`.
    pub async fn connect(address: &str, port: u16) -> io::Result<Self> {
        let addr = format!("{}:{}", address, port);
        LOGGER.info(&format!("Connecting to {addr}...")).await;

        // Establish a connection to the server.
        let mut stream = match TcpStream::connect(&addr).await {
            Ok(s) => {
                LOGGER.success(&format!("Connected to {addr}")).await;
                s
            }
            Err(e) => {
                LOGGER.error(&format!("Failed to connect to {addr}")).await;
                return Err(Error::new(e.kind(), format!("Failed to connect to ({addr}): {e}")));
            }
        };

        let (sender, mut receiver) = mpsc::channel::<Bytes>(64);
        let (event_tx, event_rx) = broadcast::channel::<ClientEvent>(16);

        let sender_clone = sender.clone();
        let event_tx_clone = event_tx.clone();

        tokio::spawn(async move {
            let (reader, mut writer) = stream.split();
            let mut reader = io::BufReader::new(reader);

            lselect!(
                // Handle local requests to send a message to the server.
                msg = receiver.recv() => {
                    match msg {
                        Some(msg) => {
                            if msg.is_empty() {
                                LOGGER.warning("Received empty message, shutting down client").await;
                                Self::handle_shutdown(writer, event_tx_clone).await;
                                break;
                            }

                            LOGGER.debug(&format!("Sending message: {:?}", msg)).await;
                            if let Err(e) = writer.write_all(&msg).await {
                                let msg = format!("Failed to send message to server: {e}");
                                LOGGER.error(&msg).await;
                                let _ = event_tx_clone.send(ClientEvent::Error(msg));
                            } else {
                                let _ = event_tx_clone.send(ClientEvent::MessageSent(msg));
                            }
                        },
                        None => {
                            LOGGER.warning("Connection closed").await;
                            Self::handle_shutdown(writer, event_tx_clone).await;
                            break;
                        }
                    }
                },
                command = reader.read_u8() => {
                    match command {
                        Ok(command) => {
                            LOGGER.debug(&format!("Received command: {command}")).await;

                            Self::handle_command(command, &sender_clone, &mut reader, &mut writer, &event_tx_clone).await;

                            // Notify listeners about the received message.
                            let _ = event_tx_clone.send(ClientEvent::CommandReceived(command));
                        },
                        Err(e) => {
                            let msg = format!("Failed to read command from server: {e}");
                            LOGGER.error(&msg).await;
                            let _ = event_tx_clone.send(ClientEvent::Error(msg));
                        }
                    }
                }
            )
        });

        // Notify connected immediately.
        let _ = event_tx.send(ClientEvent::Connected);

        Ok(Client {
            sender,
            event_tx,
            event_rx,
            cluster_servers: Vec::new(),
        })
    }

    async fn handle_shutdown(
        mut writer: tokio::net::tcp::WriteHalf<'_>,
        event_tx_clone: broadcast::Sender<ClientEvent>
    ) {
        if let Err(e) = writer.shutdown().await {
            let msg = format!("Failed to shutdown writer: {e}");
            LOGGER.error(&msg).await;
            let _ = event_tx_clone.send(ClientEvent::Error(msg));
        }
        let _ = event_tx_clone.send(ClientEvent::Disconnected);
    }

    /// Handles commands received from the server.
    /// This function is called in a separate task to handle incoming commands.
    async fn handle_command(
        command: u8,
        _sender: &mpsc::Sender<Bytes>,
        _reader: &mut io::BufReader<tokio::net::tcp::ReadHalf<'_>>,
        _writer: &mut tokio::net::tcp::WriteHalf<'_>,
        event_tx: &broadcast::Sender<ClientEvent>
    ) {
        // Todo: Handle commands.
        // Handle the command received from the server.
        match command {
            x if x == (Connection::Connect as u8) => Self::handle_connect_command().await,
            x if x == (Connection::Disconnect as u8) => Self::handle_disconnect_command().await,
            x if x == (Connection::Authenticate as u8) => Self::handle_authenticate_command().await,

            x if x == (Messaging::SendGlobalMessage as u8) => {
                Self::handle_send_global_message_command().await
            }
            x if x == (Messaging::SendPrivateMessage as u8) => {
                Self::handle_send_private_message_command().await
            }
            x if x == (Messaging::SendPartyMessage as u8) => {
                Self::handle_send_party_message_command().await
            }
            x if x == (Messaging::SendLocalMessage as u8) => {
                Self::handle_send_local_message_command().await
            }

            x if x == (Diagnostics::CheckServerType as u8) => {
                Self::handle_check_server_type_command().await
            }
            x if x == (Diagnostics::CheckServerUptime as u8) => {
                Self::handle_check_server_uptime_command().await
            }
            x if x == (Diagnostics::CheckServerPlayerCount as u8) => {
                Self::handle_check_server_player_count_command().await
            }

            _ => Self::handle_extra_command(command, event_tx).await,
        }
    }

    async fn handle_connect_command() {
        todo!();
    }
    async fn handle_disconnect_command() {
        todo!();
    }
    async fn handle_authenticate_command() {
        todo!();
    }

    async fn handle_send_global_message_command() {
        todo!();
    }
    async fn handle_send_private_message_command() {
        todo!();
    }
    async fn handle_send_party_message_command() {
        todo!();
    }
    async fn handle_send_local_message_command() {
        todo!();
    }

    async fn handle_check_server_type_command() {
        todo!();
    }
    async fn handle_check_server_uptime_command() {
        todo!();
    }
    async fn handle_check_server_player_count_command() {
        todo!();
    }

    async fn handle_extra_command(command: u8, event_tx: &broadcast::Sender<ClientEvent>) {
        let msg = format!("Unknown command received: {command}");
        LOGGER.error(&msg).await;
        let _ = event_tx.send(ClientEvent::Error(msg));
    }

    /// Sends data to the server.
    pub async fn send(&self, msg: Bytes) -> Result<(), mpsc::error::SendError<Bytes>> {
        self.sender.send(msg.clone()).await?;
        let _ = self.event_tx.send(ClientEvent::MessageSent(msg));
        Ok(())
    }

    /// Returns a cloneable event receiver for status updates.
    pub fn event_receiver(&self) -> broadcast::Receiver<ClientEvent> {
        self.event_rx.resubscribe()
    }

    /// Returns the next event from the event receiver.
    pub async fn next_event(&mut self) -> Option<ClientEvent> {
        let event = self.event_rx.recv().await;
        match event {
            Ok(event) => Some(event),
            Err(_) => None,
        }
    }

    // region: Cluster Server Utilities
    pub fn get_cluster_servers(&self) -> &[ClusterInfo] {
    &self.cluster_servers
    }

    pub fn add_cluster_server(&mut self, server: ClusterInfo) {
        self.cluster_servers.push(server);
    }

    pub fn add_cluster_servers(&mut self, servers: Vec<ClusterInfo>) {
        self.cluster_servers.extend(servers);
    }

    pub fn remove_cluster_server(&mut self, server: &ClusterInfo) {
        self.cluster_servers.retain(|s| s != server);
    }

    pub fn clear_cluster_servers(&mut self) {
        self.cluster_servers.clear();
    }
    // endregion: Cluster Server Utilities
}
