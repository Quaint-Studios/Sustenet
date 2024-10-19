const sustenet = @import("root").sustenet;
const zig_numerics = sustenet.zig_numerics;

const Vector3 = zig_numerics.vector.Vec3(f32);
const Quaternion = zig_numerics.quaternion.Quaternion(f32);
const Spawnee = @This();

id: u32,
position: Vector3,
rotation: Quaternion,

pub fn new(id: u32, position: Vector3, rotation: Quaternion) Spawnee {
    return Spawnee{
        .id = id,
        .position = position,
        .rotation = rotation,
    };
}
