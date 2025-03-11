use std::collections::BTreeSet;
use std::sync::Arc;
use std::{ net::Ipv4Addr, str::FromStr };

use dashmap::DashMap;

use shared::lselect;
use tokio::io::{ AsyncReadExt, AsyncWriteExt, BufReader };
use tokio::net::{ TcpListener, TcpStream };
use tokio::select;
use tokio::sync::mpsc::Sender;
use tokio::sync::{ mpsc, Mutex, RwLock };

use public_ip::addr;
use shared::config::cluster::{ read, Settings };
use shared::network::{ ClusterInfo, Event };
use shared::packets::cluster::FromClient;
use shared::packets::master::{ FromUnknown, ToUnknown };
use shared::security::aes::{ decrypt, generate_key, load_key, save_key };
use shared::utils::constants::DEFAULT_IP;
use shared::utils::{ self, constants };

lazy_static::lazy_static! {
    static ref CLUSTER_IDS: Arc<RwLock<BTreeSet<ClusterInfo>>> = Arc::new(
        RwLock::new(BTreeSet::new())
    );
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

    cleanup().await;
    success("The Cluster Server has been shut down.");
}

fn get_ip(ip: &str) -> Ipv4Addr {
    Ipv4Addr::from_str(ip).unwrap_or(Ipv4Addr::from_str(DEFAULT_IP).unwrap_or(Ipv4Addr::LOCALHOST))
}

async fn cleanup() {}

async fn start() {
    let Settings {
        server_name,
        max_connections,
        port,
        key_name,
        master_ip,
        master_port,
        domain_pub_key: _,
    } = read();
    let key = match load_key(key_name.as_str()) {
        Ok(key) => key,
        Err(_) => {
            if std::fs::DirBuilder::new().recursive(true).create("keys").is_err() {
                error("Failed to create the 'keys' directory.");
                panic!();
            }

            let key = generate_key();
            if save_key(key_name.as_str(), key).is_err() {
                error("Failed to save the generated key.");
                panic!();
            }

            warning(
                format!(
                    "A new AES key at 'keys/{key_name}' has been generated and saved. Make sure the Master Server also has this key for authentication."
                ).as_str()
            );

            key
        }
    };

    let (tx, mut rx) = mpsc::channel::<Box<[u8]>>(10);
    let tx_clone = tx.clone();

    // Cluster Server's connection to the Master Server.
    tokio::spawn(async move {
        let mut stream = TcpStream::connect(
            format!("{}:{}", get_ip(&master_ip), master_port)
        ).await.expect("Failed to connect to the Master Server.");

        let (reader, mut writer) = stream.split();
        let mut reader = BufReader::new(reader);

        loop {
            select! {
                command = reader.read_u8() => {
                    if command.is_err() {
                        continue;
                    }

                    debug(format!("Cluster Server received data: {:?}", command).as_str());

                    match command.unwrap() {
                        x if x == ToUnknown::VerifyCluster as u8 => {
                            let len = reader.read_u8().await.unwrap() as usize;
                            let mut passphrase = vec![0u8; len];
                            match reader.read_exact(&mut passphrase).await {
                                Ok(_) => {},
                                Err(e) => {
                                    error(format!("Failed to read passphrase to String: {:?}", e).as_str());
                                    continue;
                                }
                            }

                            let mut data = vec![FromUnknown::AnswerCluster as u8];

                            let decrypted_passphrase = decrypt(passphrase.as_slice(), &key);

                            data.push(decrypted_passphrase.len() as u8);
                            data.extend_from_slice(&decrypted_passphrase);
                            data.push(server_name.len() as u8);
                            data.extend_from_slice(&server_name.as_bytes());

                        if let Some(ip) = addr().await {
                            let ip_string = ip.to_string();
                            let ip_bytes = ip_string.as_bytes();
                            data.push(ip_bytes.len() as u8);
                            data.extend_from_slice(&ip.to_string().as_bytes());
                        } else {
                            error("Failed to get the public IP address.");
                            return;
                        }

                        data.extend_from_slice(&port.to_be_bytes());
                        data.extend_from_slice(&max_connections.to_be_bytes());


                        send_data(&tx, data.into_boxed_slice()).await;
                    }
                    x if x == ToUnknown::CreateCluster as u8 => {
                        success("We did it! We verified the cluster!");
                    }
                    _ => (),
                }
            }
                result = rx.recv() => {
                    if let Some(data) = result {
                        writer.write_all(&data).await.expect("Failed to write to the Master Server.");
                        writer.flush().await.expect("Failed to flush the writer.");
                    } else {
                        writer.shutdown().await.expect("Failed to shutdown the writer.");
                        info("Cluster Server is shutting down its client writer.");
                        break;
                    }
                }
            }
        }

        let (event_sender, mut event_receiver) = mpsc::channel::<Event>(100);

        let clients: DashMap<u32, ServerClient> = DashMap::new();
        let released_ids: Arc<Mutex<BTreeSet<u32>>> = Arc::new(Mutex::new(BTreeSet::new())); // In the future, think about reserving cluster ids. Sometimes a cluster can get a high ID, causing RAM to stay high during low loads.

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
    });

    // Send a request to the Master Server to become a cluster.
    {
        let command = FromUnknown::BecomeCluster as u8;

        let mut data = [command].to_vec();
        data.push(key_name.len() as u8);
        data.extend_from_slice(key_name.as_bytes());

        let data = data.into_boxed_slice();
        send_data(&tx_clone, data).await;
    }

    // Cluster Server Listener
    {
        let (event_sender, mut event_receiver) = mpsc::channel::<Event>(100);

        let clients: DashMap<u32, ServerClient> = DashMap::new();
        let released_ids: Arc<Mutex<BTreeSet<u32>>> = Arc::new(Mutex::new(BTreeSet::new()));

        {
            let max_connections_str = match max_connections {
                0 => "unlimited max connections".to_string(),
                1 => "1 max connection".to_string(),
                _ => format!("{} max connections", max_connections),
            };

            debug(
                format!("Starting the Cluster Server on port {} with {max_connections_str}...", port).as_str()
            );
        }

        // Listen
        {
            let tcp_listener = TcpListener::bind(
                format!("{}:{}", constants::DEFAULT_IP, port)
            ).await.expect("Failed to bind to the specified port.");

            lselect! {
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

async fn send_data(tx: &mpsc::Sender<Box<[u8]>>, data: Box<[u8]>) {
    tx.send(data).await.expect("Failed to send data to the Server.");
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
use shared::{ log_message, utils::constants::DEBUGGING };

fn debug(message: &str) {
    if !DEBUGGING {
        return;
    }
    log_message!(LogLevel::Debug, LogType::Cluster, "{}", message);
}

fn info(message: &str) {
    if !DEBUGGING {
        return;
    }
    log_message!(LogLevel::Info, LogType::Cluster, "{}", message);
}

fn warning(message: &str) {
    if !DEBUGGING {
        return;
    }
    log_message!(LogLevel::Warning, LogType::Cluster, "{}", message);
}

fn error(message: &str) {
    if !DEBUGGING {
        return;
    }
    log_message!(LogLevel::Error, LogType::Cluster, "{}", message);
}

fn success(message: &str) {
    if !DEBUGGING {
        return;
    }
    log_message!(LogLevel::Success, LogType::Cluster, "{}", message);
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
        let _name = self.name.clone(); // TODO: Implement name handling.
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

                        debug(format!("Cluster Server received data: {:?}", command).as_str());

                        match command.unwrap() {
                            x if x == FromClient::RequestClusters as u8 => {
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
