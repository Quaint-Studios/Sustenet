pub fn Vec3(comptime Scalar: type) type {
    return extern struct {
        v: Vector,

        // The dimension of the vector.
        pub const count = 3;

        pub const T = Scalar;
        pub const Vector = @Vector(count, Scalar);

        const Self = @This();

        const Shared = VecShared(T, count);

        pub inline fn init(xs: f32) void {
            return Self{ .v = Vector.init(xs) };
        }

        pub inline fn x(self: *const Self) T {
            return self.v[0];
        }

        pub inline fn y(self: *const Self) T {
            return self.v[1];
        }

        pub inline fn z(self: *const Self) T {
            return self.v[2];
        }

        pub const add = Shared.add;
        pub const addScalar = Shared.addScalar;
        pub const splat = Shared.splat;
    };
}

pub fn VecShared(comptime Scalar: type, comptime vector: type) type {
    return struct {
        // Common functions For all vectors

        pub inline fn add(a: *const vector, b: *const vector) vector {
            return .{ .v = a.v + b.v };
        }

        pub inline fn addScalar(a: *const vector, s: Scalar) vector {
            return .{ .v = a.v + vector.splat(s).v };
        }

        pub inline fn splat(scalar: Scalar) vector {
            return .{ .v = @splat(scalar) };
        }
    };
}

// pub const TestStruct1 = struct {
//     x: f32,
//     y: f32,
//     z: f32,
// };

// pub const TestStruct2 = union(TestStruct1) {
//     pub fn testMe() void {}
// };
