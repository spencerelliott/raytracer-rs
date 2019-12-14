mod camera;
mod geometry;
mod materials;
mod random;
mod raytracing;

fn main() {
    let v1 = raytracing::Vector3 {
        x: 1.0,
        y: 2.0,
        z: 3.0,
    };

    let v2 = raytracing::Vector3 {
        x: 3.0,
        y: 4.0,
        z: 5.0,
    };

    let v3 = v1 + v2;

    let ray = raytracing::Ray {
        origin: v1.clone(),
        direction: v2.clone(),
    };

    let _test_rand_01 = random::random_f32_0_to_1();
    let _test_rand = random::random_f32();
    let _test_in_sphere = random::random_in_unit_sphere();

    let lower_left = raytracing::Vector3 {
        x: -2.0,
        y: -1.0,
        z: -1.0,
    };

    let horizontal = raytracing::Vector3 {
        x: 4.0,
        y: 0.0,
        z: 0.0,
    };

    let vertical = raytracing::Vector3 {
        x: 0.0,
        y: 2.0,
        z: 0.0,
    };

    let origin = raytracing::Vector3 {
        x: 0.0,
        y: 0.0,
        z: 0.0,
    };

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

    let hitables = vec![
        &lambertian as &dyn raytracing::Hitable,
        &lambertian as &dyn raytracing::Hitable,
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
        320.0 / 280.0,
    );

    raytracing::check_for_hit(&hitables, &ray, 0.001, 3000.0);

    println!(
        "{}, length: {}, unit: {}, ray: {}, point: {}",
        v3,
        v3.length(),
        v3.as_unit_vector(),
        ray,
        ray.point_at_parameter(0.9)
    );
}
