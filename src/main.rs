mod raytracing;

fn main() {
    let v1 = raytracing::Vector3 { x: 1.0, y: 2.0, z: 3.0 };
    let v2 = raytracing::Vector3 { x: 3.0, y: 4.0, z: 5.0 };

    let v3 = v1 + v2;

    let ray = raytracing::Ray { origin: &v1, direction: &v2 };

    println!("{}, length: {}, unit: {}, ray: {}, point: {}", v3, v3.length(), v3.as_unit_vector(), ray, ray.point_at_parameter(0.9));
}
