//! The Master Server keeps track of all Cluster Servers. It also allocates
//! connecting users to Cluster Servers automatically, or allows the users
//! to manually select one.

const std = @import("std");
const sustenet = @import("root").sustenet;
const network = @import("network");
const world = sustenet.world;

const ArrayList = std.ArrayList;
const AutoHashMap = std.AutoHashMap;
const BaseServer = @import("root").sustenet.transport.BaseServer;

const MasterServer = @This();

/// A list of clients that have been registered as cluster clients.
cluster_ids: ArrayList(i32),

cluster_info: AutoHashMap(i32, world.ClusterInfo),

super: BaseServer,

pub fn new(allocator: std.mem.Allocator, max_connections: ?i32, port: ?u16) !MasterServer {
    // RSAManager.loadPubKeys();
    // AESManager.loadKeys();

    var master_server = MasterServer{
        .super = try BaseServer.new(
            allocator,
            BaseServer.ServerType.MasterServer,
            max_connections orelse 0,
            port orelse 6256,
        ),

        .cluster_ids = ArrayList(i32).init(allocator),
        .cluster_info = AutoHashMap(i32, world.ClusterInfo).init(allocator),
    };

    MasterServer.initializeData(allocator);

    try master_server.super.start(allocator);

    return master_server;
}

fn initializeData(allocator: std.mem.Allocator) void {
    if (BaseServer.packetHandlers == null) {
        BaseServer.packetHandlers = AutoHashMap(i32, BaseServer.packetHandler).init(allocator);
    }
}

//#region Memory Functions
pub fn deinit(self: *MasterServer) void {
    self.cluster_ids.deinit();
    self.cluster_info.deinit();
    self.super.deinit();
}
//#endregion
