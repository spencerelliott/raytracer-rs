use crate::raytracing::{Vector3, Ray};

pub struct Camera {
    pub origin: Vector3,
    pub lower_left: Vector3,
    pub horizontal: Vector3,
    pub vertical: Vector3
}

impl Camera {
    pub fn new(look_from: Vector3, look_at: Vector3, up: Vector3, fov: f32, aspect: f32) -> Camera {
        let theta = fov * std::f32::consts::PI / 180.0;
        let half_height = (theta / 2.0).tan();
        let half_width = aspect * half_height;

        let w = (look_from - look_at).as_unit_vector();
        let u = up.cross(w).as_unit_vector();
        let v = w.cross(u);

        Camera {
            origin: look_from,
            lower_left: look_from - half_width * u - half_height * v - w,
            horizontal: 2.0 * half_width * u,
            vertical: 2.0 * half_height * v
        }
    }

    pub fn get_ray(&self, u: f32, v: f32) -> Ray {
        Ray {
            origin: self.origin,
            direction: self.lower_left + u * self.horizontal + v * self.vertical - self.origin,
        }
    }
}