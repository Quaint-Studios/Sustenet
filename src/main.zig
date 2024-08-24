const std = @import("std");
const builtin = @import("builtin");
const Options = @import("Options.zig");
pub const sustenet = @import("sustenet.zig");

const eql = std.mem.eql;
const print = std.debug.print;
const ArrayList = std.ArrayList;
const transport = sustenet.transport;
const master = sustenet.master;
const clients = sustenet.clients;

const Constants = sustenet.utils.Constants;
const BaseServer = transport.BaseServer;

var is_running = false;

pub var client_list: std.ArrayList(clients.Client) = undefined;
// pub var cluster_server: world.ClusterServer = undefined;
pub var master_server: master.MasterServer = undefined;

pub fn main() !void {
    // Get allocator
    var gpa = std.heap.GeneralPurposeAllocator(.{}){};
    const allocator = gpa.allocator();
    defer _ = gpa.deinit();

    var argsIterator = try std.process.ArgIterator.initWithAllocator(allocator);
    defer argsIterator.deinit();

    _ = argsIterator.next(); // Skip the first argument, which is the program name

    if (argsIterator.next()) |arg| {
        if (eql(u8, arg, "help")) {
            Options.showHelp();
            return;
        } else if (eql(u8, arg, "version")) {
            print("Sustenet v{s}\n", .{sustenet.utils.Constants.VERSION});
            return;
        } else if (eql(u8, arg, "client") or eql(u8, arg, "c")) {
            // Only to be used for debugging.
            client_list = ArrayList(clients.Client).init(allocator);
            defer {
                for (client_list.items) |const_client| {
                    var client = const_client;
                    client.deinit();
                }
                client_list.deinit();
            }

            var max_clients: u32 = 1; // Default value

            // Check if the user provided a number of clients to connect
            print("{s}Starting client mode...{s} ", .{
                Constants.TERMINAL_ORANGE,
                Constants.TERMINAL_DEFAULT,
            });
            if (argsIterator.next()) |num_arg| {
                max_clients = std.fmt.parseInt(u32, num_arg, 10) catch 10;

                // Print the number of clients
                print("{s}Number of clients: {d}{s}\n", .{
                    Constants.TERMINAL_BLUE,
                    max_clients,
                    Constants.TERMINAL_DEFAULT,
                });
            } else {
                print("{s}No number of clients provided. Defaulting to 1.{s}\n", .{
                    Constants.TERMINAL_BLUE,
                    Constants.TERMINAL_DEFAULT,
                });
            }

            // Connect the clients
            for (0..max_clients) |_| {
                var client = clients.Client.new(null, null);
                defer client.deinit();
                try client_list.append(client);

                try client.connect();
            }

            print("{s}Finished connecting {d} clients to the server.{s}\n", .{
                Constants.TERMINAL_GREEN,
                max_clients,
                Constants.TERMINAL_DEFAULT,
            });
        } else if (eql(u8, arg, "cluster") or eql(u8, arg, "cs")) {
            return;
        } else if (eql(u8, arg, "master") or eql(u8, arg, "ms")) {
            // TODO Use config file

            master_server = try master.MasterServer.new(allocator, 0, 4337);
            defer master_server.deinit();
        } else {
            print("Add 'help' to this command to get a list of options.\n", .{});
            return;
        }
    }

    is_running = true;
    defer is_running = false;

    var logic_thread = try std.Thread.spawn(.{}, updateMain, .{allocator});
    logic_thread.setName("Logic Thread") catch |err| {
        print("Error setting thread name: {}\n", .{err});
    };
    logic_thread.detach();

    var buffer: [1]u8 = undefined;
    print("Press Enter to close Sustenet...", .{});
    _ = try std.io.getStdIn().reader().read(buffer[0..1]);
}

fn updateMain(allocator: std.mem.Allocator) void {
    var next = std.time.milliTimestamp();
    var ThreadManager = try transport.ThreadManager.getInstance(allocator);

    while (is_running) {
        const now = std.time.milliTimestamp();
        while (next < now) {
            // ThreadManager.updateMain();
            next += Constants.MS_PER_TICK;

            if (next > now) {
                std.time.sleep(@as(u64, @intCast(next - now)) * std.time.ns_per_ms);
            }
        }
        print("Tick\n", .{});
    }
    ThreadManager.deinit();
}

//#region Tests
test {
    std.testing.refAllDecls(@This());
}

test "create server(s) with gp_allocator" {
    var gpa = std.heap.GeneralPurposeAllocator(.{}){};
    const allocator = gpa.allocator();

    const n = 1;
    const fmn = try sustenet.utils.Utilities.formatWithCommas(n, allocator);

    print("Creating {s} servers...\n", .{fmn});

    for (0..n) |_| {
        var server = try BaseServer.new(allocator, BaseServer.ServerType.MasterServer, 10, 4337);
        defer server.deinit();

        try server.start(allocator);
    }

    print("Finished creating {s} servers.\n", .{fmn});
}
//#endregion
