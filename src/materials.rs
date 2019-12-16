use crate::raytracing::{Material, Vector3, Ray, HitRecord};
use crate::random::{random_in_unit_sphere, random_f32_0_to_1};

fn reflect(vector: &Vector3, normal: &Vector3) -> Vector3 {
    *vector - 2.0 * vector.dot(normal) * *normal
}

fn refract(vector: &Vector3, normal: &Vector3, ni_over_nt: f32) -> Option<Vector3> {
    let uv = vector.as_unit_vector();
    let dt = uv.dot(normal);
    let discriminant = 1.0 - ni_over_nt.powi(2) * (1.0 - dt.powi(2));

    if discriminant > 0.0 {
        Some(ni_over_nt * (uv - *normal * dt) - *normal * discriminant.sqrt())
    } else {
        None
    }
}

fn schlick(cosine: f32, refraction_index: f32) -> f32 {
    let r0 = ((1.0 - refraction_index) / (1.0 + refraction_index)).powi(2);

    r0 + (1.0 - r0) * (1.0 - cosine).powi(5)
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
        let outward_normal: Vector3;
        let ni_over_nt: f32;
        let cosine: f32;

        let attenuation = Vector3::ONE;

        let reflected = reflect(&ray.direction, &hit_record.normal);

        if ray.direction.dot(&hit_record.normal) > 0.0 {
            outward_normal = hit_record.normal * -1.0;
            ni_over_nt = self.refraction_index;
            cosine = self.refraction_index * ray.direction.dot(&hit_record.normal) / ray.direction.length();
        } else {
            outward_normal = hit_record.normal;
            ni_over_nt = 1.0 / self.refraction_index;
            cosine = -ray.direction.dot(&hit_record.normal) / ray.direction.length();
        }

        if let Some(refracted) = refract(&ray.direction, &outward_normal, ni_over_nt) {
            Some((Ray {
                origin: hit_record.point,
                direction: if random_f32_0_to_1() < schlick(cosine, self.refraction_index) { refracted } else { reflected }
            }, attenuation))
        } else {
            Some((Ray {
                origin: hit_record.point,
                direction: reflected,
            }, attenuation))
        }
    }
}