use tokio::io::{ self, AsyncReadExt, AsyncWriteExt };
use tokio::net::{ TcpStream, UdpSocket };
use tokio::sync::mpsc::Sender;

use crate::core::spawning::Player;
use crate::events::Event;
use crate::network::Packet;
use crate::transport::Protocols;

use super::BaseServer;

pub struct BaseClient {
    pub id: Option<u32>,
    pub name: Option<String>,

    pub tcp: TcpHandler,
    pub udp: UdpHandler,

    pub(crate) received_data: Packet,

    pub player: Option<Player>,

    pub event_sender: Sender<Event>,
}

impl BaseClient {
    pub const BUFFER_SIZE: usize = 4096;

    pub fn new(id: Option<u32>, name: Option<String>, event_sender: Sender<Event>) -> Self {
        BaseClient {
            id,
            name: None,

            tcp: TcpHandler {
                socket: None,
                receive_buffer: None,
            },
            udp: UdpHandler { socket: None },

            received_data: Packet::new(),

            player: None,

            event_sender,
        }
    }

    pub fn deinit(&mut self) {
        println!("Deinitializing BaseClient.");
        self.tcp.deinit();
        self.udp.deinit();
        self.received_data.deinit();
    }
}

/// Handles events for connecting, receiving, and debugging.
/// Also controls the socket connection.
pub struct TcpHandler {
    pub(crate) socket: Option<TcpStream>,
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
