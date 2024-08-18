//! The core for all clients. Handles basic functionality like sending
//! and receiving data. Also handles the core for connecting to servers.

const std = @import("std");
const network = @import("network");
const sustenet = @import("root").sustenet;

const net = std.net;
const testing = std.testing;
const Packet = sustenet.network.Packet;
const BaseEvent = sustenet.events.BaseEvent;
const Player = sustenet.core.spawning.Player;
const BaseClient = @This();

pub const buffer_size = 4096;

id: i32 = -1,
name: ?[]const u8 = null,

tcp: TcpHandler,
udp: UdpHandler,

received_data: ?Packet = null,

on_connected: BaseEvent, // Init these early
on_disconnected: BaseEvent,
on_received: BaseEvent,

player: ?Player = null,

pub fn new(id: i32) BaseClient {
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

        .on_connected = BaseEvent{},
        .on_disconnected = BaseEvent{},
        .on_received = BaseEvent{},
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
