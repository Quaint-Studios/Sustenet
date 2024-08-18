const sustenet = @import("root").sustenet;
const Vector3 = sustenet.zig_numerics.vector.Vec3(f32);

const Spawnee = sustenet.core.spawning.Spawnee;
const Player = @This();

super: Spawnee,
username: []u8,

pub fn new(id: u32, username: []const u8, spawn_position: Vector3) Player {
    const player = Player{
        .super = Spawnee.new(id, spawn_position),
        .username = username,
    };
    return player;
}
