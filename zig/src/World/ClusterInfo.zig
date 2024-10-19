//! TODO RE-EVALULATE
//! TODO DOCUMENTATION

const ClusterInfo = @This();

name: []const u8,
ip: []const u8,
port: u16,

pub fn init(name: []const u8, ip: []const u8, port: u16) ClusterInfo {
    return ClusterInfo{
        .name = name,
        .ip = ip,
        .port = port,
    };
}
