const std = @import("std");

const BaseTest = @This();

id: u32,
port: u16,
client: ClientS,
ids: std.ArrayList(u32),

pub fn new(allocator: std.mem.Allocator, port: u16) !BaseTest {
    const client = ClientS{
        .id = 1,
        .port = 4337,
    };

    return BaseTest{
        .id = 0,
        .port = port,
        .client = client,
        .ids = std.ArrayList(u32).init(allocator),
    };
}

pub fn deinit(self: *BaseTest) void {
    // Free ids
    self.ids.deinit();
}

const ClientS = struct {
    id: u32,
    port: u16,
};
