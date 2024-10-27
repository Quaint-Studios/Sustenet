use std::collections::HashMap;

use tokio::net::TcpListener;

use crate::utils::constants;
use crate::utils::constants::DEFAULT_IP;

use super::BaseClient;

pub enum ServerType {
    ClusterServer,
    MasterServer,
}

// List of possible errors
#[derive(Debug)]
pub enum BaseServerError {
    ClientMissing,
}

impl From<BaseServerError> for String {
    fn from(error: BaseServerError) -> String {
        match error {
            BaseServerError::ClientMissing => "Client is missing".to_string(),
        }
    }
}


type PacketHandler = fn(from_client: i32, packet: i32);

pub struct BaseServer {
    // Packet Handlers
    pub packet_handlers: Option<HashMap<i32, PacketHandler>>,

    // Network
    tcp_listener: TcpListener,
    // UDP equivalent is in BaseClient.UdpHandler.socket

    // Server Info
    server_type: ServerType,
    server_type_name: String,
    max_connections: u32,
    port: u16,

    // Data
    clients: HashMap<u32, BaseClient>,
    released_ids: Vec<u32>,

    // Events
    on_connection: Vec<Box<dyn Fn(u32) + Send + Sync>>,
    on_disconnection: Vec<Box<dyn Fn(u32) + Send + Sync>>,
    on_received: Vec<Box<dyn Fn(u32, i32) + Send + Sync>>,
}

impl BaseServer {
    pub async fn new(
        server_type: ServerType,
        max_connections: u32,
        port: Option<u16>
    ) -> Result<Self, BaseServerError> {
        let port = port.unwrap_or(constants::MASTER_PORT);
        let server_type_name = (
            match server_type {
                ServerType::ClusterServer => "Cluster Server",
                ServerType::MasterServer => "Master Server",
            }
        ).to_string();

        Ok(BaseServer {
            packet_handlers: None,

            tcp_listener: TcpListener::bind(format!("{DEFAULT_IP}:{port}")).await.unwrap(),

            server_type,
            server_type_name,
            max_connections,
            port,

            clients: HashMap::new(),
            released_ids: Vec::new(),

            on_connection: Vec::new(),
            on_disconnection: Vec::new(),
            on_received: Vec::new(),
        })
    }

    pub async fn start(&mut self) {
        println!("Starting the {}...", self.server_type_name);

        loop {

        }
    }
}
