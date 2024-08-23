//! Master namespace

pub const MasterServer = @import("MasterServer.zig");

test {
    @import("std").testing.refAllDecls(@This());
}
