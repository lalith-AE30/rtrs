use crate::{
    color::Color,
    degrees_to_radians,
    hittable::{HitRecord, Hittable},
    image::ImageInfo,
    ray::Ray,
    vec3::{unit_vector, Point3, Vec3},
};
use derive_builder::Builder;
use indicatif::{ProgressBar, ProgressStyle};
use std::io::{Error, Write};

#[derive(Default, Debug, Clone, Copy, Builder)]
#[builder(build_fn(skip))]
#[builder(setter(skip))]
pub struct Camera {
    #[builder(setter)]
    pub image_info: ImageInfo,
    #[builder(setter)]
    pub samples_per_pixel: u32,
    #[builder(setter)]
    pub max_depth: u32,
    #[builder(setter)]
    pub fov: f64,
    center: Point3,
    pixel00_loc: Point3,
    pixel_delta_u: Vec3,
    pixel_delta_v: Vec3,
    pixel_samples_scale: f64,
}

#[allow(dead_code)]
impl CameraBuilder {
    pub fn build(&self) -> Result<Camera, CameraBuilderError> {
        let mut obj = Camera::default();
        obj.image_info = Clone::clone(self.image_info.as_ref().unwrap_or(&ImageInfo::default()));
        obj.samples_per_pixel = Clone::clone(self.samples_per_pixel.as_ref().unwrap_or(&100));
        obj.max_depth = Clone::clone(self.max_depth.as_ref().unwrap_or(&8));
        obj.fov = Clone::clone(self.fov.as_ref().unwrap_or(&90.0));

        obj.initialize(obj.image_info, obj.samples_per_pixel, obj.fov);

        Ok(obj)
    }
}

impl Camera {
    pub fn initialize(&mut self, image_info: ImageInfo, samples_per_pixel: u32, fov: f64) {
        let img = image_info;

        self.pixel_samples_scale = 1.0 / samples_per_pixel as f64;

        // Camera parameter
        let focal_length = 1.0;
        let theta = degrees_to_radians(fov);
        let h = (theta / 2.0).tan();
        let viewport_height = 2.0 * h * focal_length;
        let viewport_width = viewport_height * (img.image_width as f64 / img.image_height as f64);
        let camera_center = Point3::new(0.0, 0.0, 0.0);

        let (viewport_u, viewport_v) = (
            Vec3::new(viewport_width, 0.0, 0.0),
            Vec3::new(0.0, -viewport_height, 0.0),
        );

        (self.pixel_delta_u, self.pixel_delta_v) = (
            viewport_u / img.image_width as f64,
            viewport_v / img.image_height as f64,
        );

        let viewport_upper_left =
            camera_center - Vec3::new(0.0, 0.0, focal_length) - viewport_u / 2.0 - viewport_v / 2.0;
        self.pixel00_loc = viewport_upper_left + 0.5 * (self.pixel_delta_u + self.pixel_delta_v);
    }

    pub fn render(&self, file: &mut dyn Write, world: &dyn Hittable) -> Result<(), Error> {
        let (img, samples_per_pixel, max_depth) =
            (self.image_info, self.samples_per_pixel, self.max_depth);
        file.write(format!("P3\n{} {}\n255\n", img.image_width, img.image_height).as_bytes())?;

        let bar = {
            let style = ProgressStyle::with_template(
                "[{elapsed_precise}] {bar:40.cyan/blue} {pos:>7}/{len:7} {msg}",
            )
            .unwrap()
            .progress_chars("#>.");
            let ibar = ProgressBar::new(img.image_height as u64);
            ibar.set_style(style);
            ibar
        };

        for j in 0..img.image_height {
            for i in 0..img.image_width {
                let (i, j) = (i as f64, j as f64);
                let mut pixel_color = Color::default();
                for _ in 0..samples_per_pixel {
                    let r = self.get_ray(i, j);
                    pixel_color += Camera::ray_color(&r, max_depth, world);
                }
                crate::color::write_color(file, &(self.pixel_samples_scale * pixel_color))?;
            }
            bar.inc(1);
        }

        bar.finish();
        Ok(())
    }

    fn sample_square() -> Vec3 {
        Vec3::new(fastrand::f64() - 0.5, fastrand::f64() - 0.5, 0.0)
    }

    fn get_ray(&self, i: f64, j: f64) -> Ray {
        let offset = Camera::sample_square();
        let pixel_sample = self.pixel00_loc
            + ((i + offset.x()) * self.pixel_delta_u)
            + ((j + offset.y()) * self.pixel_delta_v);

        let ray_origin = self.center;
        let ray_direction = pixel_sample - ray_origin;

        Ray::new(&ray_origin, &ray_direction)
    }

    fn ray_color(r: &Ray, depth: u32, world: &dyn Hittable) -> Color {
        if depth <= 0 {
            return Color::default();
        }

        let mut rec = HitRecord::default();
        if world.hit(r, 0.001..=f64::INFINITY, &mut rec) {
            let (mut attenuation, mut scattered) = (Color::default(), Ray::default());
            if rec
                .mat
                .clone()
                .unwrap()
                .scatter(r, &rec, &mut attenuation, &mut scattered)
            {
                return attenuation * Camera::ray_color(&scattered, depth - 1, world);
            }
            return Color::default();
        }

        let unit_direction = unit_vector(r.direction());
        let a = 0.5 * (unit_direction.y() + 1.0);
        (1.0 - a) * Color::new(1.0, 1.0, 1.0) + a * Color::new(0.5, 0.7, 1.0)
    }
}
