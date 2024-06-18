pub mod camera;
pub mod color;
pub mod hittable;
pub mod hittable_list;
pub mod image;
pub mod material;
pub mod ray;
pub mod sphere;
pub mod vec3;

const PI: f64 = 3.1415926535897932385;

fn degrees_to_radians(degrees: f64) -> f64 {
    degrees * PI / 180.0
}
