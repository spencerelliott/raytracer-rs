extern crate oidn;

mod camera;
mod geometry;
mod materials;
mod random;
mod raytracing;
mod exporter;

use std::time::SystemTime;

use exporter::Exporter;

pub const WIDTH: i32 = 640;
pub const HEIGHT: i32 = 320;
const SAMPLES: i32 = 5;
const MAX_DEPTH: i32 = 5;

fn get_screen_space_color<'a>(
    ray: &raytracing::Ray,
    hitables: &Vec<&dyn raytracing::Hitable<'a>>,
    depth: i32,
) -> raytracing::Vector3 {
    if let Some(hit_record) = raytracing::check_for_hit(hitables, ray, 0.001, std::f32::MAX) {
        if let Some((scattered, attenuation)) = hit_record.material.scatter(ray, &hit_record) {
            if depth < MAX_DEPTH {
                return attenuation * get_screen_space_color(&scattered, hitables, depth + 1);
            } else {
                return raytracing::Vector3::ZERO;
            }
        } else {
            return raytracing::Vector3 {
                x: 0.0,
                y: 0.0,
                z: 0.0,
            };
        }
    } else {
        let unit_direction = ray.direction.as_unit_vector();
        let time = 0.5 * (unit_direction.y + 1.0);

        return (1.0 - time) * raytracing::Vector3::ONE
            + time
                * raytracing::Vector3 {
                    x: 0.5,
                    y: 0.7,
                    z: 1.0,
                };
    }
}

fn main() -> std::io::Result<()> {
    let lambertian_sphere = geometry::Sphere {
        center: raytracing::Vector3 {
            x: -0.3,
            y: 0.0,
            z: -1.0,
        },
        radius: 0.5,
        material: &materials::Lambertian {
            albedo: raytracing::Vector3 {
                x: 0.8,
                y: 0.3,
                z: 0.3,
            },
        },
    };

    let metal_sphere = geometry::Sphere {
        center: raytracing::Vector3 {
            x: 1.0,
            y: 0.0,
            z: -1.3,
        },
        radius: 0.5,
        material: &materials::Metal {
            albedo: raytracing::Vector3 {
                x: 0.8,
                y: 0.6,
                z: 0.3,
            },
            fuzz: 0.01,
        },
    };

    let dielectric_sphere = geometry::Sphere {
        center: raytracing::Vector3 {
            x: 0.5,
            y: 0.8,
            z: -1.5,
        },
        radius: 0.5,
        material: &materials::Dielectric {
            refraction_index: 2.0,
        },
    };

    let floor = geometry::Sphere {
        center: raytracing::Vector3 {
            x: 0.0,
            y: -100.5,
            z: -1.0,
        },
        radius: 100.0,
        material: &materials::Lambertian {
            albedo: raytracing::Vector3 {
                x: 0.8,
                y: 0.8,
                z: 0.0,
            },
        },
    };

    let hitables = vec![
        &lambertian_sphere as &dyn raytracing::Hitable,
        &metal_sphere as &dyn raytracing::Hitable,
        &dielectric_sphere as &dyn raytracing::Hitable,
        &floor as &dyn raytracing::Hitable,
    ];

    let camera = camera::Camera::new(
        raytracing::Vector3 {
            x: 0.0,
            y: 0.4,
            z: 0.5,
        },
        raytracing::Vector3 {
            x: 0.0,
            y: 0.0,
            z: -1.0,
        },
        raytracing::Vector3 {
            x: 0.0,
            y: 1.0,
            z: 0.0,
        },
        90.0,
        WIDTH as f32 / HEIGHT as f32,
    );

    let start_time = SystemTime::now();

    let pixel_export = &mut exporter::P3Exporter::new(WIDTH, HEIGHT, "out.ppm".to_string());
    //let pixel_export = &mut exporter::OidnExporter::new(WIDTH, HEIGHT, "out.jpg".to_string());
    
    for y_pixel in (0..HEIGHT).rev() {
        for x_pixel in 0..WIDTH {
            let mut color = raytracing::Vector3::ZERO;

            for _sample in 0..SAMPLES {
                let u = (x_pixel as f32 + random::random_f32()) / WIDTH as f32;
                let v = (y_pixel as f32 + random::random_f32()) / HEIGHT as f32;

                let ray = camera.get_ray(u, v);

                color = color + get_screen_space_color(&ray, &hitables, 0);
            }

            let color = color / SAMPLES as f32;
            let color = raytracing::Vector3 {
                x: color.x.sqrt(),
                y: color.y.sqrt(),
                z: color.z.sqrt(),
            }
            .modify(|value: f32| value * 255.99);

            pixel_export.push_pixel(color.x, color.y, color.z);

            print!("{:04}x{:04}\r", x_pixel, y_pixel);
        }
    }

    pixel_export.finish();

    let end_time = SystemTime::now();

    println!(
        "It took {} seconds to generate the image",
        end_time
            .duration_since(start_time)
            .expect("Time went backwards")
            .as_secs()
    );

    Ok(())
}
