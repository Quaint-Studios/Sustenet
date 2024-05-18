const std = @import("std");
const net = std.net;
const print = std.debug.print;

const bufferSize = 4028;

pub const BaseClient = struct {
    id: u32 = 0,
    port: u16 = 4337,
    stream: ?net.Stream,

    pub fn connect(self: *BaseClient) !void {
        self.id = 1;

        const peer = try net.Address.parseIp4("127.0.0.1", self.port);

        // Connect to peer.
        self.stream = try net.tcpConnectToAddress(peer);
        defer self.stream.?.close();
        print("Connecting to {}\n", .{peer});

        try self.send("hello ziggy!");
    }

    pub fn send(self: *BaseClient, data: *const [12:0]u8) !void {
        if (self.stream == null) return;

        var writer = self.stream.?.writer();
        const size = try writer.write(data);
        print("Sending '{s}' to peer, total written: {d} bytes\n", .{ data, size });
    }
};

pub fn createClient(port: u16) BaseClient {
    return BaseClient{ .id = 0, .port = port, .stream = null };
}
