const std = @import("std");
const print = std.debug.print;

const base_server = @import("Transport/base_server.zig");
const base_client = @import("Transport/base_client.zig");

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
            print("Server mode\n", .{});
            var master_server = base_server.createMasterServer(100, 4337);
            try master_server.start();
        } else if (std.mem.eql(u8, arg, "client")) {
            print("Client mode\n", .{});
            var client = base_client.createClient(4337);
            try client.connect();
        } else {
            print("Unknown mode\n", .{});
        }
    } else {
        print("No mode specified\n", .{});
    }
}

pub fn main() !void {
    try entrypoint();

    // Prints to stderr (it's a shortcut based on `std.io.getStdErr()`)
    print("All your {s} are belong to us 2.\n", .{"codebase"});

    // stdout is for the actual output of your application, for example if you
    // are implementing gzip, then only the compressed bytes should be sent to
    // stdout, not any debugging messages.
    const stdout_file = std.io.getStdOut().writer();
    var bw = std.io.bufferedWriter(stdout_file);
    const stdout = bw.writer();

    try stdout.print("Run `zig build test` to run the tests.\n", .{});

    try bw.flush(); // don't forget to flush!
}

test "simple test" {
    var list = std.ArrayList(i32).init(std.testing.allocator);
    defer list.deinit(); // try commenting this out and see if zig detects the memory leak!
    try list.append(42);
    try std.testing.expectEqual(@as(i32, 42), list.pop());
}
