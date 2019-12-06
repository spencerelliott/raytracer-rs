use std::ops;

pub struct Vector3 {
    pub x: f32,
    pub y: f32,
    pub z: f32
}

impl ops::Add for Vector3 {
    type Output = Vector3;

    fn add(self, _rhs: Vector3) -> Vector3 {
        Vector3 { x: self.x + _rhs.x,  y: self.y + _rhs.y, z: self.z + _rhs.z }
    }
}

impl ops::Sub for Vector3 {
    type Output = Vector3;

    fn sub(self, _rhs: Vector3) -> Vector3 {
        Vector3 { x: self.x - _rhs.x, y: self.y - _rhs.y, z: self.z - _rhs.z }
    }
}

impl ops::Mul for Vector3 {
    type Output = Vector3;

    fn mul(self, _rhs: Vector3) -> Vector3 {
        Vector3 { x: self.x * _rhs.x, y: self.y * _rhs.y, z: self.z * _rhs.z }
    }
}

impl ops::Div for Vector3 {
    type Output = Vector3;

    fn div(self, _rhs: Vector3) -> Vector3 {
        Vector3 { x: self.x / _rhs.x, y: self.y / _rhs.y, z: self.z / _rhs.z }
    }
}