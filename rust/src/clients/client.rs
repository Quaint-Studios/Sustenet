use std::{ net::Ipv4Addr, str::FromStr };

use tokio::sync::mpsc::{self, Receiver, Sender};

use crate::events::Event;
use crate::transport::BaseClient;
use crate::utils::constants;
use crate::world::ClusterInfo;

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
    pub active_connection: ConnectionType,
    pub master_connection: Connection,
    pub cluster_connection: Connection,

    /// After a client logs in successfully and gets their username and id back.
    on_initialized: Vec<Box<dyn Fn() + Send>>,
    on_cluster_server_list: Vec<Box<dyn Fn(ClusterInfo) + Send>>,

    base: BaseClient,

    event_receiver: Receiver<Event>,
    event_sender: Sender<Event>,
}

impl Client {
    // TODO: ip string and port
    pub fn new(ip: Option<Ipv4Addr>, port: Option<u16>) -> Client {
        let (event_sender, event_receiver) = mpsc::channel(100); // TODO: Could be a different channel type.
        let base_client = BaseClient::new(None, None, event_sender.clone());

        return Client {
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

            on_initialized: vec![],
            on_cluster_server_list: vec![],

            base: base_client,

            event_receiver,
            event_sender,
        };
    }

    pub fn start(&self) {
        println!("Client started");
    }
}
