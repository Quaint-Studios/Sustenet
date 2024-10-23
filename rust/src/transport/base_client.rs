use tokio::io::{self, AsyncReadExt, AsyncWriteExt};
use tokio::net::{TcpStream, UdpSocket};

use crate::core::spawning::Player;
use crate::network::Packet;
use crate::transport::Protocols;

pub struct BaseClient {
    id: Option<u32>,
    name: Option<String>,

    tcp: TcpHandler,
    udp: UdpHandler,

    received_data: Packet,

    on_connected: Vec<Box<dyn Fn() + Send + Sync>>,
    on_disconnected: Vec<Box<dyn Fn(Protocols) + Send + Sync>>,
    on_received: Vec<Box<dyn Fn(Protocols, [u8]) + Send + Sync>>,

    player: Option<Player>,
}

impl BaseClient {
    pub const BUFFER_SIZE: usize = 4096;

    pub fn new(id: Option<u32>) -> Self {
        BaseClient {
            id,
            name: None,

            tcp: TcpHandler {
                socket: None,
                receive_buffer: None,
            },
            udp: UdpHandler { socket: None },

            received_data: Packet::new(),

            on_connected: vec![],
            on_disconnected: vec![],
            on_received: vec![],

            player: None,
        }
    }

    pub fn deinit(&mut self) {
        println!("Deinitializing BaseClient.");
        self.tcp.deinit();
        self.udp.deinit();
        self.received_data.deinit();
        self.on_connected.clear();
        self.on_disconnected.clear();
        self.on_received.clear();
    }
}

/// Handles events for connecting, receiving, and debugging.
/// Also controls the socket connection.
pub struct TcpHandler {
    socket: Option<TcpStream>,
    receive_buffer: Option<[u8; BaseClient::BUFFER_SIZE]>,
}

impl TcpHandler {
    pub async fn connect(&mut self, address: &str, port: u16) -> io::Result<()> {
        if let Some(socket) = &self.socket {
            // socket.connect((address, port)).await?; // TODO
        } else {
            return Err(io::Error::new(io::ErrorKind::Other, "Socket is null"));
        }
        Ok(())
    }

    pub fn receive(&mut self, _client: &mut BaseClient) {
        // TODO: Implement
    }

    pub fn deinit(&mut self) {
        if let Some(socket) = self.socket.take() {
            println!("Socket closing now.");
            // socket.shutdown(); // TODO
            println!("Socket closed.");
        } else {
            println!("Socket is null.");
        }
    }
}

pub struct UdpHandler {
    socket: Option<UdpSocket>,
}

impl UdpHandler {
    pub fn deinit(&mut self) {
        if let Some(socket) = self.socket.take() {
            // socket.shutdown().unwrap(); // TODO
        }
    }
}
