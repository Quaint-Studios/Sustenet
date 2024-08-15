const std = @import("std");
const sustenet = @import("root").sustenet;
const Constants = sustenet.utils.Constants;

//#region String Formatting
pub fn splitByPascalCase(t: []const u8) ![]const u8 {
    const allocator = std.heap.page_allocator;
    var list = std.ArrayList(u8).init(allocator);
    errdefer list.deinit();

    var start: usize = 0;
    for (t, 0..) |c, i| {
        if (std.ascii.isUpper(c)) {
            if (i != start) {
                try list.appendSlice(t[start..i]);
                try list.append(' ');
            }
            start = i;
        }
    }

    if (start != t.len) {
        try list.appendSlice(t[start..]);
    }

    const slice = list.toOwnedSlice();
    return slice;
}

pub fn consoleHeader(h: []const u8) void {
    std.debug.print("===== {s} =====\n", .{h});
}
//#endregion

//#region File Handling
// TODO ...
//#endregion

//#region Debugging
pub fn printMsg(msg: []const u8) void {
    comptime {
        if (Constants.DEBUGGING) {
            std.debug.print("{s}\n", .{msg});
        }
    }
}
//#endregion
