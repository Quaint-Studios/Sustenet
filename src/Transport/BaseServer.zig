//! The base of all server types. Takes in clients.
const std = @import("std");
const sustenet = @import("root").sustenet;
const network = @import("network");

const expect = std.testing.expect;
const print = std.debug.print;
const RwLock = std.Thread.RwLock;
const ThreadManager = sustenet.transport.ThreadManager;
const AutoHashMap = std.AutoHashMap;
const Constants = sustenet.utils.Constants;
const TcpListener = sustenet.network.TcpListener;
const Packet = sustenet.network.Packet;
const Protocols = sustenet.transport.Protocols;
const Utilities = sustenet.utils.Utilities;
const EventT1 = sustenet.events.BaseEvent.EventT1;
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
clients: AutoHashMap(i32, BaseClient),
released_ids: std.ArrayList(i32),

// Events
onConnection: EventT1(i32),
onDisconnection: EventT1(i32),
onReceived: EventT1([]u8),

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
        {
            // const func = comptime struct {
            //     const Self = @This();
            //     self: *BaseServer,
            //     pub fn exec(id: i32) void {
            //         Self.self.debugServer(Self.self.server_type_name, "Client#{d} has connected.", .{id});
            //     }
            // };
            // const func_instance = func{ .self = self };
            // self.onConnection.add(func_instance.exec);
        }

        Utilities.consoleHeader("Starting {s} on Port {d}", .{ self.server_type_name, self.port });
    }

    try self.tcp_listener.socket.listen();

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
fn onTcpConnectCallback(self: *BaseServer, _: std.mem.Allocator) void {
    const listener: TcpListener = self.tcp_listener;

    _ = listener.socket.accept() catch |err| {
        debugServer(self.server_type_name, "Error accepting TCP client: {}\n", .{err});
        return;
    };

    {
        // TODO: Fix asap
        // const func = comptime struct {
        //     server: *BaseServer,
        //     pub fn exec(this: *@This()) void {
        //         try this.server.addClient(allocator, client_socket);
        //     }
        // };

        // const func_i = func{ .server = self };

        // const callable = sustenet.events.BaseEvent.Callable(comptime addClient, .{ self, allocator, client_socket });

        // var thread_manager = try ThreadManager.getInstance(allocator);
        // thread_manager.executeOnMainThread(callable);
    }
}

fn addClient(self: *BaseServer, allocator: std.mem.Allocator, tcp_client: network.Socket) anyerror!void {
    var id: i32 = -1;
    errdefer {
        // If the id was never reset.
        // That means that a client may still exist.
        // Cleanup.
        if (id != -1) {
            self.disconnectClient(id);
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
                while (id == -1) {
                    // If there are released IDs, use one.
                    if (self.released_ids.items.len > 0) {
                        id = self.released_ids.getLast();
                        if (!self.clients.contains(id)) {
                            try self.clients.put(id, BaseClient.new(id)); // Reserve this spot.

                        } else {
                            id = -1;
                        }

                        self.released_ids.pop();
                        continue;
                    } else {
                        // Assign the next highest client ID if there's no released IDs.
                        id = self.clients.count();

                        if (!self.clients.contains(id)) {
                            self.clients.put(id, BaseClient.new(id)); // Reserve this spot here too.
                        } else {
                            id = -1;
                            continue;
                        }
                    }
                }
            }
            const client = self.clients.get(id);
            if (client == null) {
                return error.ClientMissing;
            }
            client.?.received_data = Packet.new(allocator);

            const const_id = id;

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
                // client.?.on_received.add(func.exec);
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

            self.onConnection.invoke(const_id);

            return;
        }

        if (tcp_client.endpoint == null) {
            return error.TcpClientEndpointNull;
        }

        debugServer(self.server_type_name, "{s} failed to connect. Max connections of {s} reached.", .{ tcp_client.endpoint.?.address.ipv4.value, self.max_connections });
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
/// Handles TCP data. Returns true when no more data left.
fn handleTcpData(
    allocator: std.mem.Allocator,
    client: BaseClient,
    data: []u8,
) bool {
    const packet_length: i32 = 0;

    if (client.received_data == null) {
        return true;
    }

    client.received_data.?.setBytes(allocator, data);

    if (client.received_data.?.unreadLength() >= 4) {
        packet_length = client.received_data.?.readInt();
        if (packet_length <= 0) {
            return true;
        }
    }

    while (packet_length > 0 and packet_length <= client.received_data.?.unreadLength()) {
        // TODO:
        // if(packetHandlers.Contains(packetId), maybe a try catch.

        // const packet_bytes: []u8 = client.received_data.?.readBytes(packet_length);

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

        if (client.received_data.?.unreadLength() >= 4) {
            packet_length = client.received_data.?.readInt();
            if (packet_length <= 0) {
                return true;
            }
        }

        if (packet_length <= 1) {
            return true;
        }

        return false;
    }
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
