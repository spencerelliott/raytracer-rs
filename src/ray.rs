use crate::vec3::Vector3;

pub struct Ray {
    pub origin: Vector3,
    pub direction: Vector3
}

impl Ray {
    fn point_at_parameter(self, time: f32) -> Vector3 {
        self.origin + (self.direction * time)
    }
}
