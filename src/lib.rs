use std::ops::RangeInclusive;

pub mod camera;
pub mod color;
pub mod hittable;
pub mod hittable_list;
pub mod material;
pub mod ray;
pub mod sphere;
pub mod vec3;
pub mod image;

const PI: f64 = 3.1415926535897932385;

#[allow(unused)]
fn degrees_to_radians(degrees: f64) -> f64 {
    degrees * PI / 180.0
}

pub trait ClampExt<T> {
    fn clamp(&self, x: T) -> T;
}

impl<T> ClampExt<T> for RangeInclusive<T>
where
    T: PartialOrd + Copy,
{
    fn clamp(&self, x: T) -> T {
        if x < *self.start() {
            return *self.start();
        } else if x > *self.end() {
            return *self.end();
        }
        return x;
    }
}
