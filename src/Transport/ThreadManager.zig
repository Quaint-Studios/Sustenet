const std = @import("std");
const sustenet = @import("root").sustenet;

const RwLock = std.Thread.RwLock;
const ArrayList = std.ArrayList;
const Action = sustenet.events.Action;

const ThreadManager = @This();
pub var instance: ?ThreadManager = null;

main_pool_lock: RwLock = .{},
main_pool: ArrayList(*const Action(void)),
main_pool_copied: ArrayList(*const Action(void)),
execute_event: bool = false,

pub const ThreadManagerError = error{
    InstanceIsNull,
};

/// Get the singleton instance.
pub fn getInstance(allocator: std.mem.Allocator) !ThreadManager {
    if (instance == null) {
        instance = ThreadManager{
            .main_pool = ArrayList(*const Action(void)).init(allocator),
            .main_pool_copied = ArrayList(*const Action(void)).init(allocator),
            .execute_event = false,
        };
    }

    return instance.?;
}

//#region Execution Functions
/// Sets an event to be executed on the main thread.
pub fn executeOnMainThread(
    self: *ThreadManager,
    /// The event to be executed on the main thread.
    callable: *const Action(void),
) void {
    self.main_pool_lock.lock();
    defer self.main_pool_lock.unlock();

    self.main_pool.append(callable) catch |err| {
        std.log.err("Failed to append callable to main pool: {}\n", .{err});
        return;
    };
    self.execute_event = true;
}

/// Execute all code meant to run on the main thread. Should only be called from the main thread.
pub fn updateMain(self: *ThreadManager) void {
    if (self.execute_event) {
        self.main_pool_lock.lock();
        defer self.main_pool_lock.unlock();

        self.main_pool_copied.clearAndFree();
        {
            const main_pool_slice = self.main_pool.toOwnedSlice() catch |err| {
                std.log.err("Failed to get main pool slice: {}\n", .{err});
                return;
            };
            self.main_pool_copied.appendSlice(main_pool_slice) catch |err| {
                std.log.err("Failed to append main pool slice: {}\n", .{err});
                return;
            };
            self.execute_event = false;
        }

        for (self.main_pool_copied.items) |callable| {
            callable.compute();
        }
    }
}
//#endregion

//#region Memory Functions
pub fn deinit(self: *ThreadManager) void {
    self.main_pool.clearAndFree();
    self.main_pool.deinit();
    self.main_pool.clearAndFree();
    self.main_pool_copied.deinit();
    instance = null;
}
//#endregion
