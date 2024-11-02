use crate::{
    color::{self, Color},
    degrees_to_radians,
    hittable::{HitRecord, Hittable},
    image::ImageInfo,
    ray::Ray,
    vec3::{cross, random_int_unit_disk, unit_vector, Point3, Vec3},
};
use derive_builder::Builder;
use indicatif::{ProgressBar, ProgressStyle};
use std::{
    io::{Error, Write},
    sync::Mutex,
    thread,
};

#[derive(Default, Debug, Clone, Copy, Builder)]
#[builder(
    custom_constructor,
    build_fn(private, name = "try_build"),
    setter(skip)
)]
pub struct Camera {
    #[builder(setter, default = "8")]
    pub max_depth: u32,
    #[builder(setter)]
    image_info: ImageInfo,
    #[builder(setter, default = "64")]
    samples_per_pixel: u32,
    #[builder(setter, default = "90.0")]
    fov: f64,
    #[builder(setter)]
    look_from: Point3,
    #[builder(setter)]
    look_at: Point3,
    #[builder(setter, default = "Vec3(0.0, 1.0, 0.0)")]
    vup: Vec3,
    #[builder(setter, default = "0.0")]
    defocus_angle: f64,
    #[builder(setter, default = "10.0")]
    focus_dist: f64,
    u: Vec3,
    v: Vec3,
    w: Vec3,
    pixel00_loc: Point3,
    pixel_delta_u: Vec3,
    pixel_delta_v: Vec3,
    pixel_samples_scale: f64,
    defocus_disk_u: Vec3,
    defocus_disk_v: Vec3,
}

impl CameraBuilder {
    pub fn new(image_info: &ImageInfo, look_from: &Point3, look_at: &Point3) -> Self {
        Self {
            image_info: Some(*image_info),
            look_from: Some(*look_from),
            look_at: Some(*look_at),
            ..Self::create_empty()
        }
    }

    pub fn build(&self) -> Camera {
        let mut obj = self
            .try_build()
            .expect("All required fields initialized in constructorF");
        obj.initialize(obj.image_info, obj.samples_per_pixel, obj.fov);
        obj
    }
}

impl Camera {
    pub fn initialize(&mut self, image_info: ImageInfo, samples_per_pixel: u32, fov: f64) {
        self.image_info = image_info;
        self.samples_per_pixel = samples_per_pixel;
        self.fov = fov;

        self.pixel_samples_scale = 1.0 / samples_per_pixel as f64;

        // Camera parameter
        let ray_dir = self.look_from - self.look_at;
        let theta = degrees_to_radians(fov);
        let h = (theta / 2.0).tan();
        let viewport_height = 2.0 * h * self.focus_dist;
        let viewport_width =
            viewport_height * (image_info.image_width as f64 / image_info.image_height as f64);

        self.w = unit_vector(&ray_dir);
        self.u = unit_vector(&cross(&self.vup, &self.w));
        self.v = cross(&self.w, &self.u);

        let (viewport_u, viewport_v) = (viewport_width * self.u, viewport_height * -self.v);

        (self.pixel_delta_u, self.pixel_delta_v) = (
            viewport_u / image_info.image_width as f64,
            viewport_v / image_info.image_height as f64,
        );

        let viewport_upper_left =
            self.look_from - self.focus_dist * self.w - viewport_u / 2.0 - viewport_v / 2.0;
        self.pixel00_loc = viewport_upper_left + 0.5 * (self.pixel_delta_u + self.pixel_delta_v);

        let defocus_radius = self.focus_dist * degrees_to_radians(self.defocus_angle / 2.0).tan();
        self.defocus_disk_u = defocus_radius * self.u;
        self.defocus_disk_v = defocus_radius * self.v;
    }

    pub fn render(&self, file: &mut dyn Write, world: &dyn Hittable) -> Result<(), Error> {
        let (img, samples_per_pixel, max_depth) =
            (self.image_info, self.samples_per_pixel, self.max_depth);
        file.write_all(format!("P3\n{} {}\n255\n", img.image_width, img.image_height).as_bytes())?;

        let thread_bar = {
            let style = ProgressStyle::with_template(
                "[{elapsed_precise}] {bar:40.cyan/blue} {pos:>7}/{len:7} {msg}",
            )
            .unwrap()
            .progress_chars("#>.");
            let ibar = ProgressBar::new(img.image_height as u64);
            ibar.set_style(style);
            ibar
        };

        let render_bar = {
            let style = ProgressStyle::with_template(
                "[{elapsed_precise}] {bar:40.cyan/blue} {pos:>7}/{len:7} {msg}",
            )
            .unwrap()
            .progress_chars("#>.");
            let ibar = ProgressBar::new(img.image_height as u64);
            ibar.set_style(style);
            ibar
        };

        let image = Mutex::new(vec![
            Vec3::default();
            (img.image_height * img.image_width) as usize
        ]);

        thread::scope(|s| {
            let image = &image;
            for j in 0..img.image_height {
                let bar = render_bar.clone();
                s.spawn(move || {
                    for i in 0..img.image_width {
                        let (i, j) = (i as f64, j as f64);
                        let mut pixel_color = Color::default();
                        for _ in 0..samples_per_pixel {
                            let r = self.get_ray(i, j);
                            pixel_color += Camera::ray_color(&r, max_depth, world);
                        }
                        image.lock().unwrap()[j as usize * img.image_width as usize + i as usize] =
                            pixel_color;
                    }
                    bar.inc(1);
                });
                thread_bar.inc(1);
            }
        });
        for pixel_color in image.into_inner().unwrap() {
            color::write_color(file, &(self.pixel_samples_scale * pixel_color))?;
        }

        render_bar.finish();
        Ok(())
    }

    fn sample_square() -> Vec3 {
        Vec3(fastrand::f64() - 0.5, fastrand::f64() - 0.5, 0.0)
    }

    fn defocus_disk_sample(&self) -> Point3 {
        let p = random_int_unit_disk();
        self.look_from + p.x() * self.defocus_disk_u + p.y() * self.defocus_disk_v
    }

    fn get_ray(&self, i: f64, j: f64) -> Ray {
        let offset = Camera::sample_square();
        let pixel_sample = self.pixel00_loc
            + ((i + offset.x()) * self.pixel_delta_u)
            + ((j + offset.y()) * self.pixel_delta_v);

        let ray_origin = if self.defocus_angle > 0.0 {
            self.defocus_disk_sample()
        } else {
            self.look_from
        };
        let ray_direction = pixel_sample - ray_origin;

        Ray::new(&ray_origin, &ray_direction)
    }

    fn ray_color(r: &Ray, depth: u32, world: &dyn Hittable) -> Color {
        if depth == 0 {
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
        (1.0 - a) * Color(1.0, 1.0, 1.0) + a * Color(0.5, 0.7, 1.0)
    }
}
