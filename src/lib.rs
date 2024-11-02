use std::f64::consts::PI;

pub mod camera;
pub mod color;
pub mod hittable;
pub mod hittable_list;
pub mod image;
pub mod material;
pub mod ray;
pub mod scenes;
pub mod sphere;
pub mod vec3;

fn degrees_to_radians(degrees: f64) -> f64 {
    degrees * PI / 180.0
}
