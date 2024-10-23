use quaternion::Quaternion;
use vector3::Vector3;

pub struct Spawnee {
    id: u32,
    position: Vector3,
    rotation: Quaternion<f64>,
}

impl Spawnee {
    pub fn new(id: u32, position: Vector3, rotation: Quaternion<f64>) -> Self {
        Spawnee {
            id,
            position,
            rotation,
        }
    }
}
