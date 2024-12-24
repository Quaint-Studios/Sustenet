use std::{ net::Ipv4Addr, str::FromStr };

use tokio::io::{ AsyncBufReadExt, AsyncWriteExt, BufReader, BufWriter };
use tokio::net::{ TcpStream, UdpSocket };
use tokio::select;
use tokio::sync::mpsc::{ self, Receiver, Sender };

use crate::events::{ ClientPackets, Event };
use crate::transport::Logging;
use crate::utils::constants;

pub enum ConnectionType {
    MasterServer,
    ClusterServer,
    None,
}
pub struct Connection {
    pub ip: Ipv4Addr,
    pub port: u16,
}

pub struct Client {
    pub id: Option<u32>,
    pub name: Option<String>,

    stream: Option<TcpStream>,
    socket: Option<UdpSocket>,

    pub active_connection: ConnectionType,
    pub master_connection: Connection,
    pub cluster_connection: Connection,

    event_receiver: Receiver<Event>,
    event_sender: Sender<Event>,
}

impl Client {
    // TODO: ip string and port
    pub fn new(ip: Option<Ipv4Addr>, port: Option<u16>) -> Client {
        let (event_sender, event_receiver) = mpsc::channel(100); // TODO: Could be a different channel type.

        return Client {
            id: None,
            name: None,

            stream: None,
            socket: None,

            active_connection: ConnectionType::None,
            master_connection: Connection {
                ip: ip.unwrap_or(Ipv4Addr::from_str(constants::DEFAULT_IP).unwrap()),
                port: port.unwrap_or(constants::MASTER_PORT),
            },
            // TODO: Consider merging master and cluster connection into one to save on memory.
            cluster_connection: Connection {
                // Placeholder until overridden and used.
                ip: Ipv4Addr::LOCALHOST,
                port: constants::CLUSTER_PORT,
            },

            event_receiver,
            event_sender,
        };
    }

    pub async fn start(&mut self) {
        let mut shutdown_rx = crate::app::shutdown_channel().unwrap();
        let mut is_running = true;

        select! {
            _ = shutdown_rx.recv() => {
                is_running = false;
                Self::warning("Shutting down...");
            }
            _ = self.connect(&mut is_running) => {}
        }

        if !is_running {
            self.cleanup().await;
            client_success!("Client has been shut down.");
        }
    }

    pub async fn connect(&mut self, is_running: &mut bool) {
        let mut stream = TcpStream::connect(
            format!("{}:{}", self.master_connection.ip, self.master_connection.port)
        ).await.unwrap_or_else(|_| {
            panic!("Failed to connect to Master Server.");
        });

        let (tx, mut rx) = mpsc::channel::<Vec<u8>>(10);

        let tcp_handler = tokio::spawn(async move {
            let (reader, mut writer) = stream.split();

            let mut reader = BufReader::new(reader);
            let mut line = String::new();

            loop {
                tokio::select! {
                    result = reader.read_line(&mut line) => {
                        // Break if the line is empty.
                        if result.unwrap() == 0 {
                            break;
                        }

                        println!("C Received: {:?}", line);

                        // Send the line to the channel.
                        // tx_clone.send(line.clone()).await.unwrap();
                        // line.clear();
                    }
                    result = rx.recv() => {
                        // Write the message to the writer.
                        match result {
                            Some(mut msg) => {
                                println!("C Writing message: {:?}", msg);

                                writer.write_all(&msg.as_slice()).await.expect("Failed to write to stream.");
                                writer.flush().await.unwrap();
                            }
                            None => {
                                writer.shutdown().await.unwrap();
                                !("Shutting down.");
                            }
                        }
                    }
                }
            }
        });

        println!("Connected to Master Server.");

        // UDP connection
        let socket = UdpSocket::bind("0.0.0.0:0").await.unwrap();
        socket
            .connect(format!("{}:{}", self.master_connection.ip, self.master_connection.port)).await
            .unwrap();

        self.socket = Some(socket);

        println!("Connected to Master Server via UDP.");

        // Loop and every 2-5 seconds, write a message to the server.
        while *is_running {
            tokio::time::sleep(tokio::time::Duration::from_secs(2)).await;

            let code = ClientPackets::Message as u8;
            let message = "Hello, Master Server!";
            let mut bytes = Vec::from([code, message.len() as u8]);
            bytes.extend_from_slice(message.as_bytes());
            tx.clone().send(bytes).await.unwrap();
            println!("Sent message to Master Server: {}", message);
        }

        tcp_handler.abort();
    }

    /// Cleanup the Client before shutting down.
    async fn cleanup(&self) {
        // TODO: Cleanup the Client.
        client_info!("Cleaning up the Client...");
    }

    /// After a client logs in successfully and gets their username and id back.
    pub async fn on_initialized() {
        println!("Client initialized.");
    }

    pub async fn on_cluster_server_list() {
        println!("Cluster Server List:");
    }
}

impl Logging for Client {
    fn debug(message: &str) {
        if !constants::DEBUGGING {
            return;
        }

        crate::client_debug!("{}", message);
    }
    
    fn info(message: &str) {
        if !constants::DEBUGGING {
            return;
        }

        crate::client_info!("{}", message);
    }

    fn warning(message: &str) {
        if !constants::DEBUGGING {
            return;
        }

        crate::client_warning!("{}", message);
    }

    fn error(message: &str) {
        if !constants::DEBUGGING {
            return;
        }

        crate::client_error!("{}", message);
    }

    fn success(message: &str) {
        if !constants::DEBUGGING {
            return;
        }

        crate::client_success!("{}", message);
    }
}
