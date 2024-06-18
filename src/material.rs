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
    pub albedo: Color,
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

#[derive(Debug, Clone, Copy)]
pub struct Metal {
    pub albedo: Color,
    pub fuzz: Option<f64>,
}

impl Default for Metal {
    fn default() -> Self {
        Self {
            albedo: Color(0.5, 0.5, 0.5),
            fuzz: None,
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
            Some(fuzz) => fuzz.clamp(0.0, 1.0) * random_unit_vector(),
            None => Vec3::default(),
        };
        let reflected = unit_vector(&reflect(r_in.direction(), &rec.normal)) + fuzz_vector;
        *scattered = Ray::new(&rec.p, &reflected);
        *attenuation = self.albedo;
        dot(scattered.direction(), &rec.normal) > 0.0
    }
}

pub struct Dielectric {
    pub refractive_index: f64,
    pub albedo: Color,
}

impl Dielectric {
    fn reflectance(cosine: f64, refractive_index: f64) -> f64 {
        let r0 = ((1.0 - refractive_index) / (1.0 + refractive_index)).powi(2);
        r0 + (1.0 - r0) * (1.0 - cosine).powi(5)
    }
}

impl Default for Dielectric {
    fn default() -> Self {
        Self {
            refractive_index: 1.0,
            albedo: Vec3(1.0, 1.0, 1.0),
        }
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
                *attenuation = Color(1.0, 1.0, 1.0);
                reflect(&unit_direction, &rec.normal)
            } else {
                *attenuation = if rec.front_face {
                    self.albedo
                } else {
                    Color(1.0, 1.0, 1.0)
                };
                refract(&unit_direction, &rec.normal, ri)
            };

        *scattered = Ray::new(&rec.p, &direction);
        true
    }
}
