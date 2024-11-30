use serde_repr::{ Deserialize_repr, Serialize_repr };

/// Enum to represent all possible events that can be sent to the event loop.
pub enum Event {
    Connection(u32),
    Disconnection(u32),
    ReceivedData(u32, Vec<u8>),
}

#[derive(Debug, Copy, Clone, PartialEq, Serialize_repr, Deserialize_repr)]
#[repr(u8)]
/// What a Master Server sends. What a client may receive.
pub enum MasterServerPackets {
    /// Sends a passphrase that a Cluster Client should decrypt and answer.
    PassphraseRequest,
    /// Turns a regular client into a Cluster Client and gives it a
    /// new ID. Should only be used from a Master Server.
    InitializeCluster,
    /// Sends a list of cluster servers containing their name, ip, and port.
    ClusterServerList,
    /// Send a standard message to the client.
    Message,
    /// Gives the client an ID. Validates the user locally from Master Server.
    InitializeLogin,
    /// Tells a client that the UDP connection is ready.
    UDPReady,
}

#[derive(Debug, Copy, Clone, PartialEq, Serialize_repr, Deserialize_repr)]
#[repr(u8)]
/// What a Cluster Server sends. What a client may receive.
pub enum ClusterServerPackets {
    /// Sends the name of the cluster's key to the Master Server.
    /// If the key doesn't exist, the server will do nothing but
    /// stay silent. If it does exist, it will send a generated
    /// passphrase that's encrypted with AES.
    RequestClusterAuth,
    /// Sends the decrypted key to the Master Server.
    PassphraseResponse,
    /// Sends a list of cluster servers containing their name, ip, and port.
    ClusterServerList,
    /// Send a standard message to the client.
    Message,
    /// Asks the Master Server if they're actual a valid user.
    InitializeLogin,
    /// Tells a client that the UDP connection is ready.
    UDPReady,
}

#[derive(Debug, Copy, Clone, PartialEq, Serialize_repr, Deserialize_repr)]
#[repr(u8)]
/// What a Client sends. What a server may receive.
pub enum ClientPackets {
    /// Requests a list of cluster servers from the server.
    RequestClusterServers = 2,
    /// Sends a message to the server.
    Message,
    /// Sends a username and password to the server.
    Login,
    /// Sends an ID to a server to start a UDP connection.
    StartUDP,
    /// Sends a request to the server to join a cluster.
    JoinCluster,
    /// Sends a request to the server to leave a cluster.
    LeaveCluster,

    MoveTo = 100,

    Error = 200,
}
impl ClientPackets {
    /// Get the packet type from a u8.
    pub fn from_u8(value: u8) -> Self {
        match value {
            2 => ClientPackets::RequestClusterServers,
            3 => ClientPackets::Message,
            4 => ClientPackets::Login,
            5 => ClientPackets::StartUDP,
            6 => ClientPackets::JoinCluster,
            7 => ClientPackets::LeaveCluster,
            100 => ClientPackets::MoveTo,
            _ => ClientPackets::Error,
        }
    }
}
