// assumes sigmoid with mapping range [0,1]

use amethyst::core::math::Vector2;
use std::f32::consts::{PI, TAU};

pub fn map(low: f32, high: f32, v: f32) -> f32 {
    low + v * (high - low)
}

pub fn from_bool(v: bool) -> f32 {
    if v {
        1.0
    } else {
        0.0
    }
}
pub fn into_bool(v: f32) -> bool {
    v >= 0.5
}

pub fn from_vector(v: Vector2<f32>) -> [f32; 2] {
    [v.x, v.y]
}
pub fn into_vector(a: [f32; 2]) -> Vector2<f32> {
    Vector2::new(a[0] * 2.0 - 1.0, a[1] * 2.0 - 1.0)
}

pub fn to_radians(v: f32) -> f32 {
    v * TAU - PI
}
