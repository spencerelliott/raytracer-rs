mod raytracing;
mod materials;
mod random;

fn main() {
    let v1 = raytracing::Vector3 { x: 1.0, y: 2.0, z: 3.0 };
    let v2 = raytracing::Vector3 { x: 3.0, y: 4.0, z: 5.0 };

    let v3 = v1 + v2;

    let ray = raytracing::Ray { origin: v1.clone(), direction: v2.clone() };

    let _test_rand_01 = random::random_f32_0_to_1();
    let _test_rand = random::random_f32();
    let _test_in_sphere = random::random_in_unit_sphere();

    println!("{}, length: {}, unit: {}, ray: {}, point: {}", v3, v3.length(), v3.as_unit_vector(), ray, ray.point_at_parameter(0.9));
}
