const std = @import("std");
const sustenet = @import("root").sustenet;

const ArrayList = std.ArrayList;
const Action = sustenet.utils.Extensions.Action;

pub const ThreadManager = @This();

mainPool: ArrayList(Action),
mainPoolCopied: ArrayList(Action),
executeAction: bool = false,

/// Sets an action to be executed on the main thread.
pub fn executeOnMainThread(
    /// The action to be executed on the main thread.
    action: Action,
) void {
    action.invoke();
}

/// Execute all code meant to run on the main thread. Should only be called from the main thread.
pub fn updateMain(self: ThreadManager) void {
    if (self.executeAction) {
        self.mainPoolCopied.clearAndFree();
        // RWLock mainPool
        {
            self.mainPoolCopied.appendSlice(self.mainPool);
            self.mainPool.clear();
            self.executeAction = false;
        }

        for (self.mainPoolCopied.items()) |action| {
            action.invoke();
        }
    }
}
