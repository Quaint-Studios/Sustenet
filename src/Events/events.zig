//! Events namespace

pub const BaseEvent = @import("BaseEvent.zig");

test {
    @import("std").testing.refAllDecls(@This());
}
