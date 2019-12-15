use crate::raytracing::{Material, Vector3, Ray, HitRecord};
use crate::random::random_in_unit_sphere;

fn reflect(vector: &Vector3, normal: &Vector3) -> Vector3 {
    *vector - 2.0 * vector.dot(normal) * *normal
}

pub struct Lambertian {
    pub albedo: Vector3
}

impl Material for Lambertian {
    fn scatter(&self, _ray: &Ray, hit_record: &HitRecord) -> Option<(Ray, Vector3)> {
        let target = hit_record.point + hit_record.normal + random_in_unit_sphere();

        Some((Ray {origin: hit_record.point, direction: target - hit_record.point}, self.albedo))
    }
}

pub struct Metal {
    pub albedo: Vector3,
    pub fuzz: f32
}

impl Material for Metal {
    fn scatter(&self, ray: &Ray, hit_record: &HitRecord) -> Option<(Ray, Vector3)> {
        let reflected = reflect(&ray.direction.as_unit_vector(), &hit_record.normal);
        let scatter = Ray{origin: hit_record.point, direction: reflected + self.fuzz * random_in_unit_sphere()};

        if scatter.direction.dot(&hit_record.normal) > 0.0 {
            Some((scatter, self.albedo))
        } else {
            None
        }
    }
}

pub struct Dielectric {
    pub refraction_index: f32
}

impl Material for Dielectric {
    fn scatter(&self, ray: &Ray, hit_record: &HitRecord) -> Option<(Ray, Vector3)> {
        None
    }
}