use std::collections::BTreeSet;
use std::sync::Arc;

use dashmap::DashMap;

use tokio::io::{ AsyncReadExt, AsyncWriteExt, BufReader };
use tokio::net::{ TcpListener, TcpStream };
use tokio::select;
use tokio::sync::mpsc::{ self, Sender };
use tokio::sync::{ Mutex, RwLock };

use shared::config::master::{ read, Settings };
use shared::log_message;
use shared::network::*;
use shared::packets::master::*;
use shared::security::aes::*;
use shared::utils::{ self, constants };

pub mod security;

lazy_static::lazy_static! {
    static ref CLUSTER_IDS: Arc<RwLock<BTreeSet<ClusterInfo>>> = Arc::new(
        RwLock::new(BTreeSet::new())
    );
}

#[derive(Eq)]
struct ClusterInfo {
    id: u32,
    name: String,
    ip: String,
    port: u16,
    max_connections: u32,
}

impl Ord for ClusterInfo {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        // Define how to compare two ClusterInfo instances
        // For example, if ClusterInfo has a field `id` of type i32:
        self.id.cmp(&other.id)
    }
}

impl PartialOrd for ClusterInfo {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for ClusterInfo {
    fn eq(&self, other: &Self) -> bool {
        // Define when two ClusterInfo instances are equal
        // For example, if ClusterInfo has a field `id` of type i32:
        self.id == other.id
    }
}

#[tokio::main]
async fn main() {
    let mut shutdown_rx = utils::shutdown_channel().expect("Error creating shutdown channel.");

    select! {
        _ = shutdown_rx.recv() => {
            warning("Shutting down...");
        }
        _ = start() => {}
    }

    success("The Master Server has been shut down.");
}

/// This function starts the master server.
/// It listens for an event
async fn start() {
    let Settings { server_name: _, max_connections, port } = read();
    let (event_sender, mut event_receiver) = mpsc::channel::<Event>(100);

    let clients: DashMap<u32, ServerClient> = DashMap::new();
    let released_ids: Arc<Mutex<BTreeSet<u32>>> = Arc::new(Mutex::new(BTreeSet::new())); // In the future, think about reserving cluster ids. Sometimes a cluster can get a high ID, causing RAM to stay high during low loads.

    {
        let max_connections_str = match max_connections {
            0 => "unlimited max connections".to_string(),
            1 => "1 max connection".to_string(),
            _ => format!("{} max connections", max_connections),
        };

        debug(
            format!("Starting the Master Server on port {} with {max_connections_str}...", port).as_str()
        );
    }

    // Listen
    {
        let tcp_listener = TcpListener::bind(
            format!("{}:{}", constants::DEFAULT_IP, port)
        ).await.expect("Failed to bind to the specified port.");

        loop {
            select! {
                event = event_receiver.recv() => {
                    if let Some(event) = event {
                        match event {
                            Event::Connection(id) => on_connection(id),
                            Event::Disconnection(id) => {
                                debug(format!("Client#{id} disconnected.").as_str());
                                clients.remove(&id);

                                if id >= clients.len() as u32 {
                                    info(format!("Client#{id} wasn't added to the released IDs list.").as_str());
                                    continue;
                                }

                                let mut ids = released_ids.lock().await;
                                if !(*ids).insert(id) {
                                    error(format!("ID {} already exists in the released IDs.", id).as_str());
                                    continue;
                                };
                            },
                            Event::ReceivedData(id, data) => on_received_data(id, &data),
                        }
                    }
                }
                // Listen and add clients.
                res = tcp_listener.accept() => {
                    if let Ok((stream, addr)) = res {
                        debug(format!("Accepted connection from {:?}", addr).as_str());

                        // If the max_connections is reached, return an error.
                        if max_connections != 0 && clients.len() >= (max_connections as usize) {
                            error("Max connections reached.");
                            continue;
                        }

                        // Get the next available ID and insert it.
                        let released_id: u32 = released_ids
                            .lock().await
                            .pop_first()
                            .unwrap_or(clients.len() as u32);
                        let mut client = ServerClient::new(released_id);
                        client.handle_data(event_sender.clone(), stream).await;
                        clients.insert(released_id, client);

                        event_sender.send(Event::Connection(released_id)).await.unwrap();
                    }
                }
            }
        }
    }
}

// region: Events
fn on_connection(id: u32) {
    debug(format!("Client#{id} connected").as_str());
}

fn on_received_data(id: u32, data: &[u8]) {
    debug(format!("Received data from Client#{id}: {:?}", data).as_str());
    todo!()
}

// fn on_client_connected(id: u32) {
//     debug(format!("Client connected: {}", id).as_str());
//     todo!()
// }

// fn on_client_disconnected(id: u32, protocol: Protocols) {
//     debug(format!("Client disconnected: {} {}", id, protocol as u8).as_str());
//     todo!()
// }

// fn on_client_received_data(id: u32, protocol: Protocols, data: &[u8]) {
//     debug(format!("Client received data: {} {} {:?}", id, protocol as u8, data).as_str());
//     todo!()
// }
// endregion

// region: Logging
fn debug(message: &str) {
    if !constants::DEBUGGING {
        return;
    }
    log_message!(LogLevel::Debug, LogType::Master, "{}", message);
}

fn info(message: &str) {
    if !constants::DEBUGGING {
        return;
    }
    log_message!(LogLevel::Info, LogType::Master, "{}", message);
}

fn warning(message: &str) {
    if !constants::DEBUGGING {
        return;
    }
    log_message!(LogLevel::Warning, LogType::Master, "{}", message);
}

fn error(message: &str) {
    if !constants::DEBUGGING {
        return;
    }
    log_message!(LogLevel::Error, LogType::Master, "{}", message);
}

fn success(message: &str) {
    if !constants::DEBUGGING {
        return;
    }
    log_message!(LogLevel::Success, LogType::Master, "{}", message);
}
// endregion

pub struct ServerClient {
    pub id: u32,
    pub name: Arc<RwLock<Option<String>>>,
    pub sender: Option<Sender<Box<[u8]>>>,
}

impl ServerClient {
    pub fn new(id: u32) -> Self {
        ServerClient {
            id,
            name: Arc::new(RwLock::new(None)),
            sender: None,
        }
    }

    /// Handle the data from the client.
    pub async fn handle_data(&mut self, event_sender: Sender<Event>, mut stream: TcpStream) {
        let id = self.id;
        let name = self.name.clone();
        let (tx, mut rx) = tokio::sync::mpsc::channel(10);
        self.sender = Some(tx.clone());

        tokio::spawn(async move {
            let (reader, mut writer) = stream.split();

            let mut reader = BufReader::new(reader);

            loop {
                select! {
                    // Incoming data from the client.
                    command = reader.read_u8() => {
                        if command.is_err() {
                            event_sender.send(Event::Disconnection(id)).await.expect("Failed to send disconnection event.");
                            break;
                        }

                        debug(format!("Received data from Client#{id}: {:?}", command).as_str());

                        match command.unwrap() {
                            x if x == FromUnknown::RequestClusters as u8 => {
                                let mut data = vec![ToUnknown::SendClusters as u8];
                                let cluster_ids = CLUSTER_IDS.read().await;
                                data.push(cluster_ids.len() as u8);
                                for cluster in (*cluster_ids).iter() {
                                    data.push(cluster.name.len() as u8);
                                    data.extend_from_slice(cluster.name.as_bytes());
                                    data.push(cluster.ip.len() as u8);
                                    data.extend_from_slice(cluster.ip.as_bytes());
                                    data.extend_from_slice(&cluster.port.to_be_bytes());
                                    data.extend_from_slice(&cluster.max_connections.to_be_bytes());
                                }
                                Self::send_data(&tx, data.into_boxed_slice()).await;
                            },
                            x if x == FromUnknown::JoinCluster as u8 => {
                                event_sender.send(Event::Disconnection(id)).await.expect("Failed to send disconnection event.");
                                break;
                            },
                            x if x == FromUnknown::BecomeCluster as u8 => {
                                let len = match reader.read_u8().await {
                                    Ok(len) => len,
                                    Err(e) => {
                                        error(format!("Failed to read cluster name length: {:?}", e).as_str());
                                        continue;
                                    }
                                } as usize;
                                let mut key_name = vec![0u8; len];
                                match reader.read_exact(&mut key_name).await {
                                    Ok(_) => {},
                                    Err(e) => {
                                        error(format!("Failed to read cluster name to String: {:?}", e).as_str());
                                        continue;
                                    }
                                }
                                let key_name = String::from_utf8(key_name).unwrap();
                                let key = match security::AES_KEYS.get(&key_name) {
                                    Some(key) => key,
                                    None => {
                                        error(format!("Key {} doesn't exist.", key_name).as_str());
                                        continue;
                                    }
                                };

                                let mut data = vec![ToUnknown::VerifyCluster as u8];

                                let passphrase = &security::generate_passphrase();
                                let encrypted_passphrase = encrypt(passphrase, key);

                                data.push(encrypted_passphrase.len() as u8);
                                data.extend_from_slice(&encrypted_passphrase);

                                {
                                    let mut name = name.write().await;
                                    *name = Some(String::from_utf8(passphrase.to_vec()).unwrap());
                                }
                                Self::send_data(&tx, data.into_boxed_slice()).await;
                            },
                            x if x == FromUnknown::AnswerCluster as u8 => {
                                let len = reader.read_u8().await.unwrap() as usize;
                                let mut passphrase = vec![0u8; len];
                                match reader.read_exact(&mut passphrase).await {
                                    Ok(_) => {},
                                    Err(e) => {
                                        error(format!("Failed to read the passphrase to String: {:?}", e).as_str());
                                        continue;
                                    }
                                }

                                {
                                    let passphrase = match String::from_utf8(passphrase) {
                                        Ok(passphrase) => passphrase,
                                        Err(e) => {
                                            error(format!("Failed to convert passphrase to String: {:?}", e).as_str());
                                            continue;
                                        }
                                    };

                                    let name = name.read().await;
                                    if (*name).is_none() || passphrase != *name.as_ref().expect("Failed to get saved passphrase.") {
                                        error("The passphrase doesn't match the name.");
                                        continue;
                                    } else {
                                        success(format!("The passphrase matches the name: {:?} is {}", *name, passphrase).as_str());
                                    }
                                }

                                // Read their new name they sent.
                                let len = reader.read_u8().await.unwrap() as usize;
                                let mut server_name = vec![0u8; len];
                                match reader.read_exact(&mut server_name).await {
                                    Ok(_) => {},
                                    Err(e) => {
                                        error(format!("Failed to read the server name to String: {:?}", e).as_str());
                                        continue;
                                    }
                                };

                                let server_name = match String::from_utf8(server_name) {
                                    Ok(server_name) => server_name,
                                    Err(e) => {
                                        error(format!("Failed to convert server name to String: {:?}", e).as_str());
                                        continue;
                                    }
                                };

                                {
                                    let mut name = name.write().await;
                                    *name = Some(server_name.clone());
                                }

                                {
                                    // Read IP to len. Read port u16. Read max connections u32.
                                    let len = match reader.read_u8().await {
                                        Ok(len) => len,
                                        Err(e) => {
                                            error(format!("Failed to read the IP length: {:?}", e).as_str());
                                            continue;
                                        }
                                    } as usize;
                                    let mut ip = vec![0u8; len];
                                    match reader.read_exact(&mut ip).await {
                                        Ok(_) => {},
                                        Err(e) => {
                                            error(format!("Failed to read the IP to String: {:?}", e).as_str());
                                            continue;
                                        }
                                    }
                                    let ip = match String::from_utf8(ip) {
                                        Ok(ip) => ip,
                                        Err(e) => {
                                            error(format!("Failed to convert IP to String: {:?}", e).as_str());
                                            continue;
                                        }
                                    };
                                    let port = match reader.read_u16().await {
                                        Ok(port) => port,
                                        Err(e) => {
                                            error(format!("Failed to read the port: {:?}", e).as_str());
                                            continue;
                                        }
                                    };
                                    let max_connections = match reader.read_u32().await {
                                        Ok(max_connections) => max_connections,
                                        Err(e) => {
                                            error(format!("Failed to read the max connections: {:?}", e).as_str());
                                            continue;
                                        }
                                    };

                                    let mut cluster_ids = CLUSTER_IDS.write().await;
                                    if (*cluster_ids).insert(ClusterInfo {
                                        id,
                                        name: server_name,
                                        ip,
                                        port,
                                        max_connections,
                                    }) {
                                        success(format!("Client#{id} has become a cluster.").as_str());
                                    } else {
                                        error(format!("Client#{id} failed to become a cluster.").as_str());
                                        continue;
                                    }
                                }

                                Self::send_data(&tx, Box::new([ToUnknown::CreateCluster as u8])).await;
                            },

                            // Cluster Section

                            _ => (),
                        }
                    }
                    // Outgoing data to the client.
                    result = rx.recv() => {
                        if let Some(data) = result {
                            writer.write_all(&data).await.expect("Failed to write to the Master Server.");
                            writer.flush().await.expect("Failed to flush the writer.");
                        } else {
                            writer.shutdown().await.expect("Failed to shutdown the writer.");
                            info("Cluster Server is shutting down its client writer.");
                            event_sender.send(Event::Disconnection(id)).await.expect("Failed to send disconnection event.");
                            break;
                        }
                    }
                }
            }
        });
    }

    async fn send_data(tx: &mpsc::Sender<Box<[u8]>>, data: Box<[u8]>) {
        tx.send(data).await.expect("Failed to send data out.");
    }
}
