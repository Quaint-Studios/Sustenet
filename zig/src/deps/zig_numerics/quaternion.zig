const vec = @import("vector.zig");

pub fn Quaternion(comptime Scalar: type) type {
    return extern struct {
        v: vec.Vec3(Scalar),

        pub const Vector = vec.Vec3(Scalar);

        pub const T = Vector.T;
    };
}
