//! Client namespace

pub const Client = @import("Client.zig");

test {
    @import("std").testing.refAllDecls(@This());
}
