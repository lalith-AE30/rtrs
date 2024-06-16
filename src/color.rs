use std::{
    io::{Error, Write},
    ops::Mul,
};

use crate::{vec3::Vec3, ClampExt};

pub type Color = Vec3;

impl Mul for Color {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        Self {
            e: [
                self.e[0] * rhs.e[0],
                self.e[1] * rhs.e[1],
                self.e[2] * rhs.e[2],
            ],
        }
    }
}

fn linear_to_gamma(linear_component: f64) -> f64 {
    if linear_component > 0.0 {
        return linear_component.sqrt();
    }
    0.0
}

pub fn write_color(file: &mut dyn Write, pixel_color: &Color) -> Result<(), Error> {
    let [r, g, b] = pixel_color.e;

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
