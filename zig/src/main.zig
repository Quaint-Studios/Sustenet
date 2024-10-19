const std = @import("std");
const builtin = @import("builtin");
pub const sustenet = @import("sustenet.zig");
const App = @import("App.zig");

const print = std.debug.print;

const Constants = sustenet.utils.Constants;
const ThreadManager = sustenet.transport.ThreadManager;

var is_running = false;

pub fn main() !void {
    // Get allocator
    var gpa = std.heap.GeneralPurposeAllocator(.{}){};
    const allocator = gpa.allocator();
    sustenet.allocator = allocator;
    defer _ = gpa.deinit();

    var threadManager = try ThreadManager.getInstance();
    defer threadManager.deinit();

    is_running = true;
    defer is_running = false;

    var logic_thread = try std.Thread.spawn(.{}, updateMain, .{});
    logic_thread.setName("Logic") catch |err| {
        print("Error setting thread name: {}\n", .{err});
    };
    logic_thread.detach();

    for (0..try std.Thread.getCpuCount()) |_| {
        var thread = std.Thread.spawn(.{}, updateSide, .{}) catch |err| {
            print("Error creating thread: {}\n", .{err});
            return;
        };
        thread.setName("Side") catch |err| {
            print("Error setting thread name: {}\n", .{err});
            return;
        };
        thread.detach();
    }

    const app = try allocator.create(App);
    defer allocator.destroy(app);
    app.* = App.init();
    try app.start();

    var buffer: [1]u8 = undefined;
    print("Press Enter to close Sustenet...\n", .{});
    _ = try std.io.getStdIn().reader().read(buffer[0..1]);

    print("Closing Sustenet...\n", .{});
}

fn updateMain() void {
    var next = std.time.milliTimestamp();
    var threadManager = try ThreadManager.getInstance();

    while (is_running) {
        const now = std.time.milliTimestamp();
        while (next < now) {
            threadManager.updateMain();
            next += Constants.MS_PER_TICK;

            if (next > now) {
                std.time.sleep(@as(u64, @intCast(next - now)) * std.time.ns_per_ms);
            }
        }
    }
}

fn updateSide() void {
    var next = std.time.milliTimestamp();
    var threadManager = try ThreadManager.getInstance();

    while (is_running) {
        const now = std.time.milliTimestamp();
        while (next < now) {
            threadManager.updateSide();
            const available_threads: i64 = @intCast(std.Thread.getCpuCount() catch 1);
            next += Constants.MS_PER_TICK * @max(1, available_threads - 1);

            if (next > now) {
                std.time.sleep(@as(u64, @intCast(next - now)) * std.time.ns_per_ms);
            }
        }
    }
}
