use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Sub, SubAssign, Neg};

use crate::angle::Radians;

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Vec2 {
    pub x: f32,
    pub y: f32,
}

impl Vec2 {
    pub const ZERO: Self = Self { x: 0.0, y: 0.0 };
    pub const ONE: Self = Self { x: 1.0, y: 1.0 };

    pub fn new(x: f32, y: f32) -> Self {
        Self { x, y }
    }

    pub fn length_square(&self) -> f32 {
        self.x * self.x + self.y * self.y
    }
    pub fn length(&self) -> f32 {
        (self.x * self.x + self.y * self.y).sqrt()
    }

    pub fn normalize(&self) -> Self {
        let len = self.length();
        if len <= 1e-6 {
            return Self::ZERO;
        }
        Self {
            x: self.x / len,
            y: self.y / len,
        }

    }

    pub fn normalize_in_place(&mut self) {
        let len = self.length();
        if len <= 1e-6 {
            self.x = 0f32;
            self.y = 0f32;
            return;
        }
        *self /= len;
    }

    pub fn set_mag(&self, mag: f32) -> Self {
        let len = self.length();
        if len < 1e-6 {
            return Self::ZERO;
        }
        Self {
            x: (self.x/len)*mag,
            y: (self.y/len)*mag,
        }
    }

    pub fn set_mag_in_place(&mut self, mag: f32) {
        let len = self.length();
        if len < 1e-6 {
            self.x = 0f32;
            self.y = 0.0;
            return;
        }
        self.x = (self.x/len)*mag;
        self.y = (self.y/len)*mag;
    }

    pub fn to_tuple(&self) -> (f32, f32) {
        (self.x, self.y)
    }

    pub fn from_tuple(t: (f32, f32)) -> Self {
        Self {
            x: t.0,
            y: t.1
        }
    }

    pub fn dot(&self, rhs: &Vec2) -> f32 {
        self.x * rhs.x + self.y * rhs.y
    }

    pub fn distance(&self, rhs: &Vec2) -> f32 {
        (*self - *rhs).length()
    }

    pub fn distance_squared(&self, rhs: &Vec2) -> f32 {
        (*self - *rhs).length_square()
    }

    pub fn cross(&self, rhs: &Vec2) -> f32 {
        self.x * rhs.y - self.y * rhs.x
    }

    pub fn angle(&self) -> Radians {
        Radians(self.y.atan2(self.x))
    }

    pub fn from_angle(angle: impl Into<Radians>) -> Self {
        let Radians(radians) = angle.into();
        Self { x: radians.cos(), y: radians.sin() }
    }

    pub fn rotate(&self, angle: impl Into<Radians>) -> Self {
        let Radians(radians) = angle.into();
        let (s, c) = radians.sin_cos();
        Self {
            x: self.x * c - self.y * s,
            y: self.x * s + self.y * c,
        }
    }

    pub fn perp(&self) -> Self {
        Self { x: -self.y, y: self.x }
    }

    pub fn lerp(&self, rhs: &Vec2, t: f32) -> Self {
        *self + (*rhs - *self) * t
    }

    pub fn clamp_length(&self, max: f32) -> Self {
        let len_sq = self.length_square();
        if len_sq > max * max {
            *self * (max / len_sq.sqrt())
        } else {
            *self
        }
    }
}

macro_rules! impl_vec2_binop {
    ($trait:ident, $method:ident, $op:tt) => {
        impl $trait<Vec2> for Vec2 {
            type Output = Vec2;
            fn $method(self, rhs: Vec2) -> Vec2 {
                Vec2 { x: self.x $op rhs.x, y: self.y $op rhs.y }
            }
        }
        impl $trait<&Vec2> for Vec2 {
            type Output = Vec2;
            fn $method(self, rhs: &Vec2) -> Vec2 {
                Vec2 { x: self.x $op rhs.x, y: self.y $op rhs.y }
            }
        }
        impl $trait<&mut Vec2> for Vec2 {
            type Output = Vec2;
            fn $method(self, rhs: &mut Vec2) -> Vec2 {
                Vec2 { x: self.x $op rhs.x, y: self.y $op rhs.y }
            }
        }
        impl $trait<Vec2> for &Vec2 {
            type Output = Vec2;
            fn $method(self, rhs: Vec2) -> Vec2 {
                Vec2 { x: self.x $op rhs.x, y: self.y $op rhs.y }
            }
        }
        impl $trait<&Vec2> for &Vec2 {
            type Output = Vec2;
            fn $method(self, rhs: &Vec2) -> Vec2 {
                Vec2 { x: self.x $op rhs.x, y: self.y $op rhs.y }
            }
        }
        impl $trait<&mut Vec2> for &Vec2 {
            type Output = Vec2;
            fn $method(self, rhs: &mut Vec2) -> Vec2 {
                Vec2 { x: self.x $op rhs.x, y: self.y $op rhs.y }
            }
        }
        impl $trait<Vec2> for &mut Vec2 {
            type Output = Vec2;
            fn $method(self, rhs: Vec2) -> Vec2 {
                Vec2 { x: self.x $op rhs.x, y: self.y $op rhs.y }
            }
        }
        impl $trait<&Vec2> for &mut Vec2 {
            type Output = Vec2;
            fn $method(self, rhs: &Vec2) -> Vec2 {
                Vec2 { x: self.x $op rhs.x, y: self.y $op rhs.y }
            }
        }
        impl $trait<&mut Vec2> for &mut Vec2 {
            type Output = Vec2;
            fn $method(self, rhs: &mut Vec2) -> Vec2 {
                Vec2 { x: self.x $op rhs.x, y: self.y $op rhs.y }
            }
        }
    };
}

macro_rules! impl_vec2_assignop {
    ($trait:ident, $method:ident, $op:tt) => {
        impl $trait<Vec2> for Vec2 {
            fn $method(&mut self, rhs: Vec2) {
                self.x $op rhs.x;
                self.y $op rhs.y;
            }
        }
        impl $trait<&Vec2> for Vec2 {
            fn $method(&mut self, rhs: &Vec2) {
                self.x $op rhs.x;
                self.y $op rhs.y;
            }
        }
        impl $trait<&mut Vec2> for Vec2 {
            fn $method(&mut self, rhs: &mut Vec2) {
                self.x $op rhs.x;
                self.y $op rhs.y;
            }
        }
    };
}

macro_rules! impl_vec2_scalar {
    ($trait:ident, $method:ident, $op:tt) => {
        impl $trait<f32> for Vec2 {
            type Output = Vec2;
            fn $method(self, rhs: f32) -> Vec2 {
                Vec2 { x: self.x $op rhs, y: self.y $op rhs }
            }
        }
        impl $trait<f32> for &Vec2 {
            type Output = Vec2;
            fn $method(self, rhs: f32) -> Vec2 {
                Vec2 { x: self.x $op rhs, y: self.y $op rhs }
            }
        }
        impl $trait<f32> for &mut Vec2 {
            type Output = Vec2;
            fn $method(self, rhs: f32) -> Vec2 {
                Vec2 { x: self.x $op rhs, y: self.y $op rhs }
            }
        }
    };
}

macro_rules! impl_vec2_scalar_assign {
    ($trait:ident, $method:ident, $op:tt) => {
        impl $trait<f32> for Vec2 {
            fn $method(&mut self, rhs: f32) {
                self.x $op rhs;
                self.y $op rhs;
            }
        }
    };
}

macro_rules! impl_vec2_neg {
    () => {
        impl Neg for Vec2 {
            type Output = Vec2;
            fn neg(self) -> Vec2 {
                Vec2 { x: -self.x, y: -self.y }
            }
        }
        impl Neg for &Vec2 {
            type Output = Vec2;
            fn neg(self) -> Vec2 {
                Vec2 { x: -self.x, y: -self.y }
            }
        }
        impl Neg for &mut Vec2 {
            type Output = Vec2;
            fn neg(self) -> Vec2 {
                Vec2 { x: -self.x, y: -self.y }
            }
        }
    };
}

impl_vec2_binop!(Add, add, +);
impl_vec2_binop!(Sub, sub, -);
impl_vec2_binop!(Mul, mul, *);
impl_vec2_binop!(Div, div, /);

impl_vec2_assignop!(AddAssign, add_assign, +=);
impl_vec2_assignop!(SubAssign, sub_assign, -=);
impl_vec2_assignop!(MulAssign, mul_assign, *=);
impl_vec2_assignop!(DivAssign, div_assign, /=);

impl_vec2_scalar!(Mul, mul, *);
impl_vec2_scalar!(Div, div, /);

impl_vec2_scalar_assign!(MulAssign, mul_assign, *=);
impl_vec2_scalar_assign!(DivAssign, div_assign, /=);

impl_vec2_neg!();
