use crate::raytracing::{Ray, HitRecord, Vector3, Material, Hitable};

pub struct Sphere<'a> {
    pub center: Vector3,
    pub radius: f32,
    pub material: &'a dyn Material
}

impl<'a> Hitable<'a> for Sphere<'a> {
    fn hit(&self, ray: &Ray, time_min: f32, time_max: f32) -> Option<HitRecord<'a>> {
        let origin_center = ray.origin - self.center;

        let a = ray.direction.dot(&ray.direction);
        let b = origin_center.dot(&ray.direction);
        let c = origin_center.dot(&origin_center) - self.radius * self.radius;

        let discriminant = b * b - a * c;

        if discriminant > 0.0 {
            let temp = (-b - discriminant.sqrt()) / a;

            if temp < time_max && temp > time_min {
                let point = ray.point_at_parameter(temp);

                return Some(HitRecord {
                    time: temp, 
                    point: point, 
                    normal: (point - self.center) / self.radius, 
                    material: self.material
                });
            }

            let temp = (-b + discriminant.sqrt()) / a;

            if temp < time_max && temp > time_min {
                let point = ray.point_at_parameter(temp);

                return Some(HitRecord {
                    time: temp, 
                    point: point, 
                    normal: (point - self.center) / self.radius, 
                    material: self.material
                });
            }
        }

        None
    }
}
