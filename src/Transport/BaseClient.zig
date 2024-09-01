//! The core for all clients. Handles basic functionality like sending
//! and receiving data. Also handles the core for connecting to servers.

const std = @import("std");
const network = @import("network");
const sustenet = @import("root").sustenet;

const testing = std.testing;
const ArrayList = std.ArrayList;
const Packet = sustenet.network.Packet;
const Action = sustenet.events.Action;
const ActionT1 = sustenet.events.ActionT1;
const ActionT2 = sustenet.events.ActionT2;
const Player = sustenet.core.spawning.Player;
const Protocols = sustenet.transport.Protocols;
const BaseClient = @This();

pub const buffer_size = 4096;

id: ?u32 = null,
name: ?[]const u8 = null,

tcp: TcpHandler,
udp: UdpHandler,

received_data: Packet,

on_connected: ArrayList(*Action(void)),
on_disconnected: ArrayList(*ActionT1(Protocols, void)),
on_received: ArrayList(*ActionT2(Protocols, []u8, void)),

player: ?Player = null,

pub fn new(allocator: std.mem.Allocator, id: ?u32) !BaseClient {
    const tcp_socket = try network.Socket.create(.ipv4, .tcp);
    const udp_socket = try network.Socket.create(.ipv4, .udp);

    return BaseClient{
        .id = id,

        .tcp = .{ .socket = tcp_socket },
        .udp = .{ .socket = udp_socket },

        .received_data = Packet.new(allocator),

        .on_connected = ArrayList(*Action(void)).init(allocator),
        .on_disconnected = ArrayList(*ActionT1(Protocols, void)).init(allocator),
        .on_received = ArrayList(*ActionT2(Protocols, []u8, void)).init(allocator),
    };
}

/// Handles events for connecting, receiving, and debugging.
/// Also controls the socket connection.
const TcpHandler = struct {
    const Error = error{
        SocketIsNull,
    };

    socket: ?network.Socket,
    received_buffer: [buffer_size]u8 = undefined,

    //#region Connection Functions
    pub fn connect(self: *TcpHandler, address: network.Address, port: u16) !void {
        if (self.socket == null) return error.SocketIsNull;
        try self.socket.?.connect(.{ .address = address, .port = port });
    }

    pub fn receive(_: *TcpHandler, _: *BaseClient, _: *network.Socket) void {
        // TODO Implmenet
    }

    pub fn deinit(self: *TcpHandler) void {
        if (self.socket != null) {
            self.socket.?.close();
        }
    }
};

const UdpHandler = struct {
    socket: ?network.Socket,

    fn deinit(self: *UdpHandler) void {
        if (self.socket != null) {
            self.socket.?.close();
        }
    }
};

//#region Memory Functions
pub fn deinit(self: *BaseClient) void {
    self.tcp.deinit();
    self.udp.deinit();

    self.received_data.deinit();

    self.on_connected.deinit();
    self.on_disconnected.deinit();
    self.on_received.deinit();
}
//#endregion
