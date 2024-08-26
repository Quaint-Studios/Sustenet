//! This is a collection of implementations for C#'s Action in Zig.

pub fn Action(comptime R: type) type {
    return struct {
        compute: *const fn (*Action(R)) R,
    };
}

pub fn ActionT1(comptime T1: type, comptime R: type) type {
    return struct {
        compute: *const fn (*ActionT1(T1, R), T1) R,
    };
}

pub fn ActionT2(comptime T1: type, comptime T2: type, comptime R: type) type {
    return struct {
        compute: *const fn (*ActionT2(T1, T2, R), T1, T2) R,
    };
}
