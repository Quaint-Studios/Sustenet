use std::{ collections::BTreeSet, sync::Arc };

use dashmap::DashMap;

use tokio::{ net::{ TcpListener, TcpStream }, sync::{ mpsc::{ self, Receiver, Sender }, Mutex } };

use crate::{
    clients::ServerClient,
    events::Event,
    transport::base_server::*,
    transport::Logging,
    utils::constants,
    world::ClusterInfo,
};

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

pub struct MasterServer {
    pub is_running: bool,

    // Master Server Fields
    pub cluster_ids: Vec<u32>,
    pub cluster_info: DashMap<u32, ClusterInfo>,

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
    pub event_receiver: Receiver<Event>,
    pub event_sender: Sender<Event>,
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
                ).as_str()
            );
        }

        match self.listen().await {
            Ok(_) => (),
            Err(e) => Self::error(format!("Failed to start server: {:?}", e).as_str()),
        }
    }

    /// Listens for incoming connections and handles them.
    #[inline(always)]
    async fn listen(&mut self) -> Result<(), MasterServerError> {
        tokio::join!(Self::process_events(&mut self.event_receiver), async {
            // let (tx, _rx) = broadcast::channel(10);

            Self::success("Now listening for connections.");

            while self.is_running {
                let (stream, addr) = self.tcp_listener.accept().await.unwrap();

                Self::debug(format!("Accepted connection from {:?}", addr).as_str());

                match
                    Self::add_client(
                        stream,
                        &self.max_connections,
                        &self.clients,
                        self.released_ids.clone(),
                        &self.event_sender
                    ).await
                {
                    Ok(_) => (),
                    Err(e) => Self::error(format!("Failed to add client: {:?}", e).as_str()),
                };
            }
        });

        Ok(())
    }
}

impl ServerConnection<MasterServerError> for MasterServer {
    fn on_udp_received(&self) {
        todo!()
    }

    /// Adds a client to the server.
    async fn add_client(
        stream: TcpStream,
        max_connections: &u32,
        clients: &DashMap<u32, ServerClient>,
        released_ids: Arc<Mutex<BTreeSet<u32>>>,
        event_sender: &Sender<Event>
    ) -> Result<(), MasterServerError> {
        // If the max_connections is reached, return an error.
        if *max_connections != 0 && clients.len() >= (*max_connections as usize) {
            return Err(MasterServerError::MaxConnectionsReached);
        }

        // Get the next available ID and insert it.
        let released_id: u32 = released_ids
            .lock().await
            .pop_first()
            .unwrap_or(clients.len() as u32);
        let client = ServerClient::new(released_id, event_sender.clone());
        client.handle_data(stream).await;
        clients.insert(released_id, client);

        event_sender.send(Event::Connection(released_id)).await.unwrap();

        Ok(())
    }

    async fn disconnect_client(
        client_id: u32,
        clients: &DashMap<u32, ServerClient>,
        released_ids: Arc<Mutex<BTreeSet<u32>>>
    ) {
        clients.remove(&client_id);

        if clients.len() == 0 {
            released_ids.lock().await.clear();
        } else if clients.len() > (client_id as usize) {
            released_ids.lock().await.insert(client_id);
        }

        Self::debug(format!("Disconnected Client#{client_id}").as_str());
    }
}

impl ServerEvents for MasterServer {
    async fn process_events(event_receiver: &mut Receiver<Event>) {
        while let Some(event) = event_receiver.recv().await {
            match event {
                Event::Connection(id) => Self::on_connection(id),
                Event::Disconnection(id) => Self::on_disconnection(id),
                Event::ReceivedData(id, data) => Self::on_received_data(id, &data),
            }
        }
    }

    fn on_connection(id: u32) {
        Self::debug(format!("Client#{id} connected").as_str());
    }

    fn on_disconnection(id: u32) {
        todo!()
    }

    fn on_received_data(id: u32, data: &[u8]) {
        todo!()
    }

    fn on_client_connected(id: u32) {
        todo!()
    }

    fn on_client_disconnected(id: u32, protocol: crate::transport::Protocols) {
        todo!()
    }

    fn on_client_received_data(id: u32, protocol: crate::transport::Protocols, data: &[u8]) {
        todo!()
    }
}

impl Logging for MasterServer {
    fn debug(message: &str) {
        if !constants::DEBUGGING {
            return;
        }

        crate::master_debug!("{}", message);
    }

    fn info(message: &str) {
        if !constants::DEBUGGING {
            return;
        }

        crate::master_info!("{}", message);
    }

    fn warning(message: &str) {
        if !constants::DEBUGGING {
            return;
        }

        crate::master_warning!("{}", message);
    }

    fn error(message: &str) {
        if !constants::DEBUGGING {
            return;
        }

        crate::master_error!("{}", message);
    }

    fn success(message: &str) {
        if !constants::DEBUGGING {
            return;
        }

        crate::master_success!("{}", message);
    }
}

impl MasterServer {}
