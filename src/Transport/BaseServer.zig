//! The base of all server types. Takes in clients.
const std = @import("std");
const sustenet = @import("root").sustenet;
const network = @import("network");

const expect = std.testing.expect;
const print = std.debug.print;
const AutoHashMap = std.AutoHashMap;
const Constants = sustenet.utils.Constants;
const Packet = sustenet.network.Packet;
const Utilities = sustenet.utils.Utilities;
const EventT1 = sustenet.events.BaseEvent.EventT1;
const BaseClient = sustenet.transport.BaseClient;
const BaseServer = @This();

pub const ServerType = enum { MasterServer, ClusterServer };

pub const packetHandler = *const fn (from_client: i32, packet: i32) void;
pub var packetHandlers: ?AutoHashMap(i32, packetHandler) = null;

tcp_listener: network.Socket,
// UDP equivalent is in BaseClient.UdpHandler.socket

server_type: ServerType,
server_type_name: []const u8,
max_connections: i32,
port: u16,

clients: AutoHashMap(i32, BaseClient),
released_ids: std.ArrayList(i32),

onConnection: EventT1(i32),
onDisconnection: EventT1(i32),
onReceived: EventT1([]u8),

pub fn new(allocator: std.mem.Allocator, server_type: ServerType, max_connections: i32, port: ?u16) !BaseServer {
    return BaseServer{
        .tcp_listener = try network.Socket.create(.ipv4, .tcp),

        .server_type = server_type,
        .server_type_name = serverTypeToString(server_type),
        .max_connections = max_connections,
        .port = port orelse Constants.MASTER_PORT,

        .clients = AutoHashMap(comptime i32, comptime BaseClient).init(allocator),
        .released_ids = std.ArrayList(comptime i32).init(allocator),

        .onConnection = EventT1(i32).init(allocator),
        .onDisconnection = EventT1(i32).init(allocator),
        .onReceived = EventT1([]u8).init(allocator),
    };
}

//#region Connection Functions
pub fn start(self: *BaseServer, allocator: std.mem.Allocator) !void {
    if (Constants.DEBUGGING) {
        const func = comptime struct {
            pub fn exec(id: i32) void {
                self.debugServer(self.server_type_name, "Client#" ++ id ++ " has connected.");
            }
        };
        self.onConnection.add(&func.exec);

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

//#region Utillity Functions
pub fn serverTypeToString(server_type: ServerType) []const u8 {
    switch (server_type) {
        ServerType.MasterServer => return "Master Server",
        ServerType.ClusterServer => return "Cluster Server",
    }
}
//#endregion

//#region Data Functions

//#endregion

//#region Memory Functions
pub fn deinit(self: *BaseServer) void {
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

pub fn debugServer(serverTypeName: []const u8, msg: []const u8) void {
    Utilities.printMsg("({s}) {s}", .{ serverTypeName, msg });
}
