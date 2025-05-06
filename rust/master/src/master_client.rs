use sustenet_shared::lselect;
use sustenet_shared::packets::{ ClusterSetup, Connection, Diagnostics };

use std::io::{ Error, ErrorKind };

use bytes::Bytes;
use tokio::io;
use tokio::io::{ AsyncReadExt, AsyncWriteExt };
use tokio::net::TcpStream;
use tokio::sync::broadcast;
use tokio::sync::mpsc;
use tokio::sync::mpsc::error::SendError;

use crate::master::{ LOGGER, MasterEvent };

/// Handles connections that clients and cluster servers establish with the
/// master server.
pub struct MasterClient {
    sender: mpsc::Sender<Bytes>,
}

impl MasterClient {
    pub async fn new(
        id: u64,
        stream: TcpStream,
        event_tx: broadcast::Sender<MasterEvent>
    ) -> io::Result<Self> {
        let (sender, receiver) = mpsc::channel::<Bytes>(16);
        let connection = Self { sender };

        if let Err(e) = Self::receive(id, stream, connection.sender.clone(), receiver, event_tx) {
            LOGGER.error(&format!("Failed to start connection #{id}"));
            return Err(Error::new(e.kind(), format!("Failed to start connection #{id}: {e}")));
        }

        Ok(connection)
    }

    /// Sends a message to the sender to close the connection.
    ///
    /// This should be called before getting rid of this ServerClient.
    ///
    /// It doesn't need to be called if the Disconnected event is triggered
    /// since that event only triggers when the connection is closed by the
    /// client so it's already handled.
    pub async fn close(&self) -> Result<(), SendError<Bytes>> {
        self.sender.send(Bytes::new()).await
    }

    /// Receives messages from clients and handles them.
    ///
    /// It also enables the MasterServer to send messages through this
    /// struct's sender.
    pub fn receive(
        id: u64,
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
                                LOGGER.warning("Received empty message, shutting down connection");
                                Self::handle_shutdown(writer, event_tx, id).await;
                                break;
                            }

                            LOGGER.debug(&format!("Sending message: {:?}", msg));
                            if let Err(e) = writer.write_all(&msg).await {
                                let msg = format!("Failed to send message to server: {e}");
                                LOGGER.error(&msg);
                                let _ = event_tx.send(MasterEvent::Error(msg));
                            } else {
                                // TODO: Still need to decide if we should notify about messages sent on a server.
                                // let _ = event_tx.send(MasterEvent::MessageSent(msg));
                            }
                        },
                        None => {
                            LOGGER.warning("Connection closed");
                            Self::handle_shutdown(writer, event_tx, id).await;
                            break;
                        }
                    }
                },
                command = reader.read_u8() => {
                    match command {
                        Ok(command) => {
                            LOGGER.debug(&format!("Received command: {command}"));

                            Self::handle_command(command, &sender, &mut reader, &mut writer, &event_tx);

                            // Notify listeners about the received message.
                            // TODO: Should we? I'm leaning more towards not notifying about commands.
                            // It could ruin performance.
                            // let _ = event_tx_clone.send(MasterEvent::CommandReceived(command));
                        },
                        Err(e) => {
                            match e.kind() {
                                ErrorKind::UnexpectedEof => {
                                    LOGGER.warning(&format!("Connection #{id} closed by peer (EOF)"));
                                    Self::handle_shutdown(writer, event_tx, id).await;
                                    break;
                                }
                                ErrorKind::ConnectionReset => {
                                    LOGGER.info(&format!("Connection #{id} reset by peer"));
                                    Self::handle_shutdown(writer, event_tx, id).await;
                                    break;
                                }
                                ErrorKind::ConnectionAborted => {
                                    LOGGER.info(&format!("Connection #{id} aborted"));
                                    Self::handle_shutdown(writer, event_tx, id).await;
                                    break;
                                }
                                ErrorKind::TimedOut => {
                                    LOGGER.warning(&format!("Connection #{id} timed out"));
                                    Self::handle_shutdown(writer, event_tx, id).await;
                                    break;
                                }
                                ErrorKind::BrokenPipe => {
                                    LOGGER.info(&format!("Connection #{id} broken pipe"));
                                    Self::handle_shutdown(writer, event_tx, id).await;
                                    break;
                                }
                                ErrorKind::NotConnected => {
                                    LOGGER.info(&format!("Connection #{id} not connected"));
                                    Self::handle_shutdown(writer, event_tx, id).await;
                                    break;
                                }
                                _ => {
                                    let msg = format!("Failed to read command for connection #{}: {e}", id);
                                    LOGGER.error(&msg);
                                    let _ = event_tx.send(MasterEvent::Error(msg));
                                    Self::handle_shutdown(writer, event_tx, id).await;
                                    break;
                                }
                            }

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
            LOGGER.error(&format!("Failed to send message to client: {e}"));
            return Err(e);
        }
        Ok(())
    }

    async fn handle_shutdown(
        mut writer: tokio::net::tcp::WriteHalf<'_>,
        event_tx: broadcast::Sender<MasterEvent>,
        id: u64
    ) {
        if let Err(e) = writer.shutdown().await {
            let msg = format!("Failed to shutdown writer: {e}");
            LOGGER.error(&msg);
            let _ = event_tx.send(MasterEvent::Error(msg));
        }
        let _ = event_tx.send(MasterEvent::Disconnected(id));
    }

    fn handle_command(
        command: u8,
        _sender: &mpsc::Sender<Bytes>,
        _reader: &mut io::BufReader<tokio::net::tcp::ReadHalf<'_>>,
        _writer: &mut tokio::net::tcp::WriteHalf<'_>,
        event_tx: &broadcast::Sender<MasterEvent>
    ) {
        // TODO: Handle commands.
        // Handle the command received from the client.
        match command {
            x if x == (Connection::Connect as u8) => {
                LOGGER.info("Handling Connection Connect");
            }
            x if x == (Connection::Disconnect as u8) => {
                LOGGER.info("Handling Connection Disconnect");
            }

            x if x == (Diagnostics::CheckServerType as u8) => {
                LOGGER.info("Handling Diagnostics Check Server Type");
            }
            x if x == (Diagnostics::CheckServerUptime as u8) => {
                LOGGER.info("Handling Diagnostics Check Server Uptime");
            }
            x if x == (Diagnostics::CheckServerPlayerCount as u8) => {
                LOGGER.info("Handling Diagnostics Check Server Player Count");
            }

            x if x == (ClusterSetup::Init as u8) => {
                LOGGER.info("Handling Cluster Setup Init");
            }
            x if x == (ClusterSetup::AnswerSecret as u8) => {
                LOGGER.info("Handling Cluster Setup Answer Secret");
            }

            _ => {
                let msg = format!("Unknown command received: {command}");
                LOGGER.error(&msg);
                let _ = event_tx.send(MasterEvent::Error(msg));
            }
        }
    }
}
