use forward_ref::{forward_ref_binop, forward_ref_op_assign, forward_ref_unop};
use std::ops::{Add, AddAssign, Div, Mul, Neg, RangeInclusive, Sub, SubAssign};

#[derive(Default, Debug, Copy, Clone)]
pub struct Vec3(pub f64, pub f64, pub f64);

pub type Point3 = Vec3;
#[allow(non_snake_case)]
pub fn Point3(x: f64, y: f64, z: f64) -> Point3 {
    Vec3(x, y, z)
}
impl From<Vec3> for (f64, f64, f64) {
    fn from(value: Vec3) -> Self {
        (value.0, value.1, value.2)
    }
}
impl From<&Vec3> for (f64, f64, f64) {
    fn from(value: &Vec3) -> Self {
        (value.0, value.1, value.2)
    }
}

impl Vec3 {
    pub const fn x(&self) -> f64 {
        self.0
    }

    pub const fn y(&self) -> f64 {
        self.1
    }

    pub const fn z(&self) -> f64 {
        self.2
    }

    pub fn length(&self) -> f64 {
        self.length_squared().sqrt()
    }

    pub fn length_squared(&self) -> f64 {
        dot(self, self)
    }

    pub fn near_zero(&self) -> bool {
        let s = 1e-8;
        return [self.0, self.1, self.2].iter().all(|e| e.abs() < s);
    }

    pub fn random(interval: Option<RangeInclusive<f64>>) -> Self {
        match interval {
            Some(int) => Self(
                int.start() + ((int.end() - int.start()) * fastrand::f64()),
                int.start() + ((int.end() - int.start()) * fastrand::f64()),
                int.start() + ((int.end() - int.start()) * fastrand::f64()),
            ),
            None => Self(fastrand::f64(), fastrand::f64(), fastrand::f64()),
        }
    }
}

impl Neg for Vec3 {
    type Output = Self;

    fn neg(self) -> Self::Output {
        Self(-self.0, -self.1, -self.2)
    }
}

impl Add for Vec3 {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self(self.0 + rhs.0, self.1 + rhs.1, self.2 + rhs.2)
    }
}
impl AddAssign for Vec3 {
    fn add_assign(&mut self, rhs: Self) {
        self.0 += rhs.0;
        self.1 += rhs.1;
        self.2 += rhs.2;
    }
}

impl Sub for Vec3 {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        self + (-rhs)
    }
}
impl SubAssign for Vec3 {
    fn sub_assign(&mut self, rhs: Self) {
        *self += -rhs;
    }
}

impl Mul<f64> for Vec3 {
    type Output = Vec3;

    fn mul(self, rhs: f64) -> Self::Output {
        Self(rhs * self.0, rhs * self.1, rhs * self.2)
    }
}
impl Mul<Vec3> for f64 {
    type Output = Vec3;

    fn mul(self, rhs: Vec3) -> Self::Output {
        rhs * self
    }
}

impl Div<f64> for Vec3 {
    type Output = Vec3;

    fn div(self, rhs: f64) -> Self::Output {
        self * (1.0 / rhs)
    }
}

forward_ref_unop!(impl Neg, neg for Vec3);
forward_ref_binop!(impl Add, add for Vec3, Vec3);
forward_ref_binop!(impl Sub, sub for Vec3, Vec3);
forward_ref_binop!(impl Mul, mul for Vec3, f64);
forward_ref_binop!(impl Mul, mul for f64, Vec3);
forward_ref_binop!(impl Div, div for Vec3, f64);
forward_ref_op_assign!(impl AddAssign, add_assign for Vec3, Vec3);
forward_ref_op_assign!(impl SubAssign, sub_assign for Vec3, Vec3);

pub fn dot(u: &Vec3, v: &Vec3) -> f64 {
    u.x() * v.x() + u.y() * v.y() + u.z() * v.z()
}

pub fn cross(u: &Vec3, v: &Vec3) -> Vec3 {
    Vec3(
        u.y() * v.z() - u.z() * v.y(),
        u.z() * v.x() - u.x() * v.z(),
        u.x() * v.y() - u.y() * v.x(),
    )
}

#[inline]
pub fn unit_vector(v: &Vec3) -> Vec3 {
    v / v.length()
}

pub fn random_int_unit_disk() -> Vec3 {
    loop {
        let p = Vec3(
            fastrand::f64() * 2.0 - 1.0,
            fastrand::f64() * 2.0 - 1.0,
            0.0,
        );
        if p.length_squared() < 1.0 {
            return p;
        }
    }
}

pub fn random_in_unit_sphere() -> Vec3 {
    loop {
        let p = Vec3::random(Some(-1.0..=1.0));
        if p.length_squared() < 1.0 {
            return p;
        }
    }
}

pub fn random_unit_vector() -> Vec3 {
    unit_vector(&random_in_unit_sphere())
}

pub fn random_on_hemisphere(normal: &Vec3) -> Vec3 {
    let on_unit_sphere = random_unit_vector();
    if dot(&on_unit_sphere, normal) > 0.0 {
        on_unit_sphere
    } else {
        -on_unit_sphere
    }
}

pub fn reflect(v: &Vec3, n: &Vec3) -> Vec3 {
    v - 2.0 * dot(v, n) * n
}

pub fn refract(uv: &Vec3, n: &Vec3, rel_index: f64) -> Vec3 {
    let cos_theta = dot(&-uv, n);
    let r_out_perp = rel_index * (uv + cos_theta * n);
    let r_out_parallel = -(1.0 - r_out_perp.length_squared()).abs().sqrt() * n;
    r_out_perp + r_out_parallel
}
