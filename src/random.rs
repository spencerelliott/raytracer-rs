extern crate rand;

use rand::{thread_rng, Rng};
use crate::raytracing::Vector3;

pub fn random_f32() -> f32 {
    let mut rng = thread_rng();
    rng.gen_range(-1.0, 1.0)
}

pub fn random_f32_0_to_1() -> f32 {
    let mut rng = thread_rng();
    rng.gen_range(0.0, 1.0)
}

pub fn random_in_unit_sphere() -> Vector3 {
    let mut in_sphere: Vector3;

    while {
        in_sphere = 2.0 * Vector3 {x: random_f32(), y: random_f32(), z: random_f32()};
        in_sphere.length() >= 1.0
    } {}

    in_sphere
}