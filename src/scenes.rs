use std::sync::Arc;

use crate::{
    color::Color,
    hittable_list::HittableList,
    material::{Dielectric, Lambertian, Metal},
    sphere::Sphere,
    vec3::Point3,
};

pub fn default_scene() -> HittableList {
    let mut world = HittableList::default();

    let material_ground = Arc::new(Lambertian {
        albedo: Color(0.8, 0.8, 0.0),
    });
    let material_center = Arc::new(Lambertian {
        albedo: Color(0.1, 0.2, 0.5),
    });

    let material_left = Arc::new(Dielectric {
        refractive_index: 1.5,
        albedo: Color(1.0, 1.0, 1.0),
    });

    let material_bubble = Arc::new(Dielectric {
        refractive_index: 1.0 / 1.5,
        ..Default::default()
    });

    let material_right = Arc::new(Metal {
        albedo: Color(0.8, 0.6, 0.2),
        fuzz: Some(0.3),
    });

    world.add(Sphere::new(
        &Point3(0.0, -100.5, -1.0),
        100.0,
        material_ground,
    ));
    world.add(Sphere::new(&Point3(0.0, 0.0, -1.2), 0.5, material_center));
    world.add(Sphere::new(&Point3(-1.0, 0.0, -1.0), 0.5, material_left));
    world.add(Sphere::new(&Point3(-1.0, 0.0, -1.0), 0.4, material_bubble));
    world.add(Sphere::new(&Point3(1.0, 0.0, -1.0), 0.5, material_right));

    return world;
}
