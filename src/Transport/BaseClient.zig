//! The core for all clients. Handles basic functionality like sending
//! and receiving data. Also handles the core for connecting to servers.

const std = @import("std");
const net = std.net;
const print = std.debug.print;
const testing = std.testing; // Add import for testing package
const BaseClient = @This();

const bufferSize = 4028;

id: u32 = 0,
port: u16 = 4337,
stream: ?net.Stream,

pub fn new(port: u16) BaseClient {
    return BaseClient{ .id = 0, .port = port, .stream = null };
}

pub fn connect(self: *BaseClient) !void {
    self.id = 1;

    _ = try net.Address.parseIp4("127.0.0.1", self.port);

    // Connect to server.
    // self.stream = net.tcpConnectToAddress(server) catch |err| {
    //     print("Unable to connect to Sustenet Server.\n", .{});
    //     return err;
    // };
    // defer self.stream.?.close();
    // print("Connecting to {}\n", .{server});

    // try self.send("hello ziggy!");
}

pub fn send(self: *BaseClient, data: []const u8) !void {
    if (self.stream == null) return;

    var writer = self.stream.?.writer();
    const size = try writer.write(data);
    print("Sending '{s}' to peer, total written: {d} bytes\n", .{ data, size });
}

pub fn deinit(self: *BaseClient) void {
    if (self.stream != null) {
        self.stream.?.close();
        self.stream = null;
    }
}
