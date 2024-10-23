//! TODO RE-EVALULATE
//! TODO DOCUMENTATION

use std::net::Ipv4Addr;

pub struct ClusterInfo {
    name: String,
    ip: Ipv4Addr,
    port: u16,
}

impl ClusterInfo {
    pub fn new(name: String, ip: Ipv4Addr, port: u16) -> Self {
        ClusterInfo { name, ip, port }
    }
}
