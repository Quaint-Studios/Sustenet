//! Handles the execution of code i

const std = @import("std");
const sustenet = @import("root").sustenet;

const RwLock = std.Thread.RwLock;
const ArrayList = std.ArrayList;
const Action = sustenet.events.Action;

const ThreadManager = @This();
pub var instance: ?ThreadManager = null;

main_pool_lock: RwLock = .{},
main_pool: ArrayList(*Action(void)),

side_pool_lock: RwLock = .{},
side_pool: ArrayList(*Action(void)),

execute_main_event: bool = false,
execute_side_event: bool = false,

pub const ThreadManagerError = error{
    InstanceIsNull,
};

/// Get the singleton instance.
pub fn getInstance() !*ThreadManager {
    if (instance == null) {
        instance = ThreadManager{
            .main_pool = ArrayList(*Action(void)).init(sustenet.allocator),
            .side_pool = ArrayList(*Action(void)).init(sustenet.allocator),

            .execute_main_event = false,
            .execute_side_event = false,
        };
    }

    return &instance.?;
}

//#region Execution Functions
/// Sets an event to be executed on the main thread.
pub fn executeOnMainThread(
    self: *ThreadManager,
    /// The event to be executed on the main thread.
    callable: *Action(void),
) void {
    self.main_pool_lock.lock();
    defer self.main_pool_lock.unlock();

    self.main_pool.append(callable) catch |err| {
        std.log.err("Failed to append callable to main pool: {}\n", .{err});
        return;
    };
    self.execute_main_event = true;
}

/// Execute all code meant to run on the main thread. Should only be called from the main thread.
pub fn updateMain(self: *ThreadManager) void {
    self.main_pool_lock.lock();
    if (self.execute_main_event) {
        const main_pool_slice = self.main_pool.toOwnedSlice() catch |err| {
            std.log.err("Failed to get main pool slice: {}\n", .{err});
            self.main_pool_lock.unlock();
            return;
        };
        self.execute_main_event = false;

        self.main_pool_lock.unlock();

        for (main_pool_slice) |callable| {
            callable.compute(callable);
        }
    } else {
        self.main_pool_lock.unlock();
    }
}

pub fn executeOnSideThread(
    self: *ThreadManager,
    /// The event to be executed on the main thread.
    callable: *Action(void),
) void {
    self.side_pool_lock.lock();
    defer self.side_pool_lock.unlock();

    self.side_pool.append(callable) catch |err| {
        std.log.err("Failed to append callable to side pool: {}\n", .{err});
        return;
    };
    self.execute_side_event = true;
    std.debug.print("Setting side event to {}\n", .{self.execute_side_event});
}

pub fn updateSide(self: *ThreadManager) void {
    self.side_pool_lock.lock();
    if (self.execute_side_event) {
        const side_pool_slice = self.side_pool.toOwnedSlice() catch |err| {
            self.side_pool_lock.unlock();
            std.log.err("Failed to get side pool slice: {}\n", .{err});
            return;
        };
        self.execute_side_event = false;

        self.side_pool_lock.unlock();

        std.debug.print("Calling...", .{});
        for (side_pool_slice) |const_callable| {
            var callable = const_callable;
            callable.compute(callable);
        }
        std.debug.print("CALLED", .{});
    } else {
        self.side_pool_lock.unlock();
    }
}
//#endregion

//#region Memory Functions
pub fn deinit(self: *ThreadManager) void {
    self.main_pool.clearAndFree();
    self.main_pool.deinit();

    self.side_pool.clearAndFree();
    self.side_pool.deinit();

    instance = null;
}
//#endregion
