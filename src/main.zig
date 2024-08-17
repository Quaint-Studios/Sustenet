const std = @import("std");
pub const sustenet = @import("sustenet.zig");

const print = std.debug.print;
const ArrayList = std.ArrayList;
const transport = sustenet.transport;
const clients = sustenet.clients;

const BaseServer = transport.BaseServer;

pub var client_list: std.ArrayList(clients.Client) = undefined;

var is_running = false;

// pub var cluster = undefined;
// pub var master = undefined;

fn entrypoint() !void {
    // Get allocator
    var gpa = std.heap.GeneralPurposeAllocator(.{}){};
    const allocator = gpa.allocator();
    defer _ = gpa.deinit();

    var argsIterator = try std.process.ArgIterator.initWithAllocator(allocator);
    defer argsIterator.deinit();

    _ = argsIterator.next(); // Skip the first argument, which is the program name

    if (argsIterator.next()) |arg| {
        if (std.mem.eql(u8, arg, "server")) { // ----- Server mode
            var master_server = try BaseServer.new(allocator, BaseServer.ServerType.MasterServer, 10, 4337);
            defer master_server.deinit(allocator);

            try master_server.start();
        } else if (std.mem.eql(u8, arg, "client")) { // ----- Client mode
            client_list = ArrayList(clients.Client).init(allocator);
            defer client_list.deinit();

            var max_clients: u32 = 10; // Default value
            if (argsIterator.next()) |num_arg| {
                max_clients = std.fmt.parseInt(u32, num_arg, 10) catch 10;

                // Print the number of clients
                print("Number of clients: {}\n", .{max_clients});
            }

            for (0..max_clients) |_| {
                var client = clients.Client.new(4337);
                try client_list.append(client);

                try client.connect();
            }

            print("Finished connecting {} clients to the server.\n", .{max_clients});
        } else {
            print("Unknown mode provided. Aborting.\n", .{});
        }
    } else {
        print("No mode specified. Run `zig build run -- <client|server> [max clients|max connections]`.\n", .{});
    }
}

pub fn main() !void {
    try entrypoint();
}

//#region Tests
test {
    std.testing.refAllDecls(@This());
}

test "create server(s) with gp_allocator" {
    var gpa = std.heap.GeneralPurposeAllocator(.{}){};
    const allocator = gpa.allocator();

    const n = 1;
    const fmn = try sustenet.utils.Utilities.formatWithCommas(n);

    std.debug.print("Creating {s} servers...\n", .{fmn});

    for (0..n) |_| {
        var server = try BaseServer.new(allocator, BaseServer.ServerType.MasterServer, 10, 4337);
        defer server.deinit(allocator);

        try server.start();
    }

    std.debug.print("Finished creating {s} servers.\n", .{fmn});
}
//#endregion
