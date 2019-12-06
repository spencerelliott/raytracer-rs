use std::ops;

pub struct Vector3 {
    pub x: f32,
    pub y: f32,
    pub z: f32
}

impl ops::Add<Vector3> for Vector3 {
    type Output = Vector3;

    fn add(self, _rhs: Vector3) -> Vector3 {
        let ret = Vector3 { x: self.x + _rhs.x,  y: self.y + _rhs.y, z: self.z + _rhs.z };

        return ret;
    }
}