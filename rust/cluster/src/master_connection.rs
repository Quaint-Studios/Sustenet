use sustenet_shared::lselect;
use sustenet_shared::packets::{ ClusterSetup, Connection, Diagnostics, Messaging };
use tokio::sync::mpsc::error::SendError;

use std::io::Error;
use std::net::IpAddr;

use bytes::Bytes;
use tokio::io::AsyncReadExt;
use tokio::io::{ self, AsyncWriteExt };
use tokio::net::TcpStream;
use tokio::sync::{ broadcast, mpsc };

use crate::cluster::{ ClusterEvent, LOGGER };

pub struct MasterConnection {
    ip: IpAddr,
    port: u16,
    sender: mpsc::Sender<Bytes>,
}

impl MasterConnection {
    pub async fn connect(address: &str, port: u16) -> io::Result<Self> {
        let addr = format!("{}:{}", address, port);
        LOGGER.info(&format!("Connecting to the master server at {addr}...")).await;
        
        // Establish a connection to the master server.
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

        let ip = stream.peer_addr()?.ip();

        let (sender, mut receiver) = mpsc::channel::<Bytes>(16);
        let (event_tx, event_rx) = broadcast::channel::<ClusterEvent>(16);

        let sender_clone = sender.clone();
        let event_tx_clone = event_tx.clone();

        tokio::spawn(async move {
            let (reader, mut writer) = stream.split();
            let mut reader = io::BufReader::new(reader);

            lselect!(
                // Handle local requests to send a message to the master server.
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
                                let msg = format!("Failed to send message to master server: {e}");
                                LOGGER.error(&msg).await;
                                let _ = event_tx_clone.send(ClusterEvent::Error(msg));
                            } else {
                                let _ = event_tx_clone.send(ClusterEvent::MasterMessageReceived(msg));
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
                        },
                        Err(e) => {
                            LOGGER.error(&format!("Failed to read command: {e}")).await;
                        }
                    }
                }
            );
        });


        Ok(Self { ip, port, sender })
    }

    async fn handle_shutdown(
        mut writer: tokio::net::tcp::WriteHalf<'_>,
        event_tx_clone: broadcast::Sender<ClusterEvent>
    ) {
        if let Err(e) = writer.shutdown().await {
            let msg = format!("Failed to shutdown writer: {e}");
            LOGGER.error(&msg).await;
            let _ = event_tx_clone.send(ClusterEvent::Error(msg));
        }
        let _ = event_tx_clone.send(ClusterEvent::MasterDisconnected);
    }

    /// Handles commands received from the server.
    /// This function is called in a separate task to handle incoming commands.
    async fn handle_command(
        command: u8,
        sender: &mpsc::Sender<Bytes>,
        reader: &mut io::BufReader<tokio::net::tcp::ReadHalf<'_>>,
        writer: &mut tokio::net::tcp::WriteHalf<'_>,
        event_tx: &broadcast::Sender<ClusterEvent>
    ) {
        // Handle the command received from the server.
        match command {
            x if x == (Connection::Connect as u8) => {
                LOGGER.info("Handling Connection Connect").await;
            }
            x if x == (Connection::Disconnect as u8) => {
                LOGGER.info("Handling Connection Disconnect").await;
            }

            x if x == (ClusterSetup::Init as u8) => {
                LOGGER.info("Handling Cluster Setup Init").await;
            }
            x if x == (ClusterSetup::AnswerSecret as u8) => {
                LOGGER.info("Handling Cluster Setup Answer Secret").await;
            }

            x if x == (Diagnostics::CheckServerType as u8) => {
                LOGGER.info("Handling Diagnostics Check Server Type").await;
            }
            x if x == (Diagnostics::CheckServerUptime as u8) => {
                LOGGER.info("Handling Diagnostics Check Server Uptime").await;
            }
            x if x == (Diagnostics::CheckServerPlayerCount as u8) => {
                LOGGER.info("Handling Diagnostics Check Server Player Count").await;
            }

            _ => {
                let msg = format!("Unknown command received: {command}");
                LOGGER.error(&msg).await;
                let _ = event_tx.send(ClusterEvent::Error(msg));
            }
        }
    }

    pub fn get_ip(&self) -> IpAddr {
        self.ip
    }

    pub fn get_port(&self) -> u16 {
        self.port
    }

    pub async fn send(&self, data: Bytes) -> Result<(), SendError<Bytes>> {
        self.sender.send(data).await
    }
}
