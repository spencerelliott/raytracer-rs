use crate::raytracing::{Material, Vector3, Ray, HitRecord};
use crate::random::random_in_unit_sphere;

struct Lambertian {
    albedo: Vector3
}

impl Material for Lambertian {
    fn scatter(self, ray: &Ray, hit_record: &HitRecord, attenuation: &mut Vector3, scattered: &mut Ray) -> bool {
        let target = hit_record.point + hit_record.normal + random_in_unit_sphere();

        scattered.origin = hit_record.point.clone();
        scattered.direction = target - hit_record.normal;

        attenuation.x = self.albedo.x.clone();
        attenuation.y = self.albedo.y.clone();
        attenuation.z = self.albedo.z.clone();

        return true;
    }
}

struct Metal {
    albedo: Vector3
}

impl Material for Metal {
    fn scatter(self, ray: &Ray, hit_record: &HitRecord, attenuation: &mut Vector3, scattered: &mut Ray) -> bool {
        return true;
    }
}

struct Dielectric {
    refraction_index: f32
}

impl Material for Dielectric {
    fn scatter(self, ray: &Ray, hit_record: &HitRecord, attenuation: &mut Vector3, scattered: &mut Ray) -> bool {
        return true;
    }
}