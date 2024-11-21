use tokio::net::TcpStream;

use crate::clients::ServerClient;

use super::Protocols;

pub(crate) trait ServerCore<E> {
    type Server;
    async fn new(max_connections: Option<u32>, port: Option<u16>) -> Result<Self::Server, E>;
    async fn start(&mut self);
    async fn listen(&self) -> Result<(), E>;
}

pub(crate) trait ServerConnection<E> {
    async fn on_tcp_connection(&self, stream: TcpStream);
    fn on_udp_received(&self);
    async fn add_client(&self, stream: TcpStream) -> Result<(), E>;
    fn disconnect_client(&self, client_id: u32);
    async fn clear_client(&self, client_id: u32);
}

pub(crate) trait ServerData {
    fn handle_tcp_data(&self, client: &mut ServerClient, data: &[u8]);
}

pub(crate) trait ServerEvents {
    fn process_events(&self);
    fn on_connection(&self, id: u32);
    fn on_disconnection(&self, id: u32);
    fn on_received_data(&self, id: u32, data: &[u8]);
    fn on_client_connected(&self, id: u32);
    fn on_client_disconnected(&self, id: u32, protocol: Protocols);
    fn on_client_received_data(&self, id: u32, protocol: Protocols, data: &[u8]);
}

pub(crate) trait ServerLogging {
    fn debug(message: String);
    fn info(message: String);
    fn warning(message: String);
    fn error(message: String);
    fn success(message: String);
}

// use std::collections::BTreeSet;
// use std::sync::Arc;

// use dashmap::DashMap;
// use tokio::io::{ AsyncBufReadExt, AsyncWriteExt, BufReader };
// use tokio::net::{ TcpListener, TcpStream };
// use tokio::sync::{ broadcast, mpsc, Mutex };

// use crate::events::Event;
// use crate::network::Packet;
// use crate::utils::constants;
// use crate::utils::constants::DEFAULT_IP;

// use super::{ BaseClient, Protocols, ThreadManager };

// #[derive(Clone, Copy, Debug)]
// pub enum ServerType {
//     ClusterServer,
//     MasterServer,
// }
// impl From<ServerType> for String {
//     fn from(server_type: ServerType) -> String {
//         match server_type {
//             ServerType::ClusterServer => "Cluster Server".to_string(),
//             ServerType::MasterServer => "Master Server".to_string(),
//         }
//     }
// }

// // List of possible errors
// #[derive(Debug)]
// pub enum BaseServerError {
//     ClientMissing,
//     ClientMissingID,
//     TCPError,
//     AddClientError,
// }
// impl From<BaseServerError> for String {
//     fn from(error: BaseServerError) -> String {
//         match error {
//             BaseServerError::ClientMissing => "Client is missing".to_string(),
//             BaseServerError::ClientMissingID => "Client is missing an ID".to_string(),
//             BaseServerError::TCPError => "Failed to read TCP data".to_string(),
//             BaseServerError::AddClientError => "Failed to add client".to_string(),
//         }
//     }
// }

// pub(crate) type PacketHandler = fn(from_client: u32, packet: u32);

// trait BaseServerCore where Self: {
//     fn start(&mut self);
//     fn listen(&mut self);
// }

// pub(crate) trait BaseServerConnection {
//     fn on_tcp_connection(&mut self, client: &TcpStream);
//     fn on_udp_received(&mut self);
//     fn add_client(&mut self, stream: &TcpStream);
//     fn disconnect_client(&mut self, client_id: u32);
//     fn clear_client(&mut self, client_id: u32);
// }

// pub(crate) trait BaseServerData {
//     fn handle_tcp_data(&self, client: &mut BaseClient, data: &[u8]);
// }

// pub(crate) trait BaseServerEvents {
//     fn process_events(&mut self);
//     fn on_connection(&mut self, id: u32);
//     fn on_disconnection(&mut self, id: u32);
//     fn on_received_data(&mut self, id: u32, data: &[u8]);
//     fn on_client_connected(&mut self, id: u32);
//     fn on_client_disconnected(&mut self, id: u32, protocol: Protocols);
//     fn on_client_received_data(&mut self, id: u32, protocol: Protocols, data: &[u8]);
// }

// pub(crate) trait BaseServerLogging {
//     fn debug(message: String);
//     fn success(message: String);
// }

// //
// // fn start(&mut self);
// // fn listen(&mut self);
// // fn on_tcp_connection(&mut self, client: &TcpStream);
// // fn on_udp_received(&mut self);
// // fn add_client(&mut self, stream: &TcpStream);
// // fn disconnect_client(&mut self, client_id: u32);
// // fn clear_client(&mut self, client_id: u32);
// // fn handle_tcp_data(&self, client: &mut BaseClient, data: &[u8]);
// // fn process_events(&mut self);
// // fn on_connection(&mut self, id: u32);
// // fn on_disconnection(&mut self, id: u32);
// // fn on_received_data(&mut self, id: u32, data: &[u8]);
// // fn on_client_connected(&mut self, id: u32);
// // fn on_client_disconnected(&mut self, id: u32, protocol: Protocols);
// // fn on_client_received_data(&mut self, id: u32, protocol: Protocols, data: &[u8]);

// /// The base of all server types. Takes in clients.
// pub struct BaseServer {
//     pub is_running: bool,

//     // Packet Handlers
//     pub packet_handlers: DashMap<u32, PacketHandler>,

//     // Network
//     tcp_listener: TcpListener,
//     // UDP equivalent is in BaseClient.UdpHandler.socket

//     // Server Info
//     pub server_type: ServerType,
//     pub max_connections: u32,
//     pub port: u16,

//     // Data
//     pub clients: DashMap<u32, BaseClient>,
//     pub released_ids: Arc<Mutex<BTreeSet<u32>>>,

//     // Events
//     pub event_receiver: mpsc::Receiver<Event>,
//     pub event_sender: mpsc::Sender<Event>,
// }

// impl BaseServer {
//     /// Creates a new BaseServer. Defaults the port to `utils::constants::MASTER_PORT`.
//     pub async fn new(
//         server_type: ServerType,
//         max_connections: Option<u32>,
//         port: Option<u16>
//     ) -> Result<Self, BaseServerError> {
//         let max_connections = max_connections.unwrap_or(0);
//         let port = port.unwrap_or(constants::MASTER_PORT);

//         let (event_sender, event_receiver) = mpsc::channel(100);

//         Ok(BaseServer {
//             is_running: false,

//             packet_handlers: DashMap::new(),

//             tcp_listener: TcpListener::bind(format!("{DEFAULT_IP}:{port}")).await.unwrap(),

//             server_type,
//             max_connections,
//             port,

//             clients: DashMap::new(),
//             released_ids: Arc::new(Mutex::new(BTreeSet::new())),

//             event_receiver,
//             event_sender,
//         })
//     }

//     // region: Connection Functions
//     /// Starts a server.
//     pub async fn start(&mut self) {
//         self.is_running = true;

//         {
//             let max_connections_str = match self.max_connections {
//                 0 => "unlimited".to_string(),
//                 1 => "max connection".to_string(),
//                 _ => format!("max connections: {}", self.max_connections),
//             };

//             BaseServer::debug(
//                 self.server_type,
//                 format!(
//                     "Starting the {:?} on port {} with {} max connections...",
//                     self.server_type,
//                     self.port,
//                     max_connections_str
//                 )
//             );
//         }

//         self.listen().await;
//     }

//     #[inline(always)]
//     async fn listen(&mut self) {
//         self.process_events();

//         let (tx, _rx) = broadcast::channel(10);

//         BaseServer::success(self.server_type, "Now listening for connections.".to_string());

//         while self.is_running {
//             let (mut socket, addr) = self.tcp_listener.accept().await.unwrap();

//             BaseServer::debug(self.server_type, format!("Accepted connection from {:?}", addr));

//             self.on_tcp_connection(&socket).await.ok().expect("Failed to handle TCP connection.");

//             let tx = tx.clone();
//             let mut rx = tx.subscribe();

//             tokio::spawn(async move {
//                 let (reader, mut writer) = socket.split();

//                 let mut reader = BufReader::new(reader);
//                 let mut line = String::new();

//                 loop {
//                     tokio::select! {
//                         result = reader.read_line(&mut line) => {
//                             // Break if the line is empty.
//                             if result.unwrap() == 0 {
//                                 break;
//                             }

//                             // Send the line to the channel.
//                             tx.send((line.clone(), addr)).unwrap();
//                             line.clear();
//                         }
//                         result = rx.recv() => {
//                             // Write the message to the writer.
//                             let (msg, msg_addr) = result.unwrap();

//                             if addr != msg_addr {
//                                 writer.write_all(&msg.as_bytes()).await.unwrap();
//                             }
//                         }
//                     }
//                 }
//             });
//         }
//     }

//     async fn on_tcp_connection(&mut self, client: &TcpStream) -> Result<(), BaseServerError> {
//         self.add_client(client).await.ok().expect("Failed to add client.");

//         Ok(())
//     }

//     async fn on_udp_received(&mut self) -> Result<(), BaseServerError> {
//         todo!();
//     }

//     /// Adds a client to the server, generating or reusing an ID for them.
//     async fn add_client(&mut self, stream: &TcpStream) -> Result<(), BaseServerError> {
//         let mut id: Option<u32> = None;

//         {
//             let mut released_ids = self.released_ids.lock().await;
//             if self.max_connections == 0 || self.clients.len() < (self.max_connections as usize) {
//                 // Loop until an ID is found.
//                 while id.is_none() {
//                     // If there are released IDs, use one.
//                     if released_ids.len() > 0 {
//                         id = released_ids.pop_last();
//                         if !self.clients.contains_key(&id.unwrap()) {
//                             self.clients.insert(id.unwrap(), BaseClient::new(id, None, self.event_sender.clone()));
//                             // Reserve this spot.
//                         } else {
//                             id = None;
//                         }
//                         continue;
//                     } else {
//                         // Assign the next highest client ID if there's no released IDs.
//                         id = Some(self.clients.len() as u32);

//                         if !self.clients.contains_key(&id.unwrap()) {
//                             self.clients.insert(id.unwrap(), BaseClient::new(id, None, self.event_sender.clone()));
//                             // Reserve this spot here too.
//                         } else {
//                             id = None;
//                             continue;
//                         }
//                     }
//                 }

//                 {
//                     // Check if the client was added successfully.
//                     let mut client = match self.clients.get_mut(&id.unwrap()) {
//                         Some(client) => client,
//                         None => {
//                             return Err(BaseServerError::ClientMissing);
//                         }
//                     };
//                     client.received_data = Packet::new();
//                 }
//             }
//         }

//         // If the id was never reset.
//         // That means that a client may still exist.
//         if id != None {
//             self.disconnect_client(id.unwrap()).await;
//         }

//         Ok(())
//     }

//     async fn disconnect_client(&mut self, client_id: u32) {
//         self.clear_client(client_id).await;
//     }

//     async fn clear_client(&mut self, client_id: u32) {
//         self.clients.remove(&client_id);

//         if self.clients.len() == 0 {
//             self.released_ids.lock().await.clear();
//         } else if self.clients.len() > (client_id as usize) {
//             // If the client is not the last one.
//             self.released_ids.lock().await.insert(client_id);
//         }

//         BaseServer::debug(self.server_type, format!("Disconnected Client#{client_id}"));
//     }
//     // endregion

//     // region: Data Functions
//     async fn handle_tcp_data(
//         &self,
//         client: &mut BaseClient,
//         data: &[u8]
//     ) -> Result<bool, BaseServerError> {
//         if client.id == None {
//             return Err(BaseServerError::ClientMissingID);
//         }

//         let mut packet_length = 0;

//         client.received_data.set_bytes(data.to_vec());

//         if client.received_data.unread_length() >= 4 {
//             packet_length = match client.received_data.read_uint(None) {
//                 Ok(length) => length,
//                 Err(_) => {
//                     return Err(BaseServerError::TCPError);
//                 }
//             };
//             if packet_length <= 0 {
//                 return Ok(true);
//             }
//         }

//         while
//             packet_length > 0 &&
//             packet_length <= client.received_data.unread_length().try_into().unwrap()
//         {
//             let packet_bytes = match
//                 client.received_data.read_bytes((packet_length - 4).try_into().unwrap(), None)
//             {
//                 Ok(bytes) => bytes,
//                 Err(_) => {
//                     return Err(BaseServerError::TCPError);
//                 }
//             };

//             // let thread_manager = ThreadManager::get_instance();

//             // TODO: Fix lifetime.
//             // thread_manager.execute_on_side_thread(Box::new(move ||
//             {
//                 if client.id == None {
//                     return Ok(false);
//                 }

//                 let mut packet = Packet::new_with_data(packet_bytes);
//                 let packet_id = match packet.read_uint(None) {
//                     Ok(id) => id,
//                     Err(_) => {
//                         return Err(BaseServerError::TCPError);
//                     }
//                 };

//                 if !self.packet_handlers.contains_key(&packet_id) {
//                     return Err(BaseServerError::TCPError);
//                 }

//                 // Call the packet handler.
//                 self.packet_handlers.get(&packet_id).unwrap()(client.id.unwrap(), packet_id);
//             }
//             // ));

//             packet_length = 0;

//             if client.received_data.unread_length() >= 4 {
//                 packet_length = match client.received_data.read_uint(None) {
//                     Ok(length) => length,
//                     Err(_) => {
//                         return Err(BaseServerError::TCPError);
//                     }
//                 };
//                 if packet_length <= 0 {
//                     return Ok(true);
//                 }
//             }
//         }

//         if packet_length <= 1 {
//             return Ok(true);
//         }

//         Ok(false)
//     }
//     // endregion

//     // region: Event Functions
//     pub async fn process_events(&mut self) {
//         while let Some(event) = self.event_receiver.recv().await {
//             match event {
//                 Event::Connection(id) => self.on_connection(id).await,
//                 Event::Disconnection(id) => self.on_disconnection(id).await,
//                 Event::ReceivedData(id, data) => self.on_received_data(id, &data).await,
//             }
//         }
//     }

//     pub async fn on_connection(&mut self, id: u32) {
//         BaseServer::debug(self.server_type, format!("Client#{} connected", id));
//     }

//     pub async fn on_disconnection(&mut self, id: u32) {
//         BaseServer::debug(self.server_type, format!("Client#{} disconnected", id));
//     }

//     pub async fn on_received_data(&mut self, _id: u32, _data: &[u8]) {
//         BaseServer::debug(self.server_type, format!("{:?} received data", self.server_type));
//     }

//     pub async fn on_client_connected(&mut self, id: u32) {
//         BaseServer::debug(self.server_type, format!("Client#{} connected", id));
//     }

//     pub async fn on_client_disconnected(&mut self, id: u32, protocol: Protocols) {
//         BaseServer::debug(self.server_type, format!("Client#{} ({:?}) disconnected", id, protocol));

//         self.disconnect_client(id).await;
//     }

//     pub async fn on_client_received_data(&mut self, id: u32, protocol: Protocols, data: &[u8]) {
//         BaseServer::debug(self.server_type, format!("Client#{} received data", id));

//         match protocol {
//             Protocols::TCP => {
//                 // TODO: Instead of unwrapping, be mindful of thread safety and handle the error.
//                 // let client = self.clients.get_mut(&id).unwrap();

//                 let mut client_ref = match self.clients.get_mut(&id) {
//                     Some(client) => client,
//                     None => {
//                         BaseServer::debug(self.server_type, "Client not found".to_string());
//                         return;
//                     }
//                 };
//                 let client = client_ref.value_mut();

//                 let full_reset = match self.handle_tcp_data(client, data).await {
//                     Ok(reset) => reset,
//                     Err(e) => {
//                         BaseServer::debug(
//                             self.server_type,
//                             format!("Error handling TCP data: {:?}", e)
//                         );
//                         return;
//                     }
//                 };

//                 client.received_data.reset(full_reset);
//             }
//             Protocols::UDP => {
//                 // Extra things to do goes here.
//                 return;
//             }
//         }
//     }
//     // endregion

//     pub(crate) fn debug(server_type: ServerType, message: String) {
//         if !constants::DEBUGGING {
//             return;
//         }

//         match server_type {
//             ServerType::ClusterServer => crate::cluster_debug!("{}", message),
//             ServerType::MasterServer => crate::master_debug!("{}", message),
//         }
//     }

//     pub(crate) fn success(server_type: ServerType, message: String) {
//         match server_type {
//             ServerType::ClusterServer => crate::cluster_success!("{}", message),
//             ServerType::MasterServer => crate::master_success!("{}", message),
//         }
//     }
// }

// //
// // Test:
// // Does released_ids get populated with the correct values?
// // Does it get smaller when clients are added?
// // Does it get bigger when clients are removed anywhere not at the end?
// // Does it remove elements when the client list shrinks to the next threshold?
// // (i.e. 10, 20, 30, 40, 50 then if there are 51 clients and a person joins, then the id 50 is used.)
// // (but if 51, 50, 49...39 then released_ids should delete anything higher than the largest index)
// // std::collections::BTreeSet could make this simpler by just checking if the last element is larger than the size of the list.
