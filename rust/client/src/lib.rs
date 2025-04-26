use sustenet_shared as shared;

use std::net::{ IpAddr, Ipv4Addr };
use std::str::FromStr;
use std::sync::{ Arc, LazyLock };

use tokio::io::{ AsyncReadExt, AsyncWriteExt, BufReader };
use tokio::net::TcpStream;
use tokio::sync::mpsc::Sender;
use tokio::sync::{ RwLock, mpsc };

use sustenet_shared::ClientPlugin;
use shared::logging::{ LogType, Logger };
use shared::packets::cluster::ToClient;
use shared::packets::master::ToUnknown;
use shared::utils::constants::{ DEFAULT_IP, MASTER_PORT };
use shared::{ lread_string, lselect };

lazy_static::lazy_static! {
    pub static ref CLUSTER_SERVERS: Arc<RwLock<Vec<ClusterInfo>>> = Arc::new(
        RwLock::new(Vec::new())
    );
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
pub static LOGGER: LazyLock<Logger> = LazyLock::new(|| Logger::new(LogType::Cluster));

#[derive(Debug, Clone)]
pub struct ClusterInfo {
    pub name: String,
    pub ip: String,
    pub port: u16,
    pub max_connections: u32,
}

#[derive(Clone, Copy)]
pub struct Connection {
    pub ip: IpAddr,
    pub port: u16,
    pub connection_type: ConnectionType,
}

impl From<ClusterInfo> for Connection {
    fn from(info: ClusterInfo) -> Self {
        Connection {
            ip: IpAddr::from_str(info.ip.as_str()).expect("Failed to parse the IP."),
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

pub fn get_ip(ip: &str) -> IpAddr {
    IpAddr::from_str(ip).unwrap_or(
        IpAddr::from_str(DEFAULT_IP).unwrap_or(IpAddr::V4(Ipv4Addr::LOCALHOST))
    )
}

pub async fn cleanup() {}

pub async fn start<P>(plugin: P) where P: ClientPlugin + Send + Sync + 'static {
    // Get the connection LOGGER.information.
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
    plugin.set_sender(tx.clone());

    let handler = tokio::spawn(async move {
        LOGGER.warning(format!("Connecting to the {connection_type}...").as_str());
        let mut stream = TcpStream::connect(format!("{}:{}", ip, port)).await.expect(
            format!("Failed to connect to the {connection_type} at {ip}:{port}.").as_str()
        );
        LOGGER.success(format!("Connected to the {connection_type} at {ip}:{port}.").as_str());

        let (reader, mut writer) = stream.split();
        let mut reader = BufReader::new(reader);

        lselect! {
            command = reader.read_u8() => {
                if command.is_err() {
                    continue;
                }

                LOGGER.info(format!("Received data: {:?}", command).as_str());

                match connection_type {
                    ConnectionType::MasterServer => match command.unwrap() {
                        x if x == ToUnknown::SendClusters as u8 => {
                            let amount = match reader.read_u8().await {
                                Ok(amount) => amount,
                                Err(_) => {
                                    LOGGER.error("Failed to read the amount of clusters.");
                                    continue;
                                }
                            };

                            let mut cluster_servers_tmp = Vec::new();
                            for _ in 0..amount {
                                let name = lread_string!(reader, |msg| LOGGER.error(msg), "cluster name");
                                let ip = lread_string!(reader, |msg| LOGGER.error(msg), "cluster IP");
                                let port = match reader.read_u16().await {
                                    Ok(port) => port,
                                    Err(_) => {
                                        LOGGER.error("Failed to read the cluster port.");
                                        continue;
                                    }
                                };
                                let max_connections = match reader.read_u32().await {
                                    Ok(max_connections) => max_connections,
                                    Err(_) => {
                                        LOGGER.error("Failed to read the cluster max connections.");
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

                                    LOGGER.success(format!("Received {amount} Cluster servers from the {connection_type}.").as_str());
                                    println!("{:?}", *cluster_servers);
                                }
                            }
                        },
                        cmd => plugin.receive_master(tx.clone(), cmd, &mut reader).await,
                    }
                    ConnectionType::ClusterServer => match command.unwrap() {
                        x if x == ToClient::SendClusters as u8 => {
                            let amount = match reader.read_u8().await {
                                Ok(amount) => amount,
                                Err(_) => {
                                    LOGGER.error("Failed to read the amount of clusters.");
                                    continue;
                                }
                            };

                            let mut cluster_servers_tmp = Vec::new();
                            for _ in 0..amount {
                                let name = lread_string!(reader, |msg| LOGGER.error(msg), "cluster name");
                                let ip = lread_string!(reader, |msg| LOGGER.error(msg), "cluster IP");
                                let port = match reader.read_u16().await {
                                    Ok(port) => port,
                                    Err(_) => {
                                        LOGGER.error("Failed to read the cluster port.");
                                        continue;
                                    }
                                };
                                let max_connections = match reader.read_u32().await {
                                    Ok(max_connections) => max_connections,
                                    Err(_) => {
                                        LOGGER.error("Failed to read the cluster max connections.");
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

                                    LOGGER.success(format!("Received {amount} Cluster servers from the {connection_type}.").as_str());
                                    println!("{:?}", *cluster_servers);
                                }
                            }
                        },
                        x if x == ToClient::DisconnectCluster as u8 => todo!(),
                        x if x == ToClient::LeaveCluster as u8 => todo!(),

                        x if x == ToClient::VersionOfKey as u8 => todo!(),
                        x if x == ToClient::SendPubKey as u8 => todo!(),
                        x if x == ToClient::Authenticate as u8 => todo!(),

                        x if x == ToClient::Move as u8 => todo!(),
                        cmd => plugin.receive_cluster(tx.clone(), cmd, &mut reader).await,
                    }
                    _ => (),
                }
            }
            result = rx.recv() => {
                if let Some(data) = result {
                    if data.is_empty() {
                        writer.shutdown().await.expect("Failed to shutdown the writer.");
                        LOGGER.info("Closing connection...");
                        break;
                    }

                    writer.write_all(&data).await.expect("Failed to write to the Server.");
                    writer.flush().await.expect("Failed to flush the writer.");
                    LOGGER.info(format!("Sent {data:?} as data to the {connection_type}.").as_str());
                } else {
                    writer.shutdown().await.expect("Failed to shutdown the writer.");
                    LOGGER.info("Shutting down connection...");
                    break;
                }
            }
        }
    });

    let _ = handler.await;
}

pub async fn send_data(tx: &Sender<Box<[u8]>>, data: Box<[u8]>) {
    tx.send(data).await.expect("Failed to send data to the Server.");
}

pub async fn join_cluster(tx: &Sender<Box<[u8]>>, id: usize) {
    if id < (0 as usize) {
        LOGGER.error("Failed to join a cluster. The cluster ID is invalid (less than 0).");
        return;
    }

    let cluster_servers = CLUSTER_SERVERS.read().await;
    if cluster_servers.is_empty() {
        LOGGER.error("Failed to join a cluster. No cluster servers are available.");
        return;
    }

    if id >= cluster_servers.len() {
        LOGGER.error(
            "Failed to join a cluster. The cluster ID is invalid (greater than the amount of clusters)."
        );
        return;
    }

    let cluster = (
        match cluster_servers.get(id) {
            Some(cluster) => cluster,
            None => {
                LOGGER.error("Failed to join a cluster. The cluster ID is invalid.");
                return;
            }
        }
    ).clone();

    LOGGER.success(format!("Client is joining cluster {}", cluster.name).as_str());

    let connection = match std::panic::catch_unwind(|| Connection::from(cluster)) {
        Ok(connection) => connection,
        Err(_) => {
            LOGGER.error("Failed to create a connection with the Cluster Server.");
            return;
        }
    };
    {
        // Overwrite the current connection with the cluster connection.
        *CONNECTION.write().await = Some(connection);
        stop(tx).await;
    }
}

async fn stop(tx: &Sender<Box<[u8]>>) {
    tx.send(Box::new([])).await.expect("Failed to shutdown.");
}
