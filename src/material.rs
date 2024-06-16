use crate::{
    color::Color,
    hittable::HitRecord,
    ray::Ray,
    vec3::{dot, random_unit_vector, reflect, refract, unit_vector, Vec3},
};

pub trait Material {
    fn scatter(
        &self,
        r_in: &Ray,
        rec: &HitRecord,
        attenuation: &mut Color,
        scattered: &mut Ray,
    ) -> bool;
}

#[derive(Default, Debug, Clone, Copy)]
pub struct Lambertian {
    albedo: Color,
}

impl Lambertian {
    pub fn new(albedo: &Color) -> Self {
        Self { albedo: *albedo }
    }
}

impl Material for Lambertian {
    fn scatter(
        &self,
        _r_in: &Ray,
        rec: &HitRecord,
        attenuation: &mut Color,
        scattered: &mut Ray,
    ) -> bool {
        let mut scatter_direction = rec.normal + random_unit_vector();
        if scatter_direction.near_zero() {
            scatter_direction = rec.normal;
        }
        *scattered = Ray::new(&rec.p, &scatter_direction);
        *attenuation = self.albedo;
        true
    }
}

#[derive(Default, Debug, Clone, Copy)]
pub struct Metal {
    albedo: Color,
    fuzz: Option<f64>,
}

impl Metal {
    pub fn new(albedo: &Color, fuzz: Option<f64>) -> Self {
        if fuzz != None {
            assert!(fuzz.unwrap() <= 1.0, "Fuzz cannot be greater than 1");
        }
        Self {
            albedo: *albedo,
            fuzz,
        }
    }
}

impl Material for Metal {
    fn scatter(
        &self,
        r_in: &Ray,
        rec: &HitRecord,
        attenuation: &mut Color,
        scattered: &mut Ray,
    ) -> bool {
        let fuzz_vector = match self.fuzz {
            Some(fuzz) => fuzz * random_unit_vector(),
            None => Vec3::default(),
        };
        let reflected = unit_vector(&reflect(r_in.direction(), &rec.normal)) + fuzz_vector;
        *scattered = Ray::new(&rec.p, &reflected);
        *attenuation = self.albedo;
        dot(scattered.direction(), &rec.normal) > 0.0
    }
}

pub struct Dielectric {
    refractive_index: f64,
}

impl Dielectric {
    pub fn new(refractive_index: f64) -> Self {
        Self { refractive_index }
    }

    fn reflectance(cosine: f64, refractive_index: f64) -> f64 {
        let r0 = ((1.0 - refractive_index) / (1.0 + refractive_index)).powi(2);
        r0 + (1.0 - r0) * (1.0 - cosine).powi(5)
    }
}

impl Material for Dielectric {
    fn scatter(
        &self,
        r_in: &Ray,
        rec: &HitRecord,
        attenuation: &mut Color,
        scattered: &mut Ray,
    ) -> bool {
        *attenuation = Color(1.0, 1.0, 1.0);
        let ri = if rec.front_face {
            1.0 / self.refractive_index
        } else {
            self.refractive_index
        };

        let unit_direction = unit_vector(r_in.direction());
        let cos_theta = -dot(&unit_direction, &rec.normal);
        let sin_theta = (1.0 - cos_theta * cos_theta).sqrt();

        let cannot_refract = ri * sin_theta > 1.0;
        let direction =
            if cannot_refract || Dielectric::reflectance(cos_theta, ri) > fastrand::f64() {
                reflect(&unit_direction, &rec.normal)
            } else {
                refract(&unit_direction, &rec.normal, ri)
            };

        *scattered = Ray::new(&rec.p, &direction);
        true
    }
}
