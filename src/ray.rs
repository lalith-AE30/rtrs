use crate::vec3::{Point3, Vec3};

#[derive(Debug, Clone, Copy)]
pub struct Ray {
    orig: Point3,
    dir: Vec3,
}

impl Default for Ray {
    fn default() -> Self {
        Self {
            orig: Default::default(),
            dir: Vec3(0.0, 0.0, 1.0),
        }
    }
}

impl Ray {
    pub fn new(origin: &Point3, direction: &Vec3) -> Self {
        Self {
            orig: *origin,
            dir: *direction,
        }
    }

    pub const fn origin(&self) -> &Point3 {
        &self.orig
    }

    pub const fn direction(&self) -> &Vec3 {
        &self.dir
    }

    pub fn at(&self, t: f64) -> Point3 {
        self.orig + t * self.dir
    }
}
