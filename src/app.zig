const std = @import("std");
const sustenet = @import("sustenet.zig");
const Options = @import("Options.zig");

const ArrayList = std.ArrayList;
const eql = std.mem.eql;
const Mutex = std.Thread.Mutex;
const print = std.debug.print;

const Action = sustenet.events.Action;
const Client = sustenet.clients.Client;
const Constants = sustenet.utils.Constants;
// const ClusterServer = sustenet.world.ClusterServer;
const MasterServer = sustenet.master.MasterServer;
const ThreadManager = sustenet.transport.ThreadManager;

client_list: ArrayList(Client) = undefined,
client_list_mutex: Mutex = .{},
// cluster_server: world.ClusterServer = undefined,
master_server: MasterServer = undefined,

const App = @This();

pub fn init() App {
    return App{};
}

pub fn start(self: *App) !void {
    var argsIterator = try std.process.ArgIterator.initWithAllocator(sustenet.allocator);
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
            self.startClient(&argsIterator);
            return;
        } else if (eql(u8, arg, "cluster") or eql(u8, arg, "cs")) {
            return;
        } else if (eql(u8, arg, "master") or eql(u8, arg, "ms")) {
            // TODO Use config file
            self.master_server = try MasterServer.new(sustenet.allocator, 0, 4337);
            defer self.master_server.deinit();
        } else {
            print("Add 'help' to this command to get a list of options.\n", .{});
            return;
        }
    }
}

/// Start the client mode.
///
/// Only meant for debugging.
fn startClient(self: *App, argsIterator: *std.process.ArgIterator) void {
    self.client_list = ArrayList(Client).init(sustenet.allocator);

    var max_clients: u32 = 1; // Default value

    // Check if the user provided a number of clients to connect
    print("{s}Starting client mode...{s} ", .{ Constants.TERMINAL_ORANGE, Constants.TERMINAL_DEFAULT });
    if (argsIterator.next()) |num_arg| {
        max_clients = std.fmt.parseInt(u32, num_arg, 10) catch 10;

        // Print the number of clients
        print("{s}Number of clients: {d}{s}\n", .{ Constants.TERMINAL_BLUE, max_clients, Constants.TERMINAL_DEFAULT });
    } else {
        print("{s}No number of clients provided. Defaulting to 1.{s}\n", .{ Constants.TERMINAL_BLUE, Constants.TERMINAL_DEFAULT });
    }

    // Connect the clients
    for (0..max_clients) |_| {
        const action = struct {
            action: Action(void) = .{ .compute = compute },
            app: *App,
            fn compute(action: *Action(void)) void {
                const this: *@This() = @alignCast(@fieldParentPtr("action", action));

                var client = Client.new(sustenet.allocator, null, null) catch |err| {
                    std.log.err("Failed to create client: {}\n", .{err});
                    return;
                };

                std.debug.print("Connecting client to IP {}:{}\n", .{ client.master_connection.ip, client.master_connection.port });

                this.app.client_list_mutex.lock();
                this.app.client_list.append(client) catch |err| {
                    std.log.err("Failed to append client to client list: {}\n", .{err});
                    this.app.client_list_mutex.unlock();
                    return;
                };
                this.app.client_list_mutex.unlock();

                client.connect(.MasterServer) catch {
                    std.log.debug("Error connecting client to IP {}:{}\n", .{ client.master_connection.ip, client.master_connection.port });
                };
            }
        };
        var callable = action{ .app = self };
        // callable.action.compute(&callable.action);
        var threadManager = try ThreadManager.getInstance();
        threadManager.executeOnMainThread(&callable.action);
    }

    print("{s}Finished connecting {d} clients to the server.{s}\n", .{ Constants.TERMINAL_GREEN, max_clients, Constants.TERMINAL_DEFAULT });
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
        var server = try MasterServer.new(allocator, 0, 4337);
        defer server.deinit();
    }

    print("Finished creating {s} servers.\n", .{fmn});
}
//#endregion
