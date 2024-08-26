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

received_data: ?Packet = null,

on_connected: ArrayList(*Action(void)),
on_disconnected: ArrayList(*ActionT1(Protocols, void)),
on_received: ArrayList(*ActionT2(Protocols, []u8, void)),

player: ?Player = null,

pub fn new(allocator: std.mem.Allocator, id: ?u32) BaseClient {
    return BaseClient{
        .id = id,

        .tcp = TcpHandler{
            .socket = null,
            .buffer = null,
        },
        .udp = UdpHandler{
            .socket = null,
            .buffer = null,
        },

        .on_connected = ArrayList(*Action(void)).init(allocator),
        .on_disconnected = ArrayList(*ActionT1(Protocols, void)).init(allocator),
        .on_received = ArrayList(*ActionT2(Protocols, []u8, void)).init(allocator),
    };
}

const TcpHandler = struct {
    socket: ?network.Socket,
    buffer: ?[buffer_size]u8,

    fn deinit(self: *TcpHandler) void {
        if (self.socket != null) {
            self.socket.?.close();
        }
    }
};

const UdpHandler = struct {
    socket: ?network.Socket,
    buffer: ?[buffer_size]u8,

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

    if (self.received_data != null) {
        self.received_data.?.deinit();
    }
}
//#endregion
