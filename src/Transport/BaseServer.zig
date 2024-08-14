//! The base of all server types. Takes in clients.

const std = @import("std");
const expect = std.testing.expect;
const print = std.debug.print;
const BaseServer = @This();

const net = std.net;

const ServerType = enum { MasterServer, ClusterServer };

is_listening: bool = false,
server_type: ServerType,
max_connections: u32,
port: u16,
pub fn start(self: *BaseServer) !void {
    print("Starting server on port {d}\n", .{self.port});

    const loopback = try net.Ip4Address.parse("127.0.0.1", self.port);
    const localhost = net.Address{ .in = loopback };
    var server = try localhost.listen(.{
        .reuse_address = true,
    });
    defer server.deinit();

    const addr = server.listen_address;

    print("Listening on {}, access this port to end the program\n", .{addr.getPort()});

    self.is_listening = true;

    try self.handle(&server);
}

pub fn createMasterServer(max_connections: u32, port: u16) BaseServer {
    print("Creating master server\n", .{});

    return BaseServer{
        .server_type = ServerType.MasterServer,
        .max_connections = max_connections,
        .port = port,
    };
}

pub fn createClusterServer(max_connections: u32, port: u16) BaseServer {
    return BaseServer{
        .server_type = ServerType.ClusterServer,
        .max_connections = max_connections,
        .port = port,
    };
}

pub fn handle(self: *BaseServer, server: *net.Server) !void {
    var gpa = std.heap.GeneralPurposeAllocator(.{}){};
    defer _ = gpa.deinit();
    const allocator = gpa.allocator();

    while (self.is_listening) {
        var client = try server.accept();
        defer client.stream.close();

        print("Connection received! {} is sending data.\n", .{client.address});

        const message = try client.stream.reader().readAllAlloc(allocator, 1024);
        defer allocator.free(message);

        print("{} says {s}\n", .{ client.address, message });
    }
}

test "setup server" {
    const server = BaseServer.createMasterServer(100, 8080);
    try expect(server.server_type == ServerType.MasterServer);
}
