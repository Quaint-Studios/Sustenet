use sustenet_shared::{ lselect, packets::{ ClusterSetup, Connection, Diagnostics } };

use std::io::Error;

use bytes::Bytes;
use tokio::{ io::{ self, AsyncReadExt, AsyncWriteExt }, net::TcpStream, sync::{ broadcast, mpsc::{self, error::SendError} } };

use crate::master::{ LOGGER, MasterEvent };

/// Handles connections that clients and cluster servers establish with the
/// master server.
pub struct MasterClient {
    sender: mpsc::Sender<Bytes>,
}

impl MasterClient {
    pub async fn new(
        id: u32,
        stream: TcpStream,
        event_tx: broadcast::Sender<MasterEvent>
    ) -> io::Result<Self> {
        let (sender, receiver) = mpsc::channel::<Bytes>(16);
        let connection = Self { sender };

        if let Err(e) = Self::receive(id, stream, connection.sender.clone(), receiver, event_tx) {
            LOGGER.error(&format!("Failed to start connection #{id}")).await;
            return Err(Error::new(e.kind(), format!("Failed to start connection #{id}: {e}")));
        }

        Ok(connection)
    }

    /// Sends a message to the sender to close the connection.
    ///
    /// This should be called before getting rid of this ServerClient.
    pub async fn close(&self) {
        self.sender.send(Bytes::new()).await.unwrap();
    }

    /// Receives messages from clients and handles them.
    ///
    /// It also enables the MasterServer to send messages through this
    /// struct's sender.
    pub fn receive(
        id: u32,
        mut stream: TcpStream,
        sender: mpsc::Sender<Bytes>,
        mut receiver: mpsc::Receiver<Bytes>,
        event_tx: broadcast::Sender<MasterEvent>
    ) -> io::Result<()> {
        tokio::spawn(async move {
            let (reader, mut writer) = stream.split();
            let mut reader = io::BufReader::new(reader);

            lselect!(
                // Handle local requests to send a message to the the other side of the connection.
                msg = receiver.recv() => {
                    match msg {
                        Some(msg) => {
                            if msg.is_empty() {
                                LOGGER.warning("Received empty message, shutting down connection").await;
                                Self::handle_shutdown(writer, event_tx, id).await;
                                break;
                            }

                            LOGGER.debug(&format!("Sending message: {:?}", msg)).await;
                            if let Err(e) = writer.write_all(&msg).await {
                                let msg = format!("Failed to send message to server: {e}");
                                LOGGER.error(&msg).await;
                                let _ = event_tx.send(MasterEvent::Error(msg));
                            } else {
                                // TODO: Still need to decide if we should notify about messages sent on a server.
                                // let _ = event_tx.send(MasterEvent::MessageSent(msg));
                            }
                        },
                        None => {
                            LOGGER.warning("Connection closed").await;
                            Self::handle_shutdown(writer, event_tx, id).await;
                            break;
                        }
                    }
                },
                command = reader.read_u8() => {
                    match command {
                        Ok(command) => {
                            LOGGER.debug(&format!("Received command: {command}")).await;

                            Self::handle_command(command, &sender, &mut reader, &mut writer, &event_tx).await;

                            // Notify listeners about the received message.
                            // TODO: Should we? I'm leaning more towards not notifying about commands.
                            // It could ruin performance.
                            // let _ = event_tx_clone.send(MasterEvent::CommandReceived(command));
                        },
                        Err(e) => {
                            let msg = format!("Failed to read command for connection #{}: {e}", id);
                            LOGGER.error(&msg).await;
                            let _ = event_tx.send(MasterEvent::Error(msg));
                        }
                    }
                }
            );
        });

        Ok(())
    }

    /// An external method to allow the master server to send messages to the client.
    pub async fn send(&self, bytes: Bytes) -> Result<(), SendError<Bytes>> {
        if let Err(e) = self.sender.send(bytes).await {
            LOGGER.error(&format!("Failed to send message to client: {e}")).await;
            return Err(e);
        }
        Ok(())
    }

    async fn handle_shutdown(
        mut writer: tokio::net::tcp::WriteHalf<'_>,
        event_tx: broadcast::Sender<MasterEvent>,
        id: u32
    ) {
        if let Err(e) = writer.shutdown().await {
            let msg = format!("Failed to shutdown writer: {e}");
            LOGGER.error(&msg).await;
            let _ = event_tx.send(MasterEvent::Error(msg));
        }
        let _ = event_tx.send(MasterEvent::Disconnected(id));
    }

    async fn handle_command(
        command: u8,
        sender: &mpsc::Sender<Bytes>,
        reader: &mut io::BufReader<tokio::net::tcp::ReadHalf<'_>>,
        writer: &mut tokio::net::tcp::WriteHalf<'_>,
        event_tx: &broadcast::Sender<MasterEvent>
    ) {
        // Handle the command received from the server.
        match command {
            x if x == (Connection::Connect as u8) => {
                LOGGER.info("Handling Connection Connect").await;
            }
            x if x == (Connection::Disconnect as u8) => {
                LOGGER.info("Handling Connection Disconnect").await;
            }

            x if x == (Diagnostics::CheckServerType as u8) => {
                LOGGER.info("Handling Diagnostics Check Server Type").await;
            }
            x if x == (Diagnostics::CheckServerVersion as u8) => {
                LOGGER.info("Handling Diagnostics Check Server Version").await;
            }
            x if x == (Diagnostics::CheckServerUptime as u8) => {
                LOGGER.info("Handling Diagnostics Check Server Uptime").await;
            }
            x if x == (Diagnostics::CheckServerPlayerCount as u8) => {
                LOGGER.info("Handling Diagnostics Check Server Player Count").await;
            }

            x if x == (ClusterSetup::Init as u8) => {
                LOGGER.info("Handling Cluster Setup Init").await;
            }
            x if x == (ClusterSetup::AnswerSecret as u8) => {
                LOGGER.info("Handling Cluster Setup Answer Secret").await;
            }

            _ => {
                let msg = format!("Unknown command received: {command}");
                LOGGER.error(&msg).await;
                let _ = event_tx.send(MasterEvent::Error(msg));
            }
        }
    }
}
