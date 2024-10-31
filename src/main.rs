use rtrs::{
    camera::CameraBuilder, image::ImageInfo, ray::Ray, scenes::default_scene, vec3::Point3,
};
use std::{fs::File, io::BufWriter};

fn main() {
    std::env::set_var("RUST_LOG", "debug");
    env_logger::init();

    let file = File::create("img.ppm").unwrap();
    let mut file = BufWriter::new(file);

    let world = default_scene();

    let cam = CameraBuilder::new(
        ImageInfo::from_aspect(720, 16.0 / 9.0),
        Ray::new(&Point3(0.0, 0.0, 0.0), &Point3(0.0, 0.0, -1.0)),
    )
    .samples_per_pixel(48)
    .fov(90.0)
    .max_depth(128)
    .build();

    cam.render(&mut file, &world).unwrap();
}
