mod camera;
mod geometry;
mod materials;
mod random;
mod raytracing;

use std::fs::File;
use std::io::prelude::*;

const WIDTH: i32 = 320;
const HEIGHT: i32 = 180;
const SAMPLES: i32 = 2;

fn get_screen_space_color<'a>(
    ray: &raytracing::Ray,
    hitables: &Vec<&dyn raytracing::Hitable<'a>>,
    depth: i32,
) -> raytracing::Vector3 {
    if let Some(hit_record) = raytracing::check_for_hit(hitables, ray, 0.001, std::f32::MAX) {
        if let Some((scattered, attenuation)) = hit_record.material.scatter(ray, &hit_record) {
            if depth < 50 {
                return attenuation * get_screen_space_color(ray, hitables, depth + 1);
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
    let lambertian = geometry::Sphere {
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
                z: 0.8,
            },
        },
    };

    let hitables = vec![
        &lambertian as &dyn raytracing::Hitable,
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
        (WIDTH / HEIGHT) as f32,
    );

    let mut p3_file = File::create("out.ppm")?;
    p3_file.write_fmt(format_args!("P3\n{} {}\n255\n", WIDTH, HEIGHT))?;

    for y_pixel in (0..HEIGHT).rev() {
        for x_pixel in 0..WIDTH {
            let mut color = raytracing::Vector3::ZERO;

            for _sample in 0..SAMPLES {
                let u = x_pixel as f32 + random::random_f32() / WIDTH as f32;
                let v = y_pixel as f32 + random::random_f32() / HEIGHT as f32;

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

            p3_file.write_fmt(format_args!(
                "{} {} {}\n",
                color.x as i32, color.y as i32, color.z as i32
            ))?;
        }
    }

    Ok(())
}
