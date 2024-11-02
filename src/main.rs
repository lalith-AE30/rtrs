use rtrs::{camera::CameraBuilder, image::ImageInfo, scenes::test_scene, vec3::Point3};
use std::{fs::File, io::BufWriter};

fn main() {
    std::env::set_var("RUST_LOG", "debug");
    env_logger::init();

    let file = File::create("img.ppm").unwrap();
    let mut file = BufWriter::new(file);

    let world = test_scene();

    let cam = CameraBuilder::new(
        &ImageInfo::from_aspect(144, 16.0 / 9.0),
        &Point3(13.0, 2.0, 3.0),
        &Point3(0.0, 0.0, 0.0),
    )
    .samples_per_pixel(50)
    .fov(20.0)
    .max_depth(50)
    .defocus_angle(0.6)
    .focus_dist(10.0)
    .build();

    cam.render(&mut file, &world).unwrap();
}
