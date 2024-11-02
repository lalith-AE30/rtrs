use crate::{
    hittable::{HitRecord, Hittable},
    material::Material,
    vec3::{dot, Point3},
};
use std::sync::Arc;

pub struct Sphere {
    center: Point3,
    radius: f64,
    mat: Arc<dyn Material>,
}

impl Sphere {
    pub fn new(center: &Point3, radius: f64, mat: Arc<dyn Material>) -> Arc<Self> {
        assert!(radius >= 0.0, "Radius must be positive");
        Arc::new(Self {
            center: *center,
            radius,
            mat,
        })
    }
}

impl Hittable for Sphere {
    fn hit(
        &self,
        r: &crate::ray::Ray,
        interval: std::ops::RangeInclusive<f64>,
        rec: &mut HitRecord,
    ) -> bool {
        let oc = self.center - r.origin();
        let a = r.direction().length_squared();
        let h = dot(r.direction(), &oc);
        let c = oc.length_squared() - self.radius * self.radius;

        let discriminant = h * h - a * c;

        if discriminant < 0.0 {
            return false;
        }

        let sqrtd = discriminant.sqrt();

        let mut root = (h - sqrtd) / a;
        if !interval.contains(&root) {
            root = (h + sqrtd) / a;
            if !interval.contains(&root) {
                return false;
            }
        }

        rec.t = root;
        rec.p = r.at(rec.t);
        let outward_normal = (rec.p - self.center) / self.radius;
        rec.set_face_normal(r, &outward_normal);
        rec.mat = Some(self.mat.clone());

        true
    }
}
