const std = @import("std");
const sustenet = @import("root").sustenet;

const RwLock = std.Thread.RwLock;
const ArrayList = std.ArrayList;
const Event = sustenet.events.BaseEvent.Event();

const ThreadManager = @This();
pub var instance: ?ThreadManager = null;

main_pool_lock: RwLock = .{},
main_pool: Event,
main_pool_copied: Event,
execute_event: bool = false,

pub const ThreadManagerError = error{
    InstanceIsNull,
};

/// Get the singleton instance.
pub fn getInstance(allocator: std.mem.Allocator) !ThreadManager {
    if (instance == null) {
        instance = ThreadManager{
            .main_pool = Event.init(allocator),
            .main_pool_copied = Event.init(allocator),
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
    callable: *const fn () void,
) void {
    self.main_pool_lock.lock();
    defer self.main_pool_lock.unlock();

    self.main_pool.add(callable) catch unreachable;
    self.execute_event = true;
}

/// Execute all code meant to run on the main thread. Should only be called from the main thread.
pub fn updateMain(self: *ThreadManager) void {
    if (self.execute_event) {
        self.main_pool_lock.lock();
        defer self.main_pool_lock.unlock();

        self.main_pool_copied.clear();
        {
            const main_pool_slice = self.main_pool.invokes.toOwnedSlice() catch unreachable;
            self.main_pool_copied.invokes.appendSlice(main_pool_slice) catch unreachable;
            self.execute_event = false;
        }

        self.main_pool_copied.invoke();
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
