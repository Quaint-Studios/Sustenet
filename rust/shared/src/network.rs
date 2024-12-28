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

#[derive(Eq)]
pub struct ClusterInfo {
    pub id: u32,
    pub name: String,
    pub ip: String,
    pub port: u16,
    pub max_connections: u32,
}

impl Ord for ClusterInfo {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        // Define how to compare two ClusterInfo instances
        // For example, if ClusterInfo has a field `id` of type i32:
        self.id.cmp(&other.id)
    }
}

impl PartialOrd for ClusterInfo {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for ClusterInfo {
    fn eq(&self, other: &Self) -> bool {
        // Define when two ClusterInfo instances are equal
        // For example, if ClusterInfo has a field `id` of type i32:
        self.id == other.id
    }
}