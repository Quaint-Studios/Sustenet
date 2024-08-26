//! A standard client that connects to a server.

const std = @import("std");
const sustenet = @import("root").sustenet;
const network = @import("network");

const ArrayList = std.ArrayList;
const AutoHashMap = std.AutoHashMap;
const Action = sustenet.events.Action;
const ActionT1 = sustenet.events.ActionT1;
const BaseClient = @import("root").sustenet.transport.BaseClient;
const ClusterInfo = sustenet.world.ClusterInfo;

const Client = @This();

pub const ConnectionType = enum { MasterServer, ClusterServer, None };
pub const Connection = struct { ip: network.Address.IPv4, port: u16 };

pub const packetHandler = *const fn (from_client: i32, packet: i32) void;
var packetHandlers: ?AutoHashMap(i32, packetHandler) = null;

active_connection: ConnectionType,
master_connection: ?Connection,
cluster_connection: ?Connection,

on_initialized: ArrayList(*Action(void)),
on_cluster_server_list: ArrayList(*ActionT1(ClusterInfo, void)),

super: BaseClient,

// TODO: ip string and port
pub fn new(allocator: std.mem.Allocator, _: ?[]const u8, _: ?u16) Client {
    var client = Client{
        .super = BaseClient.new(allocator, null),

        .active_connection = ConnectionType.None,
        .master_connection = null,
        .cluster_connection = null,
        .on_initialized = ArrayList(*Action(void)).init(allocator),
        .on_cluster_server_list = ArrayList(*ActionT1(ClusterInfo, void)).init(allocator),
    };

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
