use std::{ops::RangeInclusive, sync::Arc};

use crate::{
    material::Material, ray::Ray, vec3::{dot, Point3, Vec3}
};

#[derive(Default, Clone)]
pub struct HitRecord {
    pub p: Point3,
    pub t: f64,
    pub normal: Vec3,
    pub mat: Option<Arc<dyn Material>>,
    pub front_face: bool,
}

impl HitRecord {
    pub fn set_face_normal(&mut self, r: &Ray, outward_normal: &Vec3) {
        self.front_face = dot(r.direction(), outward_normal) < 0.0;
        self.normal = if self.front_face {
            *outward_normal
        } else {
            -*outward_normal
        };
    }
}

pub trait Hittable {
    fn hit(&self, r: &Ray, interval: RangeInclusive<f64>, rec: &mut HitRecord) -> bool;
}
