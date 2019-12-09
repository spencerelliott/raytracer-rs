mod vec3;

fn main() {
    let v1 = vec3::Vector3 { x: 1.0, y: 2.0, z: 3.0 };
    let v2 = vec3::Vector3 { x: 3.0, y: 4.0, z: 5.0 };

    let v3 = v1 + v2;

    println!("{}, length: {}, unit: {}", v3, v3.length(), v3.as_unit_vector());
}
