//! Handles connections to a server and sending messages.
use sustenet_shared::logging::{ LogType, Logger };

use std::sync::LazyLock;

use bytes::Bytes;
use tokio::io;
use tokio::sync::{ broadcast, mpsc };

/// Global logger for the client module.
pub static LOGGER: LazyLock<Logger> = LazyLock::new(|| Logger::new(LogType::Client));

#[derive(Debug, Clone)]
pub struct ClusterInfo {
    pub name: String,
    pub ip: String,
    pub port: u16,
    pub max_connections: u32,
}

/// Events emitted by the client to notify listeners (such as a game engine).
#[derive(Debug, Clone)]
pub enum ClientEvent {
    Connected,
    Disconnected,
    MessageSent(Bytes),
    MessageReceived(Bytes),
    Error(String),
}

/// Handles connection to a master or cluster server, and provides async channels for interaction.
pub struct Client {
    /// Sends messages to the server.
    sender: mpsc::Sender<Bytes>,
    /// Receives messages from the server.
    receiver: mpsc::Receiver<Bytes>,
    /// Sends events to listeners.
    event_tx: broadcast::Sender<ClientEvent>,
    /// Receives events about connection state and activity.
    event_rx: broadcast::Receiver<ClientEvent>,
    /// Cluster servers this client knows about.
    pub cluster_servers: Vec<ClusterInfo>,
}

impl Client {
    /// Attempts to connect to a server at the specified address and port and returns a `ClientHandle`.
    pub async fn connect(address: &str, port: u16) -> io::Result<Self> {
        LOGGER.info(&format!("Connecting to {}:{}...", address, port)).await;

        let (sender, receiver) = mpsc::channel::<Bytes>(64);
        let (event_tx, event_rx) = broadcast::channel::<ClientEvent>(16);

        // Notify connected immediately.
        let _ = event_tx.send(ClientEvent::Connected);

        Ok(Client {
            sender,
            receiver,
            event_tx,
            event_rx,
            cluster_servers: Vec::new(),
        })
    }

    // region: Connection Management
    /// Sends a message to the server.
    pub async fn send_message(&self, msg: Bytes) -> Result<(), mpsc::error::SendError<Bytes>> {
        self.sender.send(msg.clone()).await?;
        // In a full implementation, you would also notify event listeners here:
        // (In real code, you may not want to clone and send every message event.)
        let _ = self.event_tx.send(ClientEvent::MessageSent(msg));
        Ok(())
    }

    /// Receives the next message from the server.
    pub async fn receive_message(&mut self) -> Option<Bytes> {
        self.receiver.recv().await
    }
    // endregion: Connection Management

    // region: Event Management
    /// Returns a cloneable event receiver for status updates.
    pub fn event_receiver(&self) -> broadcast::Receiver<ClientEvent> {
        self.event_rx.resubscribe()
    }

    /// Returns the next event from the event receiver.
    pub async fn next_event(&mut self) -> Option<ClientEvent> {
        let event = self.event_rx.recv().await;
        match event {
            Ok(event) => Some(event),
            Err(_) => None,
        }
    }
    // endregion: Event Management

    // region: Cluster Server Management
    pub fn get_cluster_servers(&self) -> &[ClusterInfo] {
        &self.cluster_servers
    }

    pub fn add_cluster_server(&mut self, server: ClusterInfo) {
        self.cluster_servers.push(server);
    }

    pub fn add_cluster_servers(&mut self, servers: Vec<ClusterInfo>) {
        self.cluster_servers.extend(servers);
    }

    pub fn remove_cluster_server(&mut self, server: &ClusterInfo) {
        todo!("Remove cluster server from the list.");
    }

    pub fn clear_cluster_servers(&mut self) {
        self.cluster_servers.clear();
    }
    // endregion: Cluster Server Management
}
