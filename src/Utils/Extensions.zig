//! Extensions for various things you might find useful from other languages.

//#region Action
/// A Zig implmenetation of C#'s `Action`.
const Action = struct {
    invoke: fn () void,

    pub fn create(callable: fn () void) Action {
        return Action{ .invoke = callable };
    }
};

/// A Zig implmenetation of C#'s `Action<T>`.
const ActionT = struct {
    invoke: fn (comptime T: type) void,

    pub fn create(callable: fn (T: type) void) ActionT {
        return ActionT{ .invoke = callable };
    }
};

/// A Zig implmenetation of C#'s `Action<T1, T2>`.
const ActionT1T2 = struct {
    invoke: fn (comptime T1: type, comptime T2: type) void,

    pub fn create(callable: fn (T1: type, T2: type) void) ActionT1T2 {
        return ActionT1T2{ .invoke = callable };
    }
};
//#endregion
