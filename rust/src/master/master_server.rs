use std::{ collections::BTreeSet, sync::Arc };

use dashmap::DashMap;
use tokio::{
    io::{ AsyncBufReadExt, AsyncWriteExt, BufReader },
    net::{ TcpListener, TcpStream },
    sync::{ broadcast, mpsc, Mutex },
};

use crate::{
    clients::ServerClient,
    events::Event,
    transport::base_server::*,
    utils::constants,
    world::ClusterInfo,
};

pub struct MasterServer {
    pub is_running: bool,

    // Master Server Fields
    pub cluster_ids: Vec<u32>,
    pub cluster_info: DashMap<u32, ClusterInfo>,

    // Packet Handlers
    // pub packet_handlers: DashMap<u32, PacketHandler>,

    // Network
    tcp_listener: TcpListener,
    // UDP equivalent is in BaseClient.UdpHandler.socket

    // Server Info
    pub max_connections: u32,
    pub port: u16,

    // Data
    pub clients: DashMap<u32, ServerClient>,
    pub released_ids: Arc<Mutex<BTreeSet<u32>>>,

    // Events
    pub event_receiver: mpsc::Receiver<Event>,
    pub event_sender: mpsc::Sender<Event>,
}

#[derive(Debug)]
pub enum MasterServerError {
    ClientMissing,
    ClientMissingID,
    TCPError,
    AddClientError,

    MaxConnectionsReached,
}
impl From<MasterServerError> for String {
    fn from(error: MasterServerError) -> String {
        match error {
            MasterServerError::ClientMissing => "Client is missing".to_string(),
            MasterServerError::ClientMissingID => "Client is missing an ID".to_string(),
            MasterServerError::TCPError => "Failed to read TCP data".to_string(),
            MasterServerError::AddClientError => "Failed to add client".to_string(),

            MasterServerError::MaxConnectionsReached => "Max connections reached".to_string(),
        }
    }
}

impl ServerCore<MasterServerError> for MasterServer {
    type Server = MasterServer;

    /// Creates a new MasterServer.
    ///
    /// * `max_connenctions` Defaults the max_connections to unlimited.
    /// * `port` Defaults to `utils::constants::MASTER_PORT`.
    async fn new(
        max_connections: Option<u32>,
        port: Option<u16>
    ) -> Result<Self, MasterServerError> {
        let listener = TcpListener::bind(
            format!("{}:{}", constants::DEFAULT_IP, port.unwrap_or(constants::MASTER_PORT))
        ).await.unwrap();
        let (event_sender, event_receiver) = mpsc::channel(100);

        Ok(MasterServer {
            is_running: false,

            cluster_ids: vec![],
            cluster_info: DashMap::new(),

            tcp_listener: listener,

            max_connections: max_connections.unwrap_or(0),
            port: constants::MASTER_PORT,

            clients: DashMap::new(),
            released_ids: Arc::new(Mutex::new(BTreeSet::new())),

            event_receiver,
            event_sender,
        })
    }

    /// Starts a server.
    async fn start(&mut self) {
        self.is_running = true;

        {
            let max_connections_str = match self.max_connections {
                0 => "unlimited max connections".to_string(),
                1 => "1 max connection".to_string(),
                _ => format!("{} max connections", self.max_connections),
            };

            Self::debug(
                format!(
                    "Starting the Master Server on port {} with {max_connections_str}...",
                    self.port
                )
            );
        }

        self.listen().await;
    }

    /// Listens for incoming connections and handles them.
    #[inline(always)]
    async fn listen(&self) -> Result<(), MasterServerError> {
        self.process_events();

        // let (tx, _rx) = broadcast::channel(10);

        Self::success("Now listening for connections.".to_string());

        while self.is_running {
            let (mut stream, addr) = self.tcp_listener.accept().await.unwrap();

            Self::debug(format!("Accepted connection from {:?}", addr));

            self.on_tcp_connection(stream).await;
        }

        Ok(())
    }
}

impl ServerConnection<MasterServerError> for MasterServer {
    async fn on_tcp_connection(&self, stream: TcpStream) {
        match self.add_client(stream).await {
            Ok(_) => (),
            Err(e) => Self::error(format!("Failed to add client: {:?}", e)),
        }
    }

    fn on_udp_received(&self) {
        todo!()
    }

    /// Adds a client to the server.
    async fn add_client(&self, stream: TcpStream) -> Result<(), MasterServerError> {
        // If the max_connections is reached, return an error.
        if self.max_connections != 0 && self.clients.len() >= (self.max_connections as usize) {
            return Err(MasterServerError::MaxConnectionsReached);
        }

        // Get the next available ID and insert it.
        let released_id: Option<u32> = self.released_ids.lock().await.pop_first();
        self.clients.insert(
            released_id.unwrap_or(self.clients.len() as u32),
            ServerClient::new(
                released_id.unwrap_or(self.clients.len() as u32),
                stream,
                self.event_sender.clone()
            )
        );

        Ok(())
    }

    fn disconnect_client(&self, client_id: u32) {
        self.clear_client(client_id);
    }

    async fn clear_client(&self, client_id: u32) {
        self.clients.remove(&client_id);

        if self.clients.len() == 0 {
            self.released_ids.lock().await.clear();
        } else if self.clients.len() > (client_id as usize) {
            self.released_ids.lock().await.insert(client_id);
        }

        Self::debug(format!("Disconnected Client#{client_id}"));
    }
}

impl ServerEvents for MasterServer {
    fn process_events(&self) {
        todo!()
    }

    fn on_connection(&self, id: u32) {
        todo!()
    }

    fn on_disconnection(&self, id: u32) {
        todo!()
    }

    fn on_received_data(&self, id: u32, data: &[u8]) {
        todo!()
    }

    fn on_client_connected(&self, id: u32) {
        todo!()
    }

    fn on_client_disconnected(&self, id: u32, protocol: crate::transport::Protocols) {
        todo!()
    }

    fn on_client_received_data(&self, id: u32, protocol: crate::transport::Protocols, data: &[u8]) {
        todo!()
    }
}

impl ServerLogging for MasterServer {
    fn debug(message: String) {
        if !constants::DEBUGGING {
            return;
        }

        crate::master_debug!("{}", message);
    }

    fn info(message: String) {
        if !constants::DEBUGGING {
            return;
        }

        crate::master_info!("{}", message);
    }

    fn warning(message: String) {
        if !constants::DEBUGGING {
            return;
        }

        crate::master_warning!("{}", message);
    }

    fn error(message: String) {
        if !constants::DEBUGGING {
            return;
        }

        crate::master_error!("{}", message);
    }

    fn success(message: String) {
        if !constants::DEBUGGING {
            return;
        }

        crate::master_success!("{}", message);
    }
}

impl MasterServer {}
