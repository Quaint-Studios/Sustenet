//! Events namespace

const action = @import("Action.zig");
pub const Action = action.Action;
pub const ActionT1 = action.ActionT1;
pub const ActionT2 = action.ActionT2;

test {
    @import("std").testing.refAllDecls(@This());
}
