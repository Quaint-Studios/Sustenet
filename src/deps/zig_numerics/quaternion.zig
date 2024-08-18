const vec = @import("Vector.zig");

pub fn Quaternion(comptime Scalar: type) type {
    return extern struct {
        v: vec.Vec3(Scalar),

        pub const Vector = vec.Vec3(Scalar);

        pub const T = Vector.T;
    };
}
