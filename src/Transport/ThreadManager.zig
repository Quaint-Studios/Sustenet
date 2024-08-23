const std = @import("std");
const sustenet = @import("root").sustenet;

const RwLock = std.Thread.RwLock;
const ArrayList = std.ArrayList;
const Event = sustenet.events.BaseEvent.Event();

const ThreadManager = @This();
var instance: ?ThreadManager = null;

main_pool_lock: RwLock = .{},
main_pool: Event,
main_pool_copied: Event,
execute_action: bool = false,

/// Get the singleton instance.
pub fn getInstance(allocator: std.mem.Allocator) !ThreadManager {
    if (instance == null) {
        instance = ThreadManager{
            .main_pool = Event.init(allocator),
            .main_pool_copied = Event.init(allocator),
            .execute_action = false,
        };
    }

    return instance.?;
}

//#region Execution Functions
/// Sets an action to be executed on the main thread.
pub fn executeOnMainThread(
    self: *ThreadManager,
    /// The action to be executed on the main thread.
    action: Event,
) void {
    self.main_pool_lock.lock();
    defer self.main_pool_lock.unlock();

    self.main_pool.append(action) catch unreachable;
    self.execute_action = true;
}

/// Execute all code meant to run on the main thread. Should only be called from the main thread.
pub fn updateMain(self: *ThreadManager) void {
    if (self.execute_action) {
        self.main_pool_lock.lock();
        defer self.main_pool_lock.unlock();

        self.main_pool_copied.clearAndFree();
        {
            const main_pool_slice = self.main_pool.toOwnedSlice() catch unreachable;
            self.main_pool_copied.appendSlice(main_pool_slice) catch unreachable;
            self.execute_action = false;
        }

        for (self.main_pool_copied.items) |action| {
            action.invoke();
        }
    }
}
//#endregion

//#region Memory Functions
pub fn deinit(self: *ThreadManager) void {
    self.main_pool.deinit();
    self.main_pool_copied.deinit();
    instance = null;
}
//#endregion
