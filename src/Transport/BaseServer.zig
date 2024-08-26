//! The base of all server types. Takes in clients.
const std = @import("std");
const sustenet = @import("root").sustenet;
const network = @import("network");

const expect = std.testing.expect;
const print = std.debug.print;
const RwLock = std.Thread.RwLock;
const ArrayList = std.ArrayList;
const Action = sustenet.events.Action;
const ActionT1 = sustenet.events.ActionT1;
const ActionT2 = sustenet.events.ActionT2;
const ThreadManager = sustenet.transport.ThreadManager;
const AutoHashMap = std.AutoHashMap;
const Constants = sustenet.utils.Constants;
const TcpListener = sustenet.network.TcpListener;
const Packet = sustenet.network.Packet;
const Protocols = sustenet.transport.Protocols;
const Utilities = sustenet.utils.Utilities;
const BaseClient = sustenet.transport.BaseClient;
const BaseServer = @This();

pub const ServerType = enum { MasterServer, ClusterServer };

const BaseServerError = error{
    ClientMissing,
    PacketHandlersNull,
    PacketHandlerIdNotFound,
    TcpClientEndpointNull,
};

// Packet Handlers
pub const packetHandler = *const fn (from_client: i32, packet: i32) void;
pub var packetHandlers: ?AutoHashMap(i32, packetHandler) = null;

// Network
tcp_listener: TcpListener,
// UDP equivalent is in BaseClient.UdpHandler.socket

// Server Info
server_type: ServerType,
server_type_name: []const u8,
max_connections: i32,
port: u16,

// Data
clients: AutoHashMap(u32, BaseClient),
released_ids: std.ArrayList(u32),

// Events
onConnection: ArrayList(*ActionT1(u32, void)),
onDisconnection: ArrayList(*ActionT1(u32, void)),
onReceived: ArrayList(*ActionT1([]u8, void)),

// Locks
clients_lock: RwLock = .{},
released_ids_lock: RwLock = .{},

/// Creates a new BaseServer. Defaults the port to Constants.MASTER_PORT.
pub fn new(allocator: std.mem.Allocator, server_type: ServerType, max_connections: i32, port: ?u16) !BaseServer {
    return BaseServer{
        .tcp_listener = try TcpListener.new(.{ .ipv4 = network.Address.IPv4.any }, port orelse Constants.MASTER_PORT),

        .server_type = server_type,
        .server_type_name = serverTypeToString(server_type),
        .max_connections = max_connections,
        .port = port orelse Constants.MASTER_PORT,

        .clients = AutoHashMap(comptime u32, comptime BaseClient).init(allocator),
        .released_ids = std.ArrayList(comptime u32).init(allocator),

        .onConnection = ArrayList(*ActionT1(u32, void)).init(allocator),
        .onDisconnection = ArrayList(*ActionT1(u32, void)).init(allocator),
        .onReceived = ArrayList(*ActionT1([]u8, void)).init(allocator),
    };
}

//#region Connection Functions
pub fn start(self: *BaseServer, allocator: std.mem.Allocator) !void {
    if (Constants.DEBUGGING) {
        {
            const action = struct {
                action: ActionT1(u32, void) = .{ .compute = compute },
                server: *BaseServer,
                fn compute(action: *ActionT1(u32, void), id: u32) void {
                    const this: *@This() = @alignCast(@fieldParentPtr("action", action));
                    BaseServer.debugServer(this.server.*.server_type_name, "Client#{} has connected.", .{id});
                }
            };
            var callable = action{ .server = self };
            try self.onConnection.append(&callable.action);
            for (self.onConnection.items) |item| {
                item.compute(item, 444);
            }
        }

        Utilities.consoleHeader("Starting {s} on Port {d}", .{ self.server_type_name, self.port });
    }

    // Threaded wrapper for network.Socket.accept().
    //
    // Waits until a new TCP client connects to this socket
    // and accepts the incoming TCP connection. This function
    // is only allowed for a bound TCP socket. `listen()` must
    // have been called before!
    {
        self.tcp_listener.thread = try std.Thread.spawn(.{}, onTcpConnectCallback, .{ self, allocator });
        try self.tcp_listener.thread.?.setName("TcpListener");
        self.tcp_listener.thread.?.detach();
    }

    if (Constants.DEBUGGING) {
        Utilities.consoleHeader("{s} Started (Max connections: {d})", .{ self.server_type_name, self.max_connections });
    }
}

/// Handles new TCP connections.
fn onTcpConnectCallback(self: *BaseServer, allocator: std.mem.Allocator) void {
    self.tcp_listener.socket.listen() catch |err| {
        debugServer(self.server_type_name, "Error listening on TCP socket: {}\n", .{err});
        return;
    };

    const client_socket = self.tcp_listener.socket.accept() catch |err| {
        debugServer(self.server_type_name, "Error accepting TCP client: {}\n", .{err});
        return;
    };

    {
        const action = struct {
            action: Action(void) = .{ .compute = compute },
            server: *BaseServer,
            allocator: std.mem.Allocator,
            client_socket: network.Socket,
            fn compute(action: *Action(void)) void {
                const this: *@This() = @alignCast(@fieldParentPtr("action", action));
                this.server.addClient(this.allocator, this.client_socket) catch |err| {
                    debugServer(this.server.server_type_name, "Error adding client: {}\n", .{err});
                };
            }
        };
        const callable = action{
            .server = self,
            .allocator = allocator,
            .client_socket = client_socket,
        };

        var thread_manager = try ThreadManager.getInstance(allocator);
        thread_manager.executeOnMainThread(&callable.action);
    }
}

fn addClient(self: *BaseServer, allocator: std.mem.Allocator, tcp_client: network.Socket) anyerror!void {
    var id: ?u32 = null;
    errdefer {
        // If the id was never reset.
        // That means that a client may still exist.
        // Cleanup.
        if (id != null) {
            self.disconnectClient(id.?);
        }
    }

    if (self.max_connections == 0 or self.clients.count() < self.max_connections) {
        self.clients_lock.lock();
        {
            defer self.clients_lock.unlock();

            self.released_ids_lock.lock();
            {
                defer self.released_ids_lock.unlock();

                // Loop until an ID is found.
                while (id == null) {
                    // If there are released IDs, use one.
                    if (self.released_ids.items.len > 0) {
                        id = self.released_ids.getLast();
                        if (!self.clients.contains(id.?)) {
                            try self.clients.put(id.?, BaseClient.new(allocator, id.?)); // Reserve this spot.

                        } else {
                            id = null;
                        }

                        _ = self.released_ids.pop();
                        continue;
                    } else {
                        // Assign the next highest client ID if there's no released IDs.
                        id = self.clients.count();

                        if (!self.clients.contains(id.?)) {
                            try self.clients.put(id.?, BaseClient.new(allocator, id.?)); // Reserve this spot here too.
                        } else {
                            id = null;
                            continue;
                        }
                    }
                }
            }
            var client = self.clients.get(id.?);
            if (client == null) {
                return error.ClientMissing;
            }
            client.?.received_data = Packet.new(allocator);

            {
                // const func = comptime struct {
                //     pub fn exec(protocol: Protocols, data: []u8) void {
                //         // TODO find a way to handle errors here.
                //         // errdefer {
                //         //     debugServer(self.server_type_name, "Something went wrong with a message received from Client#{d}.", .{client.?.id});
                //         // }

                //         switch (protocol) {
                //             Protocols.TCP => {
                //                 client.?.received_data.?.reset(self.handleTcpData(allocator, client.?, data));
                //                 return;
                //             },
                //             Protocols.UDP => {
                //                 // Extra things to do goes here.
                //                 return;
                //             },
                //         }
                //     }
                // };

                const action = struct {
                    action: ActionT2(Protocols, []u8, void) = .{ .compute = compute },
                    id: u32,
                    server: *BaseServer,
                    allocator: std.mem.Allocator,
                    client: *BaseClient,
                    fn compute(action: *ActionT2(Protocols, []u8, void), protocol: Protocols, data: []u8) void {
                        const this: *@This() = @alignCast(@fieldParentPtr("action", action));
                        // Error handle
                        errdefer {
                            debugServer(this.server.server_type_name, "Something went wrong with a message received from Client#{d}.", .{this.client.id});
                        }

                        switch (protocol) {
                            Protocols.TCP => {
                                const this_client = this.server.clients.get(this.id);
                                if (this_client == null) {
                                    // Error handle
                                    return;
                                }

                                var received_data = this_client.?.received_data;
                                if (received_data == null) {
                                    // Error handle
                                    return;
                                }

                                received_data.?.reset(this.server.handleTcpData(this.allocator, this_client.?, data));
                                return;
                            },
                            Protocols.UDP => {
                                // Extra things to do goes here.
                                return;
                            },
                        }
                    }
                };
                var callable = action{
                    .id = id.?,
                    .server = self,
                    .allocator = allocator,
                    .client = &client.?,
                };
                try client.?.on_received.append(&callable.action);
            }

            {
                // const func = comptime struct {
                //     pub fn exec() void {
                //         self.udpReady(const_id);
                //     }
                // };
                // client.?.on_connected.add(func.exec);
            }

            {
                // const func = comptime struct {
                //     pub fn exec(_: Protocols) void {
                //         self.disconnectClient(const_id);
                //     }
                // };
                // client.?.on_disconnected.add(func.exec);
            }

            // client.?.tcp.receive(client, tcp_client);

            for (self.onConnection.items) |item| {
                item.compute(item, id.?);
            }

            return;
        }

        if (tcp_client.endpoint == null) {
            return error.TcpClientEndpointNull;
        }

        debugServer(self.server_type_name, "{s} failed to connect. Max connections of {s} reached.", .{ tcp_client.endpoint.?.address.ipv4.value, self.max_connections });
    }
}

/// Kicks a client off the server and clears their entry.
fn disconnectClient(self: *BaseServer, client_id: u32) void {
    self.clearClient(client_id);
}

/// Frees up a client ID by wiping them from the server list.
fn clearClient(_: *BaseServer, _: u32) void {
    // TODO: Implmenet
    // self.clients_lock.lock();
    // {
    //     defer self.clients_lock.unlock();

    //     if (self.clients.contains(client_id)) {
    //         const client = self.clients.get(client_id);
    //         if (client == null) {
    //             return;
    //         }

    //         client.?.deinit();
    //         self.clients.remove(client_id);
    //         self.released_ids_lock.lock();
    //         {
    //             defer self.released_ids_lock.unlock();
    //             self.released_ids.append(client_id);
    //         }
    //         for (self.onDisconnection.items) |item| {
    //             item(client_id);
    //         }
    //     }
    // }
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
/// Handles TCP data. Returns true when no more data left.
fn handleTcpData(
    self: *BaseServer,
    allocator: std.mem.Allocator,
    client: BaseClient,
    data: []u8,
) bool {
    var packet_length: i32 = 0;

    if (client.received_data == null) {
        return true;
    }

    var received_data = client.received_data.?;

    received_data.setBytes(allocator, data) catch |err| {
        debugServer(self.server_type_name, "Error setting bytes: {}\n", .{err});
        return true;
    };

    if (received_data.unreadLength() >= 4) {
        packet_length = received_data.readInt();
        if (packet_length <= 0) {
            return true;
        }
    }

    while (packet_length > 0 and packet_length <= received_data.unreadLength()) {
        // TODO:
        // if(packetHandlers.Contains(packetId), maybe a try catch.

        // const packet_bytes: []u8 = received_data.readBytes(packet_length);

        {
            //     const func = comptime struct {
            //         pub fn exec() !void {
            //             if (packetHandlers == null) {
            //                 return error.PacketHandlersNull;
            //             }

            //             const packet: Packet = Packet.newWithData(allocator, packet_bytes);
            //             const packet_id = packet.readInt();

            //             const packet_handler = packetHandlers.?.get(packet_id);

            //             if (packet_handler == null) {
            //                 return error.PacketHandlerIdNotFound;
            //             }

            //             packet_handler.?.*(client.id, packet);
            //         }
            //     };
            //     ThreadManager.getInstance(allocator).executeOnMainThread(func.exec);
        }

        if (received_data.unreadLength() >= 4) {
            packet_length = received_data.readInt();
            if (packet_length <= 0) {
                return true;
            }
        }
    }

    if (packet_length <= 1) {
        return true;
    }

    return false;
}
//#endregion

//#region Memory Functions
pub fn deinit(self: *BaseServer) void {
    // Free packetHandlers
    if (BaseServer.packetHandlers != null) {
        BaseServer.packetHandlers.?.deinit();
        BaseServer.packetHandlers = null;
    }

    self.tcp_listener.deinit();

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

    // Free events
    self.onConnection.deinit();
    self.onDisconnection.deinit();
    self.onReceived.deinit();
}
//#endregion

pub fn debugServer(server_type_name: []const u8, comptime msg: []const u8, args: anytype) void {
    Utilities.printMsg("({s}) " ++ msg, .{ server_type_name, args });
}
