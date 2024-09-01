//! A standard client that connects to a server.

const std = @import("std");
const sustenet = @import("root").sustenet;
const network = @import("network");

const ArrayList = std.ArrayList;
const AutoHashMap = std.AutoHashMap;
const Action = sustenet.events.Action;
const ActionT1 = sustenet.events.ActionT1;
const ActionT2 = sustenet.events.ActionT2;
const Packet = sustenet.network.Packet;
const BaseClient = sustenet.transport.BaseClient;
const Protocols = sustenet.transport.Protocols;
const ThreadManager = sustenet.transport.ThreadManager;
const ClusterInfo = sustenet.world.ClusterInfo;
const Constants = sustenet.utils.Constants;

const Client = @This();

pub const ConnectionType = enum { MasterServer, ClusterServer, None };
pub const Connection = struct { ip: network.Address, port: u16 };

const packetHandler = *const fn (packet: Packet) void;
/// A hashmap on how a packet should be handled.
var packetHandlers: ?AutoHashMap(i32, packetHandler) = null;

active_connection: ConnectionType,
master_connection: Connection,
cluster_connection: Connection,

/// After a client logs in successfully and gets their username and id back.
on_initialized: ArrayList(*Action(void)),
on_cluster_server_list: ArrayList(*ActionT1(ClusterInfo, void)),

super: BaseClient,

const Errors = error{
    UnreachableIpAddress,
};

// TODO: ip string and port
pub fn new(allocator: std.mem.Allocator, ip: ?[]const u8, port: ?u16) !Client {
    const base_client = try BaseClient.new(allocator, null);
    const address = network.Address.IPv4.parse(ip orelse "127.0.0.1") catch return error.UnreachableIpAddress;

    var client = Client{
        .super = base_client,

        .active_connection = ConnectionType.None,
        .master_connection = Connection{
            .ip = .{ .ipv4 = address },
            .port = port orelse Constants.MASTER_PORT,
        },
        // TODO: Consider merging master and cluster connection into one to save on memory.
        .cluster_connection = Connection{
            // Placeholder until overridden and used.
            .ip = .{ .ipv4 = network.Address.IPv4.loopback },
            .port = Constants.CLUSTER_PORT,
        },
        .on_initialized = ArrayList(*Action(void)).init(allocator),
        .on_cluster_server_list = ArrayList(*ActionT1(ClusterInfo, void)).init(allocator),
    };

    {
        const action = struct {
            action: Action(void) = .{ .compute = compute },
            client: *Client,
            allocator: std.mem.Allocator,
            fn compute(action: *Action(void)) void {
                const this: *@This() = @alignCast(@fieldParentPtr("action", action));
                this.client.*.super.received_data = Packet.new(this.allocator);
            }
        };
        var callable = action{ .client = &client, .allocator = allocator };
        try client.super.on_connected.append(&callable.action);
    }

    {
        const action = struct {
            action: ActionT2(Protocols, []u8, void) = .{ .compute = compute },
            client: *Client,
            allocator: std.mem.Allocator,
            fn compute(action: *ActionT2(Protocols, []u8, void), protocol: Protocols, data: []u8) void {
                const this: *@This() = @alignCast(@fieldParentPtr("action", action));
                switch (protocol) {
                    .TCP => this.client.*.super.received_data.reset(this.client.*.handleTcpData(this.allocator, data)),
                    .UDP => _ = this.client.handleUdpData(this.allocator, data),
                }
            }
        };
        var callable = action{ .client = &client, .allocator = allocator };
        try client.super.on_received.append(&callable.action);
    }

    {
        // TODO on_initialized
    }

    client.initializeClientData();
    return client;
}

//#region Connection Functions
pub fn login(self: *Client, username: []const u8) void {
    // If the user currently doesn't have a username, let them attempt to login.
    if (username.len > 2) self.ValidateLogin(username);
}

/// Connects to the currently assigned IP and port.
pub fn connect(self: *Client, connectType: ConnectionType) !void {
    self.active_connection = connectType;

    switch (connectType) {
        .MasterServer => try self.super.tcp.connect(self.master_connection.ip, self.master_connection.port),
        .ClusterServer => try self.super.tcp.connect(self.cluster_connection.ip, self.cluster_connection.port),
        .None => {},
    }
}
//#endregion

//#region Request Functions

//#endregion

//#region Data Functions
fn handleTcpData(self: *Client, allocator: std.mem.Allocator, data: []u8) bool {
    var packet_length: u32 = 0;

    self.super.received_data.setBytes(allocator, data) catch |err| {
        std.log.err("Failed to set bytes: {}\n", .{err});
        return true;
    };

    if (self.super.received_data.unreadLength() >= 4) {
        packet_length = self.super.received_data.readUInt(null) catch |err| {
            std.log.err("Failed to read packet length: {}\n", .{err});
            return true;
        };
        if (packet_length == 0) return true;
    }

    while (packet_length > 0 and packet_length <= self.super.received_data.unreadLength()) {
        const packet_bytes = self.super.received_data.readBytes(packet_length, null) catch |err| {
            std.log.err("Failed to read packet bytes: {}\n", .{err});
            return true;
        };

        var threadManager = try ThreadManager.getInstance(allocator);
        {
            const action = struct {
                action: Action(void) = .{ .compute = compute },
                allocator: std.mem.Allocator,
                packet_bytes: []u8,
                fn compute(action: *Action(void)) void {
                    const this: *@This() = @alignCast(@fieldParentPtr("action", action));
                    var packet = Packet.newWithData(this.allocator, this.packet_bytes) catch |err| {
                        std.log.err("Failed to create packet: {}\n", .{err});
                        return;
                    };
                    // NOTE: Null check? Or just wing it becasue it should 100% be set by now.
                    const packet_id = packet.readInt(null) catch |err| {
                        std.log.err("Failed to read packet id: {}\n", .{err});
                        return;
                    };
                    const handler = packetHandlers.?.get(packet_id);
                    if (handler != null) handler.?(packet);
                }
            };
            var callable = action{ .allocator = allocator, .packet_bytes = packet_bytes };
            threadManager.executeOnMainThread(&callable.action);
        }

        packet_length = 0;

        if (self.super.received_data.unreadLength() >= 4) {
            packet_length = self.super.received_data.readUInt(null) catch |err| {
                std.log.err("Failed to read packet length: {}\n", .{err});
                return true;
            };
            if (packet_length == 0) return true;
        }
    }

    if (packet_length <= 1) return true;

    return false;
}

fn handleUdpData(self: *Client, allocator: std.mem.Allocator, data: []u8) bool {
    {
        var new_data: []u8 = undefined;
        {
            var packet = Packet.newWithData(allocator, data) catch |err| {
                std.log.err("Failed to create packet: {}\n", .{err});
                return false;
            };
            const packet_length = packet.readUInt(null) catch |err| {
                std.log.err("Failed to read packet length: {}\n", .{err});
                return false;
            };
            new_data = packet.readBytes(packet_length, null) catch |err| {
                std.log.err("Failed to read packet bytes: {}\n", .{err});
                return false;
            };
        }

        const action = struct {
            action: Action(void) = .{ .compute = compute },
            client: *Client,
            allocator: std.mem.Allocator,
            data: []u8,
            fn compute(action: *Action(void)) void {
                const this: *@This() = @alignCast(@fieldParentPtr("action", action));
                var packet = Packet.newWithData(this.allocator, this.data) catch |err| {
                    std.log.err("Failed to create packet: {}\n", .{err});
                    return;
                };
                const packet_id = packet.readInt(null) catch |err| {
                    std.log.err("Failed to read packet id: {}\n", .{err});
                    return;
                };
                const handler = packetHandlers.?.get(packet_id);
                if (handler != null) handler.?(packet);
            }
        };
        var callable = action{ .client = self, .allocator = allocator, .data = new_data };
        var threadManager = ThreadManager.getInstance(allocator) catch |err| {
            std.log.err("Failed to get thread manager: {}\n", .{err});
            return false;
        };
        threadManager.executeOnMainThread(&callable.action);
    }
    return false;
}
//#endregion

pub fn initializeClientData(_: *Client) void {}

//#region Memory Functions
pub fn deinit(self: *Client) void {
    self.on_cluster_server_list.deinit();
    self.on_initialized.deinit();

    self.super.deinit();
}
//#endregion
