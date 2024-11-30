use tokio::io::{ self, AsyncReadExt, AsyncWriteExt };
use tokio::net::{ TcpStream, UdpSocket };
use tokio::sync::mpsc::Sender;

use crate::core::spawning::Player;
use crate::events::Event;
use crate::network::Packet;

pub struct BaseClient {
    pub id: Option<u32>,
    pub name: Option<String>,

    pub tcp: TcpHandler,
    pub udp: UdpHandler,

    pub(crate) received_data: Packet,

    pub event_sender: Sender<Event>,

    pub player: Option<Player>,
}

impl BaseClient {
    pub const BUFFER_SIZE: usize = 4096;

    pub fn new(id: Option<u32>, name: Option<String>, event_sender: Sender<Event>) -> Self {
        BaseClient {
            id,
            name,

            tcp: TcpHandler {
                stream: None,
                receive_buffer: None,
            },
            udp: UdpHandler { socket: None },

            received_data: Packet::new(),

            event_sender,
            
            player: None,
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
    pub(crate) stream: Option<TcpStream>,
    receive_buffer: Option<[u8; BaseClient::BUFFER_SIZE]>,
}

impl TcpHandler {
    pub async fn connect(&mut self, address: &str, port: u16) -> io::Result<()> {
        if let Some(socket) = &self.stream {
            // socket.connect((address, port)).await?; // TODO
        } else {
            return Err(io::Error::new(io::ErrorKind::Other, "Socket is null"));
        }
        Ok(())
    }

    /// Used for servers that create local records of clients.
    /// It will wipe any existing connections and start a new one.
    /// 
    /// Should disconnect the client if it produces an error.
    /// 
    /// * `socket` - The socket to replace the current socket with.
    pub async fn receive(&mut self, stream: TcpStream) -> io::Result<()> {
        if self.stream.is_some() {
            self.stream.as_mut().unwrap().shutdown().await?;
        }

        self.stream = Some(stream);

        if self.receive_buffer.is_none() {
            self.receive_buffer = Some([0; BaseClient::BUFFER_SIZE]);
        }

        Ok(())
    }

    pub fn deinit(&mut self) {
        if let Some(socket) = self.stream.take() {
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
