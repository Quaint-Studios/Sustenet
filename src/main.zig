const std = @import("std");
const print = std.debug.print;

const sustenet = @import("sustenet.zig");
const transport = sustenet.transport;

fn entrypoint() !void {
    // Get allocator
    var gpa = std.heap.GeneralPurposeAllocator(.{}){};
    const allocator = gpa.allocator();
    defer _ = gpa.deinit();

    // Parse args into string array (error union needs 'try')
    // const args = try std.process.argsAlloc(allocator);

    var argsIterator = try std.process.ArgIterator.initWithAllocator(allocator);
    defer argsIterator.deinit();

    // defer std.process.argsFree(allocator, args);

    _ = argsIterator.next(); // Skip the first argument, which is the program name

    if (argsIterator.next()) |arg| {
        if (std.mem.eql(u8, arg, "server")) {
            print("Server mode.\n", .{});
            var master_server = transport.BaseServer.createMasterServer(100, 4337);
            try master_server.start();
        } else if (std.mem.eql(u8, arg, "client")) {
            print("Client mode.\n", .{});
            var client = transport.BaseClient.createClient(4337);
            try client.connect();
        } else {
            print("Unknown mode.\n", .{});
        }
    } else {
        print("No mode specified. Run `zig build run -- <client|server>`.\n", .{});
    }
}

var is_running = false;

// pub var clients: Client = ...;

pub fn main() !void {
    try entrypoint();
}

test "simple test" {
    var list = std.ArrayList(i32).init(std.testing.allocator);
    defer list.deinit(); // try commenting this out and see if zig detects the memory leak!
    try list.append(42);
    try std.testing.expectEqual(@as(i32, 42), list.pop());
}
