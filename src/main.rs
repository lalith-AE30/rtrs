use rtrs::{
    camera::CameraBuilder,
    color::Color,
    hittable_list::HittableList,
    material::{Dielectric, Lambertian, Metal},
    ray::Ray,
    sphere::Sphere,
    vec3::{Point3, Vec3},
};
use std::{fs::File, io::BufWriter, sync::Arc};

fn main() {
    std::env::set_var("RUST_LOG", "debug");
    env_logger::init();

    let file = File::create("img.ppm").unwrap();
    let mut file = BufWriter::new(file);

    let mut world = HittableList::default();

    let material_ground = Arc::new(Lambertian::new(&Color::new(0.8, 0.8, 0.0)));
    let material_center = Arc::new(Lambertian::new(&Color::new(0.1, 0.2, 0.5)));

    let material_right = Arc::new(Metal::new(&Color::new(0.8, 0.6, 0.2), Some(0.1)));

    let material_left = Arc::new(Dielectric::new(1.5));
    let material_bubble = Arc::new(Dielectric::new(1.0 / 1.5));

    world.add(Arc::new(Sphere::new(
        &Point3::new(0.0, -100.5, -1.0),
        100.0,
        material_ground,
    )));
    world.add(Arc::new(Sphere::new(
        &Point3::new(0.0, 0.0, -1.2),
        0.5,
        material_center,
    )));
    world.add(Arc::new(Sphere::new(
        &Point3::new(-1.0, 0.0, -1.0),
        0.5,
        material_left,
    )));
    world.add(Arc::new(Sphere::new(
        &Point3::new(-1.0, 0.0, -1.0),
        0.4,
        material_bubble,
    )));
    world.add(Arc::new(Sphere::new(
        &Point3::new(1.0, 0.0, -1.0),
        0.5,
        material_right,
    )));

    let cam = CameraBuilder::default()
        .view_ray(Ray::new(&Vec3::default(), &Vec3::new(0.0, 0.0, -1.0)))
        .build()
        .unwrap();

    cam.render(&mut file, &world).unwrap();
}
