#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Vec3I {
    pub x: i32,
    pub y: i32,
    pub z: i32,
}

impl Vec3I {
    pub fn new(x: i32, y: i32, z: i32) -> Self {
        Vec3I { x, y, z }
    }
}
