use quaternion::Quaternion;
use vector3::Vector3;

use super::Spawnee;

pub struct Player {
    base: Spawnee,
    username: Vec<u8>,
}

impl Player {
    pub fn new(id: u32, username: &[u8], spawn_pos: Vector3, spawn_rot: Quaternion<f64>) -> Self {
        let base = Spawnee::new(id, spawn_pos, spawn_rot);
        Player {
            base,
            username: username.to_vec(),
        }
    }
}
