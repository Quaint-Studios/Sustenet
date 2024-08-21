pub fn Action(varargs: anytype, comptime R: type) type {
    return struct {
        compute: *const fn (varargs, *Action(R)) R,
    };
}

pub fn create(varargs: anytype, comptime R: type, compute: *const fn (varargs, *Action(R)) R) Action(varargs, R) {
    return Action(varargs, R){ .compute = compute };
}
