use crate::{vec3::Vec3, ClampExt};
use std::{
    io::{Error, Write},
    ops::Mul,
};

pub type Color = Vec3;
#[allow(non_snake_case)]
pub fn Color(r: f64, g: f64, b: f64) -> Color {
    Vec3(r, g, b)
}

impl Mul for Color {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        Self(self.0 * rhs.0, self.1 * rhs.1, self.2 * rhs.2)
    }
}

fn linear_to_gamma(linear_component: f64) -> f64 {
    if linear_component > 0.0 {
        return linear_component.sqrt();
    }
    0.0
}

pub fn write_color(file: &mut dyn Write, pixel_color: &Color) -> Result<(), Error> {
    let (r, g, b) = pixel_color.into();

    let (r, g, b) = (linear_to_gamma(r), linear_to_gamma(g), linear_to_gamma(b));

    let intensity = 0.0..=0.999;
    let (rbyte, gbyte, bbyte) = (
        (256.0 * intensity.clamp(r)) as i32,
        (256.0 * intensity.clamp(g)) as i32,
        (256.0 * intensity.clamp(b)) as i32,
    );

    file.write(format!("{} {} {}\n", rbyte, gbyte, bbyte).as_bytes())?;

    Ok(())
}
