//! This is a collection of implementations for C#'s Action in Zig.

/// Allows you to pass variables easier in a function.
///
/// Usage:
///
/// ```zig
/// const action = struct {
///     action: Action(void) = .{ .compute = compute },
///     field: *Pointer,
///     fn compute(action: *Action(void)) void {
///         const this: *@This() = @alignCast(@fieldParentPtr("action", action));
///         Struct.func(this.field.*.some_field_on_struct);
///     }
/// };
/// var callable = action{ .field = self };
/// try self.collection.append(&callable.action);
/// ```
pub fn Action(comptime R: type) type {
    return struct {
        compute: *const fn (*Action(R)) R,
    };
}

/// Allows you to pass variables easier in a function.
///
/// Usage:
///
/// ```zig
/// const action = struct {
///     action: ActionT1(u32, void) = .{ .compute = compute },
///     field: *Pointer,
///     fn compute(action: *ActionT1(u32, void), id: u32) void {
///         const this: *@This() = @alignCast(@fieldParentPtr("action", action));
///         Struct.func(this.field.*.some_field_on_struct, id);
///     }
/// };
/// var callable = action{ .field = self };
/// try self.collection.append(&callable.action);
/// ```
pub fn ActionT1(comptime T1: type, comptime R: type) type {
    return struct {
        compute: *const fn (*ActionT1(T1, R), T1) R,
    };
}

/// Allows you to pass variables easier in a function.
///
/// Usage:
///
/// ```zig
/// const action = struct {
///     action: ActionT2(u32, u8, void) = .{ .compute = compute },
///     field: *Pointer,
///     fn compute(action: *ActionT2(u32, void), id: u32, port: u8) void {
///         const this: *@This() = @alignCast(@fieldParentPtr("action", action));
///         Struct.func(this.field.*.some_field_on_struct, id, port);
///     }
/// };
/// var callable = action{ .field = self };
/// try self.collection.append(&callable.action);
/// ```
pub fn ActionT2(comptime T1: type, comptime T2: type, comptime R: type) type {
    return struct {
        compute: *const fn (*ActionT2(T1, T2, R), T1, T2) R,
    };
}
