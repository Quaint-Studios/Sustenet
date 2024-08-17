//! The base of all server types. Takes in clients.
const std = @import("std");
const sustenet = @import("root").sustenet;

const expect = std.testing.expect;
const print = std.debug.print;
const AutoHashMap = std.AutoHashMap;
const Constants = sustenet.utils.Constants;
const Packet = sustenet.network.Packet;
const Utilities = sustenet.utils.Utilities;
const BaseClient = sustenet.transport.BaseClient;
const BaseServer = @This();

pub const ServerType = enum { MasterServer, ClusterServer };

// UDP equivalent is in BaseClient.UdpHandler.socket
// tcp_listener: i32, // Just set as i32 to shutup the compiler

server_type: ServerType,
server_type_name: []const u8,
max_connections: i32,
port: u16,

clients: AutoHashMap(i32, BaseClient),
released_ids: std.ArrayList(i32),

pub const packetHandler = *const fn (from_client: i32, packet: i32) void;

var packetHandlers: ?AutoHashMap(i32, packetHandler) = null;

// onConnection: BaseEvent(comptime i32),
// onDisconnection: BaseEvent(comptime i32),
// onReceived: BaseEvent(comptime []u8),

pub fn new(allocator: std.mem.Allocator, server_type: ServerType, max_connections: i32, port: ?u16) !BaseServer {
    var baseServer = BaseServer{
        .server_type = server_type,
        .server_type_name = serverTypeToString(server_type),
        .max_connections = max_connections,
        .port = port orelse Constants.MASTER_PORT,

        .clients = AutoHashMap(comptime i32, comptime BaseClient).init(allocator),
        .released_ids = std.ArrayList(comptime i32).init(allocator),
    };
    baseServer.initializeData(allocator);

    return baseServer;
}

//#region Connection Functions
pub fn start(self: *BaseServer) !void {
    const allocator = std.heap.page_allocator;
    if (Constants.DEBUGGING) {
        const header = try std.fmt.allocPrint(allocator, "Starting {s} on Port {d}", .{ self.server_type_name, self.port });
        defer allocator.free(header);
        Utilities.consoleHeader(header);
    }

    // TODO: Implement server start

    if (Constants.DEBUGGING) {
        const header = try std.fmt.allocPrint(allocator, "{s} Started (Max connections: {d})", .{ self.server_type_name, self.max_connections });
        defer allocator.free(header);
        Utilities.consoleHeader(header);
    }
}
//#endregion

//#region Data Functions
pub fn initializeData(_: *BaseServer, allocator: std.mem.Allocator) void {
    if (BaseServer.packetHandlers == null) {
        BaseServer.packetHandlers = AutoHashMap(i32, packetHandler).init(allocator);
    }
}
//#endregion

//#region Utillity Functions
pub fn serverTypeToString(server_type: ServerType) []const u8 {
    switch (server_type) {
        ServerType.MasterServer => return "Master Server",
        ServerType.ClusterServer => return "Cluster Server",
    }
}
//#endregion

//#region Memory Functions
pub fn deinit(self: *BaseServer, _: std.mem.Allocator) void {
    // Free clients
    {
        var it = self.clients.iterator();
        while (it.next()) |entry| {
            var client = entry.value_ptr.*;
            client.deinit();
        }
        self.clients.deinit();
    }

    // Free released_ids
    self.released_ids.deinit();

    // Free packetHandlers
    if (BaseServer.packetHandlers != null) {
        BaseServer.packetHandlers.?.deinit();
        BaseServer.packetHandlers = null;
    }
}
//#endregion
