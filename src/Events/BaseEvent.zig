//! This is a collection of implementations for Action in Zig.
//! Instead of being called Action, they're all Event.

const std = @import("std");

const ArrayList = std.ArrayList;

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
            for (self.invokes.items()) |callable| {
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

        pub inline fn add(self: *Self, callable: *const fn (T) void) void {
            self.invokes.append(callable);
        }

        pub inline fn addSlice(self: *Self, callables: []const *const fn (T) void) void {
            self.invokes.appendSlice(callables);
        }

        pub inline fn clear(self: *Self) void {
            self.invokes.clearAndFree();
        }

        pub inline fn invoke(self: *Self, arg: T) void {
            for (self.invokes.items()) |callable| {
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

        pub inline fn add(self: *Self, callable: *const fn (T1, T2) void) void {
            self.invokes.append(callable);
        }

        pub inline fn addSlice(self: *Self, callables: []const *const fn (T1, T2) void) void {
            self.invokes.appendSlice(callables);
        }

        pub inline fn clear(self: *Self) void {
            self.invokes.clearAndFree();
        }

        pub inline fn invoke(self: *Self, arg1: T1, arg2: T2) void {
            for (self.invokes.items()) |callable| {
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
