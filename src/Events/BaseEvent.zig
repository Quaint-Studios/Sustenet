//! This is a collection of implementations for Action in Zig.
//! Instead of being called Action, they're all Event.

const std = @import("std");

const ArrayList = std.ArrayList;

//#region Callable WORK IN PROGRESS REGION
/// A way to have closures in Zig.
///
/// ```zig
/// /// TODO example
/// ```
// pub fn Callable(comptime funcT: type, argsT: type) type {
//     return struct {
//         func: funcT,
//         args: argsT,

//         const Self = @This();

//         pub fn init(func: funcT, args: argsT) Self {
//             return Self{ .func = func, .args = args };
//         }

//         pub fn call(self: *Self) void {
//             const ArgsType = @TypeOf(self.args);
//             const args_type_info = @typeInfo(ArgsType);
//             if (args_type_info != .Struct) {
//                 @compileError("expected tuple or struct argument, found " ++ @typeName(ArgsType));
//             }

//             const fields_info = args_type_info.Struct.fields;
//             if (fields_info.len == 0) {
//                 self.func();
//             } else if (fields_info.len == 1) {
//                 self.func(@field(self.args, fields_info[0].name));
//             } else if (fields_info.len == 2) {
//                 self.func(
//                     @field(self.args, fields_info[0].name),
//                     @field(self.args, fields_info[1].name),
//                 );
//             } else if (fields_info.len == 3) {
//                 self.func(
//                     @field(self.args, fields_info[0].name),
//                     @field(self.args, fields_info[1].name),
//                     @field(self.args, fields_info[2].name),
//                 );
//             } else {
//                 @compileError("3 argument max are supported per Callable call");
//             }
//         }
//     };
// }

pub fn Callable(comptime funcT: anytype, args: anytype) CallableWrapper(funcT, @TypeOf(args)) {
    return .{ .func = funcT, .args = args };
}

fn CallableWrapper(comptime funcT: anytype, comptime Args: type) type {
    return struct {
        func: *const @TypeOf(funcT),
        args: Args,

        const Self = @This();

        pub fn call(self: *Self) void {
            @call(.auto, self.func, .{self.args});
        }
    };
}
//#endregion

//#region Action
/// A Zig implmenetation of C#'s `Action`.
pub fn Event() type {
    return struct {
        invokes: List,

        const List = ArrayList(*const fn () void);

        const Self = @This();

        pub inline fn init(allocator: std.mem.Allocator) Self {
            return Self{ .invokes = ArrayList(*const fn () void).init(allocator) };
        }

        pub inline fn add(self: *Self, callable: *const fn () void) !void {
            try self.invokes.append(callable);
        }

        pub inline fn addSlice(self: *Self, callables: []const *const fn () void) !void {
            try self.invokes.appendSlice(callables);
        }

        pub inline fn clear(self: *Self) void {
            self.invokes.clearAndFree();
        }

        pub inline fn invoke(self: *Self) void {
            for (self.invokes.items) |callable| {
                callable();
            }
        }

        pub inline fn deinit(self: *Self) void {
            self.invokes.clearAndFree();
            self.invokes.deinit();
        }
    };
}

/// A Zig implmenetation of C#'s `Action<T>`.
pub fn EventT1(comptime _T: type) type {
    return struct {
        invokes: List,

        pub const T = _T;
        const List = ArrayList(*const fn (T) void);

        const Self = @This();

        pub inline fn init(allocator: std.mem.Allocator) Self {
            return Self{ .invokes = ArrayList(*const fn (T) void).init(allocator) };
        }

        pub inline fn add(self: *Self, callable: *const fn (T) void) !void {
            try self.invokes.append(callable);
        }

        pub inline fn addSlice(self: *Self, callables: []const *const fn (T) void) !void {
            try self.invokes.appendSlice(callables);
        }

        pub inline fn clear(self: *Self) void {
            self.invokes.clearAndFree();
        }

        pub inline fn invoke(self: *Self, arg: T) void {
            for (self.invokes.items) |callable| {
                callable(arg);
            }
        }

        pub inline fn deinit(self: *Self) void {
            self.invokes.clearAndFree();
            self.invokes.deinit();
        }
    };
}

/// A Zig implmenetation of C#'s `Action<T1, T2>`.
pub fn EventT2(comptime _T1: type, comptime _T2: type) type {
    return struct {
        invokes: List,

        pub const T1 = _T1;
        pub const T2 = _T2;
        const List = ArrayList(*const fn (T1, T2) void);

        const Self = @This();

        pub inline fn init(allocator: std.mem.Allocator) Self {
            return Self{ .invokes = ArrayList(*const fn (T1, T2) void).init(allocator) };
        }

        pub inline fn add(self: *Self, callable: *const fn (T1, T2) void) !void {
            try self.invokes.append(callable);
        }

        pub inline fn addSlice(self: *Self, callables: []const *const fn (T1, T2) void) !void {
            try self.invokes.appendSlice(callables);
        }

        pub inline fn clear(self: *Self) void {
            self.invokes.clearAndFree();
        }

        pub inline fn invoke(self: *Self, arg1: T1, arg2: T2) void {
            for (self.invokes.items) |callable| {
                callable(arg1, arg2);
            }
        }

        pub inline fn deinit(self: *Self) void {
            self.invokes.clearAndFree();
            self.invokes.deinit();
        }
    };
}
//#endregion
