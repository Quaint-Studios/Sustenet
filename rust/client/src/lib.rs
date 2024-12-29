use std::sync::Arc;
use std::{ net::Ipv4Addr, str::FromStr };

use tokio::io::{ AsyncReadExt, AsyncWriteExt, BufReader };
use tokio::net::TcpStream;
use tokio::sync::mpsc::Sender;
use tokio::sync::{ mpsc, Mutex, RwLock };

use shared::packets::cluster::ToClient;
use shared::packets::master::{ FromUnknown, ToUnknown };
use shared::utils::constants;
use shared::utils::constants::{ DEFAULT_IP, MASTER_PORT };
use shared::{ log_message, lread_string, lselect };

lazy_static::lazy_static! {
    pub static ref CLUSTER_SERVERS: Arc<RwLock<Vec<ClusterInfo>>> = Arc::new(
        RwLock::new(Vec::new())
    );
    pub static ref SENDER: Arc<Mutex<Option<Sender<Box<[u8]>>>>> = Arc::new(Mutex::new(None));
    pub static ref CONNECTION: Arc<RwLock<Option<Connection>>> = Arc::new(
        RwLock::new(
            Some(Connection {
                ip: get_ip(DEFAULT_IP),
                port: MASTER_PORT,
                connection_type: ConnectionType::MasterServer,
            })
        )
    );
}

#[derive(Debug, Clone)]
pub struct ClusterInfo {
    pub name: String,
    pub ip: String,
    pub port: u16,
    pub max_connections: u32,
}

#[derive(Clone, Copy)]
pub struct Connection {
    pub ip: Ipv4Addr,
    pub port: u16,
    connection_type: ConnectionType,
}

impl From<ClusterInfo> for Connection {
    fn from(info: ClusterInfo) -> Self {
        Connection {
            ip: Ipv4Addr::from_str(info.ip.as_str()).expect("Failed to parse the IP."),
            port: info.port,
            connection_type: ConnectionType::ClusterServer,
        }
    }
}

#[derive(Clone, Copy, Eq, PartialEq)]
pub enum ConnectionType {
    MasterServer,
    ClusterServer,
    None,
}

impl std::fmt::Display for ConnectionType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            ConnectionType::MasterServer => write!(f, "Master Server"),
            ConnectionType::ClusterServer => write!(f, "Cluster Server"),
            ConnectionType::None => write!(f, "Unknown"),
        }
    }
}

pub fn get_ip(ip: &str) -> Ipv4Addr {
    Ipv4Addr::from_str(ip).unwrap_or(Ipv4Addr::from_str(DEFAULT_IP).unwrap_or(Ipv4Addr::LOCALHOST))
}

pub async fn cleanup() {}

pub async fn start() {
    // Get the connection information.
    let connection = *CONNECTION.read().await;
    if connection.is_none() {
        return;
    }
    let connection = connection.unwrap();

    let ip = connection.ip;
    let port = connection.port;
    let connection_type = connection.connection_type;
    {
        *CONNECTION.write().await = None;
    }

    let (tx, mut rx) = mpsc::channel::<Box<[u8]>>(10);
    {
        SENDER.lock().await.replace(tx);
    }

    let handler = tokio::spawn(async move {
        warning(format!("Connecting to the {connection_type}...").as_str());
        let mut stream = TcpStream::connect(format!("{}:{}", ip, port)).await.expect(
            format!("Failed to connect to the {connection_type} at {ip}:{port}.").as_str()
        );
        success(format!("Connected to the {connection_type} at {ip}:{port}.").as_str());

        let (reader, mut writer) = stream.split();
        let mut reader = BufReader::new(reader);

        lselect! {
            command = reader.read_u8() => {
                if command.is_err() {
                    continue;
                }

                info(format!("Received data: {:?}", command).as_str());

                match connection_type {
                    ConnectionType::MasterServer => match command.unwrap() {
                        x if x == ToUnknown::SendClusters as u8 => {
                            let amount = match reader.read_u8().await {
                                Ok(amount) => amount,
                                Err(_) => {
                                    error("Failed to read the amount of clusters.");
                                    continue;
                                }
                            };

                            let mut cluster_servers_tmp = Vec::new();
                            for _ in 0..amount {
                                let name = lread_string!(reader, error, "cluster name");
                                let ip = lread_string!(reader, error, "cluster IP");
                                let port = match reader.read_u16().await {
                                    Ok(port) => port,
                                    Err(_) => {
                                        error("Failed to read the cluster port.");
                                        continue;
                                    }
                                };
                                let max_connections = match reader.read_u32().await {
                                    Ok(max_connections) => max_connections,
                                    Err(_) => {
                                        error("Failed to read the cluster max connections.");
                                        continue;
                                    }
                                };

                                cluster_servers_tmp.push(ClusterInfo {
                                    name,
                                    ip,
                                    port,
                                    max_connections,
                                });
                            }

                            {
                                {
                                    let mut cluster_servers = CLUSTER_SERVERS.write().await;
                                    *cluster_servers = cluster_servers_tmp;

                                    success(format!("Received {amount} Cluster servers from the {connection_type}.").as_str());
                                    println!("{:?}", *cluster_servers);
                                }
                                join_cluster(0).await;
                            }
                        },
                        _ => (),
                    }
                    ConnectionType::ClusterServer => match command.unwrap() {
                        x if x == ToClient::SendClusters as u8 => {
                            let amount = match reader.read_u8().await {
                                Ok(amount) => amount,
                                Err(_) => {
                                    error("Failed to read the amount of clusters.");
                                    continue;
                                }
                            };

                            let mut cluster_servers_tmp = Vec::new();
                            for _ in 0..amount {
                                let name = lread_string!(reader, error, "cluster name");
                                let ip = lread_string!(reader, error, "cluster IP");
                                let port = match reader.read_u16().await {
                                    Ok(port) => port,
                                    Err(_) => {
                                        error("Failed to read the cluster port.");
                                        continue;
                                    }
                                };
                                let max_connections = match reader.read_u32().await {
                                    Ok(max_connections) => max_connections,
                                    Err(_) => {
                                        error("Failed to read the cluster max connections.");
                                        continue;
                                    }
                                };

                                cluster_servers_tmp.push(ClusterInfo {
                                    name,
                                    ip,
                                    port,
                                    max_connections,
                                });
                            }

                            {
                                {
                                    let mut cluster_servers = CLUSTER_SERVERS.write().await;
                                    *cluster_servers = cluster_servers_tmp;

                                    success(format!("Received {amount} Cluster servers from the {connection_type}.").as_str());
                                    println!("{:?}", *cluster_servers);
                                }
                                join_cluster(0).await;
                            }
                        },
                        x if x == ToClient::DisconnectCluster as u8 => todo!(),
                        x if x == ToClient::LeaveCluster as u8 => todo!(),

                        x if x == ToClient::VersionOfKey as u8 => todo!(),
                        x if x == ToClient::SendPubKey as u8 => todo!(),
                        x if x == ToClient::Authenticate as u8 => todo!(),

                        x if x == ToClient::Move as u8 => todo!(),
                        _ => (),
                    }
                    _ => (),
                }
            }
            result = rx.recv() => {
                if let Some(data) = result {
                    if data.is_empty() {
                        writer.shutdown().await.expect("Failed to shutdown the writer.");
                        info("Closing connection...");
                        break;
                    }

                    writer.write_all(&data).await.expect("Failed to write to the Server.");
                    writer.flush().await.expect("Failed to flush the writer.");
                    info(format!("Sent {data:?} as data to the {connection_type}.").as_str());
                } else {
                    writer.shutdown().await.expect("Failed to shutdown the writer.");
                    info("Shutting down connection...");
                    break;
                }
            }
        }
    });

    if connection_type == ConnectionType::MasterServer {
        send_data(Box::new([FromUnknown::RequestClusters as u8])).await;
    }

    let _ = handler.await;
}

pub async fn send_data(data: Box<[u8]>) {
    let tx = SENDER.lock().await;
    match tx.as_ref() {
        Some(tx) => {
            tx.send(data).await.expect("Failed to send data to the Server.");
        }
        None => {
            error("Failed to send data to the Server. The Sender is not set.");
        }
    }
}

pub async fn join_cluster(id: usize) {
    let cluster_servers = CLUSTER_SERVERS.read().await;
    if cluster_servers.is_empty() {
        error("Failed to join a cluster. No cluster servers are available.");
        return;
    }
    let cluster = (
        match cluster_servers.get(id) {
            Some(cluster) => cluster,
            None => {
                error("Failed to join a cluster. The cluster ID is invalid.");
                return;
            }
        }
    ).clone();

    success(format!("Client is joining cluster {}", cluster.name).as_str());

    let connection = match std::panic::catch_unwind(|| Connection::from(cluster)) {
        Ok(connection) => connection,
        Err(_) => {
            error("Failed to create a connection with the Cluster Server.");
            return;
        }
    };
    {
        // Overwrite the current connection with the cluster connection.
        *CONNECTION.write().await = Some(connection);
        let tx = SENDER.lock().await;
        let tx = match tx.as_ref() {
            Some(tx) => tx,
            None => {
                error("Failed to send data to the Server. The Sender is not set.");
                return;
            }
        };
        tx.send(Box::new([])).await.expect("Failed to shutdown.");
    }
}

// region: Logging
// fn debug(message: &str) {
//     if !constants::DEBUGGING {
//         return;
//     }
//     log_message!(LogLevel::Debug, LogType::Client, "{}", message);
// }

pub fn info(message: &str) {
    if !constants::DEBUGGING {
        return;
    }
    log_message!(LogLevel::Info, LogType::Client, "{}", message);
}

pub fn warning(message: &str) {
    if !constants::DEBUGGING {
        return;
    }
    log_message!(LogLevel::Warning, LogType::Client, "{}", message);
}

pub fn error(message: &str) {
    if !constants::DEBUGGING {
        return;
    }
    log_message!(LogLevel::Error, LogType::Client, "{}", message);
}

pub fn success(message: &str) {
    if !constants::DEBUGGING {
        return;
    }
    log_message!(LogLevel::Success, LogType::Client, "{}", message);
}
// endregion
