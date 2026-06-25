use crate::{angle::Radians, vec::Vec3};
use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Sub, SubAssign};

#[cfg(feature = "serde")]
use serde::{Serialize, Deserialize};

#[repr(C)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Quat {
    w: f32,
    x: f32,
    y: f32,
    z: f32,
}

impl Quat {
    pub const IDENTITY: Quat = Quat {
        w: 1.0,
        x: 0.0,
        y: 0.0,
        z: 0.0,
    };

    pub fn new(w: f32, x: f32, y: f32, z: f32) -> Self {
        Self { w, x, y, z }
    }

    pub fn from_axis_angle(axis: Vec3, angle: impl Into<Radians>) -> Self {
        let Radians(rad) = angle.into();
        let axis = axis.normalize();
        Self {
            w: (rad / 2.0).cos(),
            x: axis.x * (rad / 2.0).sin(),
            y: axis.y * (rad / 2.0).sin(),
            z: axis.z * (rad / 2.0).sin(),
        }
    }

    pub fn length_square(&self) -> f32 {
        self.w * self.w + self.x * self.x + self.y * self.y + self.z * self.z
    }

    pub fn length(&self) -> f32 {
        // Not calling self.length_square() on purpose to leave call stack empty
        (self.w * self.w + self.x * self.x + self.y * self.y + self.z * self.z).sqrt()
    }

    pub fn normalize(&self) -> Self {
        let len = self.length();
        if len < 1e-6 {
            return Quat::IDENTITY;
        };
        Self {
            w: self.w / len,
            x: self.x / len,
            y: self.y / len,
            z: self.z / len,
        }
    }

    pub fn normalize_in_place(&mut self) {
        let len = self.length();
        if len < 1e-6 {
            self.w = 1.0;
            self.x = 0.0;
            self.y = 0.0;
            self.z = 0.0;
            return;
        }
        self.w /= len;
        self.x /= len;
        self.y /= len;
        self.z /= len;
    }

    pub fn conjugate(&self) -> Self {
        Self {
            w: self.w,
            x: -self.x,
            y: -self.y,
            z: -self.z,
        }
    }

    pub fn conjugate_in_place(&mut self) {
        self.x = -self.x;
        self.y = -self.y;
        self.z = -self.z;
    }

    pub fn inverse(&self) -> Self {
        let len_sq = self.length_square();
        if len_sq < 1e-6 {
            return Quat::IDENTITY;
        }
        let con = self.conjugate();
        Self {
            w: con.w / len_sq,
            x: con.x / len_sq,
            y: con.y / len_sq,
            z: con.z / len_sq,
        }
    }

    pub fn inverse_in_place(&mut self) {
        let len_sq = self.length_square();
        if len_sq < 1e-6 {
            self.w = 1.0;
            self.x = 0.0;
            self.y = 0.0;
            self.z = 0.0;
            return;
        }
        self.w /= len_sq;
        self.x = -self.x / len_sq;
        self.y = -self.y / len_sq;
        self.z = -self.z / len_sq;
    }
}

// ── Operator macros ──────────────────────────────────────────────────────────
//
// These generate trait impls for all combinations of Quat, &Quat, and &mut Quat
// so callers are never forced to insert extra clones or derefs.
//

macro_rules! impl_quat_binop {
    ($trait:ident, $method:ident, $op:tt) => {
        impl $trait<Quat> for Quat {
            type Output = Quat;
            fn $method(self, rhs: Quat) -> Quat {
                Quat { w: self.w $op rhs.w, x: self.x $op rhs.x,
                       y: self.y $op rhs.y, z: self.z $op rhs.z }
            }
        }
        impl $trait<&Quat> for Quat {
            type Output = Quat;
            fn $method(self, rhs: &Quat) -> Quat {
                Quat { w: self.w $op rhs.w, x: self.x $op rhs.x,
                       y: self.y $op rhs.y, z: self.z $op rhs.z }
            }
        }
        impl $trait<&mut Quat> for Quat {
            type Output = Quat;
            fn $method(self, rhs: &mut Quat) -> Quat {
                Quat { w: self.w $op rhs.w, x: self.x $op rhs.x,
                       y: self.y $op rhs.y, z: self.z $op rhs.z }
            }
        }
        impl $trait<Quat> for &Quat {
            type Output = Quat;
            fn $method(self, rhs: Quat) -> Quat {
                Quat { w: self.w $op rhs.w, x: self.x $op rhs.x,
                       y: self.y $op rhs.y, z: self.z $op rhs.z }
            }
        }
        impl $trait<&Quat> for &Quat {
            type Output = Quat;
            fn $method(self, rhs: &Quat) -> Quat {
                Quat { w: self.w $op rhs.w, x: self.x $op rhs.x,
                       y: self.y $op rhs.y, z: self.z $op rhs.z }
            }
        }
        impl $trait<&mut Quat> for &Quat {
            type Output = Quat;
            fn $method(self, rhs: &mut Quat) -> Quat {
                Quat { w: self.w $op rhs.w, x: self.x $op rhs.x,
                       y: self.y $op rhs.y, z: self.z $op rhs.z }
            }
        }
        impl $trait<Quat> for &mut Quat {
            type Output = Quat;
            fn $method(self, rhs: Quat) -> Quat {
                Quat { w: self.w $op rhs.w, x: self.x $op rhs.x,
                       y: self.y $op rhs.y, z: self.z $op rhs.z }
            }
        }
        impl $trait<&Quat> for &mut Quat {
            type Output = Quat;
            fn $method(self, rhs: &Quat) -> Quat {
                Quat { w: self.w $op rhs.w, x: self.x $op rhs.x,
                       y: self.y $op rhs.y, z: self.z $op rhs.z }
            }
        }
        impl $trait<&mut Quat> for &mut Quat {
            type Output = Quat;
            fn $method(self, rhs: &mut Quat) -> Quat {
                Quat { w: self.w $op rhs.w, x: self.x $op rhs.x,
                       y: self.y $op rhs.y, z: self.z $op rhs.z }
            }
        }
    };
}

macro_rules! impl_quat_assignop {
    ($trait:ident, $method:ident, $op:tt) => {
        impl $trait<Quat> for Quat {
            fn $method(&mut self, rhs: Quat) {
                self.w $op rhs.w; self.x $op rhs.x;
                self.y $op rhs.y; self.z $op rhs.z;
            }
        }
        impl $trait<&Quat> for Quat {
            fn $method(&mut self, rhs: &Quat) {
                self.w $op rhs.w; self.x $op rhs.x;
                self.y $op rhs.y; self.z $op rhs.z;
            }
        }
        impl $trait<&mut Quat> for Quat {
            fn $method(&mut self, rhs: &mut Quat) {
                self.w $op rhs.w; self.x $op rhs.x;
                self.y $op rhs.y; self.z $op rhs.z;
            }
        }
    };
}

macro_rules! impl_quat_scalar {
    ($trait:ident, $method:ident, $op:tt) => {
        impl $trait<f32> for Quat {
            type Output = Quat;
            fn $method(self, rhs: f32) -> Quat {
                Quat { w: self.w $op rhs, x: self.x $op rhs,
                       y: self.y $op rhs, z: self.z $op rhs }
            }
        }
        impl $trait<f32> for &Quat {
            type Output = Quat;
            fn $method(self, rhs: f32) -> Quat {
                Quat { w: self.w $op rhs, x: self.x $op rhs,
                       y: self.y $op rhs, z: self.z $op rhs }
            }
        }
        impl $trait<f32> for &mut Quat {
            type Output = Quat;
            fn $method(self, rhs: f32) -> Quat {
                Quat { w: self.w $op rhs, x: self.x $op rhs,
                       y: self.y $op rhs, z: self.z $op rhs }
            }
        }
    };
}

macro_rules! impl_quat_scalar_assign {
    ($trait:ident, $method:ident, $op:tt) => {
        impl $trait<f32> for Quat {
            fn $method(&mut self, rhs: f32) {
                self.w $op rhs; self.x $op rhs;
                self.y $op rhs; self.z $op rhs;
            }
        }
    };
}

macro_rules! impl_quat_mul {
    ($lhs:ty, $rhs:ty) => {
        impl Mul<$rhs> for $lhs {
            type Output = Quat;
            fn mul(self, rhs: $rhs) -> Quat {
                Quat {
                    w: self.w * rhs.w - self.x * rhs.x - self.y * rhs.y - self.z * rhs.z,
                    x: self.w * rhs.x + self.x * rhs.w + self.y * rhs.z - self.z * rhs.y,
                    y: self.w * rhs.y - self.x * rhs.z + self.y * rhs.w + self.z * rhs.x,
                    z: self.w * rhs.z + self.x * rhs.y - self.y * rhs.x + self.z * rhs.w,
                }
            }
        }
    };
}

impl_quat_mul!(Quat, Quat);
impl_quat_mul!(Quat, &Quat);
impl_quat_mul!(Quat, &mut Quat);
impl_quat_mul!(&Quat, Quat);
impl_quat_mul!(&Quat, &Quat);
impl_quat_mul!(&Quat, &mut Quat);
impl_quat_mul!(&mut Quat, Quat);
impl_quat_mul!(&mut Quat, &Quat);
impl_quat_mul!(&mut Quat, &mut Quat);

impl MulAssign<Quat> for Quat {
    fn mul_assign(&mut self, rhs: Quat) {
        *self = &*self * rhs;
    }
}
impl MulAssign<&Quat> for Quat {
    fn mul_assign(&mut self, rhs: &Quat) {
        *self = &*self * rhs;
    }
}
impl MulAssign<&mut Quat> for Quat {
    fn mul_assign(&mut self, rhs: &mut Quat) {
        *self = &*self * &*rhs;
    }
}

impl Neg for Quat {
    type Output = Quat;
    fn neg(self) -> Quat {
        Quat {
            w: -self.w,
            x: -self.x,
            y: -self.y,
            z: -self.z,
        }
    }
}
impl Neg for &Quat {
    type Output = Quat;
    fn neg(self) -> Quat {
        Quat {
            w: -self.w,
            x: -self.x,
            y: -self.y,
            z: -self.z,
        }
    }
}
impl Neg for &mut Quat {
    type Output = Quat;
    fn neg(self) -> Quat {
        Quat {
            w: -self.w,
            x: -self.x,
            y: -self.y,
            z: -self.z,
        }
    }
}

impl_quat_binop!(Add, add, +);
impl_quat_binop!(Sub, sub, -);

impl_quat_assignop!(AddAssign, add_assign, +=);
impl_quat_assignop!(SubAssign, sub_assign, -=);

impl_quat_scalar!(Mul, mul, *);
impl_quat_scalar!(Div, div, /);

impl_quat_scalar_assign!(MulAssign, mul_assign, *=);
impl_quat_scalar_assign!(DivAssign, div_assign, /=);

impl PartialOrd for Quat {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.length_square().partial_cmp(&other.length_square())
    }
    fn lt(&self, other: &Self) -> bool {
        self.length_square() < other.length_square()
    }
    fn gt(&self, other: &Self) -> bool {
        self.length_square() > other.length_square()
    }
    fn le(&self, other: &Self) -> bool {
        self.length_square() <= other.length_square()
    }
    fn ge(&self, other: &Self) -> bool {
        self.length_square() >= other.length_square()
    }
}
