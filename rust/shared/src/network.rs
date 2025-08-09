use std::net::IpAddr;

pub enum Protocols {
    TCP,
    UDP,
}

/// Enum to represent all possible events that can be sent to the event loop.
pub enum Event {
    Connection(u32),
    Disconnection(u32),
    ReceivedData(u32, Vec<u8>),
}

/// Used to store cluster information that we can reuse.
pub struct ClusterInfo {
    pub id: u64,
    pub name: String,
	pub ip: IpAddr,
    pub port: u16,
    pub max_connections: u32,
    pub start_time: u32,
}