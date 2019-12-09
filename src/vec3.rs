use std::ops;
use std::fmt;

#[derive(Copy, Clone)]
pub struct Vector3 {
    pub x: f32,
    pub y: f32,
    pub z: f32
}

impl fmt::Display for Vector3 {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_fmt(format_args!("Vec3[{}, {}, {}]", self.x, self.y, self.z))
    }
}

impl Vector3 {
    pub fn length(self) -> f32 {
        (self.x * self.x) + (self.y * self.y) + (self.z * self.z)
    }

    pub fn squared_length(self) -> f32 {
        self.length().sqrt()
    }

    pub fn dot(self, _rhs: Vector3) -> f32 {
        self.x * _rhs.x + self.y * _rhs.y + self.z * _rhs.z
    }

    pub fn cross(self, _rhs: Vector3) -> Vector3 {
        Vector3 {
            x: self.y * _rhs.z - self.z * _rhs.x,
            y: -(self.x * _rhs.z - self.z * _rhs.x),
            z: self.x * _rhs.y - self.y * _rhs.x
        }
    }

    pub fn as_unit_vector(self) -> Vector3 {
        let k = 1.0 / self.length();

        Vector3 {
            x: self.x * k,
            y: self.y * k,
            z: self.z * k
        }
    }
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

impl ops::Mul<f32> for Vector3 {
    type Output = Vector3;

    fn mul(self, _rhs: f32) -> Vector3 {
        Vector3 {
            x: self.x * _rhs,
            y: self.y * _rhs,
            z: self.z * _rhs
        }
    }
}

impl ops::Div for Vector3 {
    type Output = Vector3;

    fn div(self, _rhs: Vector3) -> Vector3 {
        Vector3 { x: self.x / _rhs.x, y: self.y / _rhs.y, z: self.z / _rhs.z }
    }
}

impl ops::Div<f32> for Vector3 {
    type Output = Vector3;

    fn div(self, _rhs: f32) -> Vector3 {
        Vector3 {
            x: self.x / _rhs,
            y: self.y / _rhs,
            z: self.z / _rhs
        }
    }
}
