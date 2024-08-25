const std = @import("std");
const sustenet = @import("root").sustenet;
const Constants = sustenet.utils.Constants;

//#region String Formatting

/// Caution: This function is not safe for use in production code.
/// Potential memory leak.
///
/// TODO: Fix this function.
pub fn splitByPascalCase(t: []const u8, allocator: std.mem.Allocator) ![]const u8 {
    var list = std.ArrayList(u8).init(allocator);
    defer list.deinit();

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

    return list.toOwnedSlice();
}

pub fn formatWithCommas(value: comptime_int, allocator: std.mem.Allocator) ![]const u8 {
    var buffer: [32]u8 = undefined; // Adjust size as needed
    var fba = std.io.fixedBufferStream(&buffer);
    var writer = fba.writer();
    try writer.print("{d}", .{value});
    const str = writer.context.getWritten();

    var result = std.ArrayList(u8).init(allocator);
    defer result.deinit();

    var count: i32 = 0;
    for (str, 0..) |_, i| {
        if (count > 0 and @mod(count, 3) == 0) {
            try result.append(',');
        }
        try result.append(str[i]);
        count += 1;
    }

    return result.toOwnedSlice();
}

pub fn consoleHeader(h: []const u8, args: anytype) void {
    comptime {
        if (Constants.DEBUGGING) {
            std.debug.print("===== " ++ h ++ " =====\n", .{args});
        }
    }
}
//#endregion

//#region File Handling
// TODO ...
//#endregion

//#region Debugging
pub fn printMsg(msg: []const u8, args: anytype) void {
    comptime {
        if (Constants.DEBUGGING) {
            std.debug.print(msg ++ "\n", args orelse .{});
        }
    }
}
//#endregion
