use std::sync::Arc;

use crate::{
    color::Color,
    hittable_list::HittableList,
    material::{Dielectric, Lambertian, Material, Metal},
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

    world
}

pub fn test_scene() -> HittableList {
    let mut world = HittableList::default();

    let ground_material = Arc::new(Lambertian {
        albedo: Color(0.5, 0.5, 0.5),
    });
    world.add(Sphere::new(
        &Point3(0.0, -1000.0, 0.0),
        1000.0,
        ground_material,
    ));

    for a in -11..11 {
        for b in -11..11 {
            let choose_mat = fastrand::f64();
            let center = Point3(
                a as f64 + 0.9 * fastrand::f64(),
                0.2,
                b as f64 + 0.9 * fastrand::f64(),
            );

            if (center - Point3(4.0, 0.2, 0.0)).length() > 0.9 {
                let sphere_material: Arc<dyn Material>;

                if choose_mat < 0.8 {
                    let albedo = Color::random(None) * Color::random(None);
                    sphere_material = Arc::new(Lambertian { albedo });
                } else if choose_mat < 0.95 {
                    let albedo = Color::random(Some(0.5..=1.0));
                    let fuzz = fastrand::f64() * 0.5;
                    sphere_material = Arc::new(Metal {
                        albedo,
                        fuzz: Some(fuzz),
                    })
                } else {
                    sphere_material = Arc::new(Dielectric {
                        refractive_index: 1.5,
                        ..Default::default()
                    });
                }
                world.add(Sphere::new(&center, 0.2, sphere_material));
            }
        }
    }

    let material1 = Arc::new(Dielectric {
        refractive_index: 1.5,
        ..Default::default()
    });
    world.add(Sphere::new(&Point3(0.0, 1.0, 0.0), 1.0, material1));

    let material2 = Arc::new(Lambertian {
        albedo: Color(0.4, 0.2, 0.1),
    });
    world.add(Sphere::new(&Point3(-4.0, 1.0, 0.0), 1.0, material2));

    let material3 = Arc::new(Metal {
        albedo: Color(0.7, 0.6, 0.5),
        fuzz: None,
    });
    world.add(Sphere::new(&Point3(4.0, 1.0, 0.0), 1.0, material3));

    world
}
