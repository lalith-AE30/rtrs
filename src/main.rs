use rtrs::{
    camera::CameraBuilder,
    color::Color,
    hittable_list::HittableList,
    image::ImageInfo,
    material::{Dielectric, Lambertian, Metal},
    ray::Ray,
    sphere::Sphere,
    vec3::Point3,
};
use std::{fs::File, io::BufWriter, sync::Arc};

fn main() {
    std::env::set_var("RUST_LOG", "debug");
    env_logger::init();

    let file = File::create("img.ppm").unwrap();
    let mut file = BufWriter::new(file);

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

    world.add(Arc::new(Sphere::new(
        &Point3(0.0, -100.5, -1.0),
        100.0,
        material_ground,
    )));
    world.add(Arc::new(Sphere::new(
        &Point3(0.0, 0.0, -1.2),
        0.5,
        material_center,
    )));
    world.add(Arc::new(Sphere::new(
        &Point3(-1.0, 0.0, -1.0),
        0.5,
        material_left,
    )));
    world.add(Arc::new(Sphere::new(
        &Point3(-1.0, 0.0, -1.0),
        0.4,
        material_bubble,
    )));
    world.add(Arc::new(Sphere::new(
        &Point3(1.0, 0.0, -1.0),
        0.5,
        material_right,
    )));

    let cam = CameraBuilder::new(
        ImageInfo::from_aspect(720, 16.0 / 9.0),
        Ray::new(&Point3(2.0, 2.0, 1.0), &Point3(0.0, 0.0, -1.0)),
    )
    .samples_per_pixel(48)
    .fov(30.0)
    .max_depth(128)
    .build();

    cam.render(&mut file, &world).unwrap();
}
