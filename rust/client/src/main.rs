use std::sync::Arc;
use std::{ net::Ipv4Addr, str::FromStr };

use tokio::io::{ AsyncReadExt, AsyncWriteExt, BufReader };
use tokio::net::TcpStream;
use tokio::select;
use tokio::sync::{ mpsc, RwLock };

use shared::log_message;
use shared::packets::cluster::ToClient;
use shared::packets::master::{ FromUnknown, ToUnknown };
use shared::utils::constants::{ DEFAULT_IP, MASTER_PORT };
use shared::utils::{ self, constants };

lazy_static::lazy_static! {
    static ref CLUSTER_SERVERS: Arc<RwLock<Vec<ClusterInfo>>> = Arc::new(RwLock::new(Vec::new()));
}

#[derive(Debug)]
pub struct ClusterInfo {
    pub name: String,
    pub ip: String,
    pub port: u16,
    pub max_connections: u32,
}

pub enum ConnectionType {
    MasterServer,
    ClusterServer,
    None,
}

#[tokio::main]
async fn main() {
    let mut shutdown_rx = utils::shutdown_channel().expect("Error creating shutdown channel.");

    select! {
        _ = shutdown_rx.recv() => {
            warning("Shutting down...");
        }
        _ = start(get_ip(DEFAULT_IP), MASTER_PORT) => {}
    }

    cleanup().await;
    success("The Client has been shut down.");
}

fn get_ip(ip: &str) -> Ipv4Addr {
    Ipv4Addr::from_str(ip).unwrap_or(Ipv4Addr::from_str(DEFAULT_IP).unwrap_or(Ipv4Addr::LOCALHOST))
}

async fn cleanup() {}

async fn start(ip: Ipv4Addr, port: u16) {
    // TODO: Mutating this may not work since it's moved.
    let connection_type = ConnectionType::MasterServer;

    let (tx, mut rx) = mpsc::channel::<Box<[u8]>>(10);

    let handler = tokio::spawn(async move {
        let mut stream = TcpStream::connect(format!("{}:{}", ip, port)).await.expect(
            "Failed to connect to the Master Server."
        );

        let (reader, mut writer) = stream.split();
        let mut reader = BufReader::new(reader);

        loop {
            select! {
                command = reader.read_u8() => {
                    if command.is_err() {
                        continue;
                    }

                    debug(format!("Client received data: {:?}", command).as_str());

                    match connection_type {
                        ConnectionType::MasterServer => match command.unwrap() {
                            x if x == ToUnknown::SendClusters as u8 => {
                                //amount
                                let amount = match reader.read_u8().await {
                                    Ok(amount) => amount,
                                    Err(_) => {
                                        error("Failed to read the amount of clusters.");
                                        continue;
                                    }
                                };

                                let mut cluster_servers_tmp = Vec::new();
                                for _ in 0..amount {
                                    //name
                                    let name_len = match reader.read_u8().await {
                                        Ok(len) => len,
                                        Err(e) => {
                                            error(format!("Failed to read the cluster name len. {:?}", e).as_str());
                                            continue;
                                        }
                                    } as usize;
                                    let mut name = vec![0u8; name_len as usize];
                                    match reader.read_exact(&mut name).await {
                                        Ok(_) => (),
                                        Err(e) => {
                                            error(format!("Failed to read the cluster name. {:?}", e).as_str());
                                            continue;
                                        }
                                    };
                                    let name = match String::from_utf8(name) {
                                        Ok(name) => name,
                                        Err(e) => {
                                            error(format!("Failed to convert the cluster name to a String. {:?}", e).as_str());
                                            continue;
                                        }
                                    };
                                    //ip
                                    let ip_len = match reader.read_u8().await {
                                        Ok(len) => len,
                                        Err(e) => {
                                            error(format!("Failed to read the cluster IP len. {:?}", e).as_str());
                                            continue;
                                        }
                                    } as usize;
                                    let mut ip = vec![0u8; ip_len as usize];
                                    match reader.read_exact(&mut ip).await {
                                        Ok(_) => (),
                                        Err(e) => {
                                            error(format!("Failed to read the cluster IP. {:?}", e).as_str());
                                            continue;
                                        }
                                    };
                                    let ip = match String::from_utf8(ip) {
                                        Ok(ip) => ip,
                                        Err(e) => {
                                            error(format!("Failed to convert the cluster IP to a String. {:?}", e).as_str());
                                            continue;
                                        }
                                    };
                                    //port
                                    let port = match reader.read_u16().await {
                                        Ok(port) => port,
                                        Err(_) => {
                                            error("Failed to read the cluster port.");
                                            continue;
                                        }
                                    };
                                    //max_connections
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

                                let mut cluster_servers = CLUSTER_SERVERS.write().await;
                                *cluster_servers = cluster_servers_tmp;

                                success("Client received the cluster servers from the Master Server.");
                                println!("{:?}", *cluster_servers);
                            },
                            _ => (),
                        }
                        ConnectionType::ClusterServer => match command.unwrap() {
                            x if x == ToClient::SendClusters as u8 => todo!(),
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
                        writer.write_all(&data).await.expect("Failed to write to the Server.");
                        writer.flush().await.expect("Failed to flush the writer.");
                        success(format!("Client sent {data:?} as data to the Master Server.").as_str());
                    } else {
                        writer.shutdown().await.expect("Failed to shutdown the writer.");
                        info("Client is shutting down its writer.");
                        break;
                    }
                }
            }
        }
    });

    send_data(&tx, Box::new([FromUnknown::RequestClusters as u8])).await;

    match handler.await {
        Ok(_) => {}
        Err(e) => {
            error(format!("Error: {:?}", e).as_str());
        }
    }
}

async fn send_data(tx: &mpsc::Sender<Box<[u8]>>, data: Box<[u8]>) {
    tx.send(data).await.expect("Failed to send data to the Server.");
}

// region: Logging
fn debug(message: &str) {
    if !constants::DEBUGGING {
        return;
    }
    log_message!(LogLevel::Debug, LogType::Client, "{}", message);
}

fn info(message: &str) {
    if !constants::DEBUGGING {
        return;
    }
    log_message!(LogLevel::Info, LogType::Client, "{}", message);
}

fn warning(message: &str) {
    if !constants::DEBUGGING {
        return;
    }
    log_message!(LogLevel::Warning, LogType::Client, "{}", message);
}

fn error(message: &str) {
    if !constants::DEBUGGING {
        return;
    }
    log_message!(LogLevel::Error, LogType::Client, "{}", message);
}

fn success(message: &str) {
    if !constants::DEBUGGING {
        return;
    }
    log_message!(LogLevel::Success, LogType::Client, "{}", message);
}
// endregion
