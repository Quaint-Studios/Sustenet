const std = @import("std");
pub const sustenet = @import("sustenet.zig");

const print = std.debug.print;
const ArrayList = std.ArrayList;
const transport = sustenet.transport;
const clients = sustenet.clients;

const BaseServer = transport.BaseServer;

pub var client_list: std.ArrayList(clients.Client) = undefined;
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

var is_running = false;

// pub var clients: Client = ...;

pub fn main() !void {
    try entrypoint();
}

test {
    std.testing.refAllDecls(@This());
}

// test "create server with gpa_allocator" {
//     var gpa = std.heap.GeneralPurposeAllocator(.{}){};
//     const allocator = gpa.allocator();
//     defer _ = gpa.deinit();

//     const n = 100000;

//     for (0..n) |_| {
//         var server = try BaseServer.new(allocator, BaseServer.ServerType.MasterServer, 10, 4337);
//         defer server.deinit();

//         try server.start();
//     }
// }

test "create server with page_allocator" {
    var gpa = std.heap.GeneralPurposeAllocator(.{}){};
    const allocator = gpa.allocator();

    const n = 1_000_000;
    const fmn = try sustenet.utils.Utilities.formatWithCommas(n);

    std.debug.print("Creating {s} servers...\n", .{fmn});

    for (0..n) |_| {
        var server = try BaseServer.new(allocator, BaseServer.ServerType.MasterServer, 10, 4337);
        defer server.deinit(allocator);

        try server.start();
    }

    std.debug.print("Finished creating {s} servers.\n", .{fmn});

    // std.time.sleep(4 * std.time.ns_per_s);
}

// test "create client with page_allocator" {
//     const n = 1;

//     for (0..n) |_| {
//         var client = clients.Client.new(4337);
//         defer client.super.deinit();

//         try client.connect();
//     }

//     // std.time.sleep(4 * std.time.ns_per_s);
// }

// test "create server with arena_allocator" {
//     var gpa = std.heap.GeneralPurposeAllocator(.{}){};
//     const allocator = gpa.allocator();

//     var arena = std.heap.ArenaAllocator.init(allocator);
//     defer arena.deinit();

//     const aa = arena.allocator();

//     const n = 750000;

//     for (0..n) |_| {
//         var server = try BaseServer.new(aa, BaseServer.ServerType.MasterServer, 10, 4337);
//         defer server.deinit();

//         try server.start();
//     }
// }
