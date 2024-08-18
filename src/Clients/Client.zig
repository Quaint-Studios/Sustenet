//! A standard client that connects to a server.

const std = @import("std");
const sustenet = @import("root").sustenet;
const network = @import("network");

const AutoHashMap = std.AutoHashMap;
const BaseEvent = sustenet.events.BaseEvent;
const BaseClient = @import("root").sustenet.transport.BaseClient;

const Client = @This();

pub const ConnectionType = enum { MasterServer, ClusterServer, None };
pub const Connection = struct { ip: network.Address.IPv4, port: u16 };

pub const packetHandler = *const fn (from_client: i32, packet: i32) void;
var packetHandlers: ?AutoHashMap(i32, packetHandler) = null;

active_connection: ConnectionType,
master_connection: ?Connection,
cluster_connection: ?Connection,

on_initialized: BaseEvent, // Init these early
on_cluster_server_list: BaseEvent,

super: BaseClient,

pub fn new(_: ?[]const u8, _: ?u16) Client {
    var client = Client{
        .super = BaseClient.new(-1),

        .active_connection = ConnectionType.None,
        .master_connection = null,
        .cluster_connection = null,
        .on_initialized = BaseEvent{},
        .on_cluster_server_list = BaseEvent{},
    };

    // client.super.on_connected = BaseEvent.init();
    // client.super.on_received = BaseEvent.init();
    // client.super.on_initialized = BaseEvent.init();

    client.initializeClientData();
    return client;
}

pub fn connect(_: *Client) !void {
    // try self.super.connect();
}

pub fn initializeClientData(_: *Client) void {}

//#region Memory Functions
pub fn deinit(self: *Client) void {
    self.super.deinit();
}
//#endregion
