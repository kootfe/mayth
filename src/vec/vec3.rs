//! A 3-D vector type for f32 arithmetic, geometry, and angle operations.
//!
//! [`Vec3`] supports all standard arithmetic operators (`+`, `-`, `*`, `/`, unary `-`)
//! against both other vectors and bare `f32` scalars, in owned, `&`, and `&mut` forms.
//!
//! # Example
//! ```
//! use mayth::vec3::Vec3;
//! use mayth::angle::Degrees;
//!
//! let v = Vec3::new(1.0, 0.0, 0.0);
//! let rotated = v.rotate(Vec3::new(0.0, 1.0, 0.0), Degrees(90.0));
//! ```

use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Sub, SubAssign};

use crate::angle::Radians;

/// A 3-D vector with `f32` components.
///
/// Implements component-wise arithmetic with other [`Vec3`] values and
/// uniform scaling by `f32`, in all reference combinations.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Vec3 {
    /// Horizontal component.
    pub x: f32,
    /// Vertical component.
    pub y: f32,
    /// Depth component.
    pub z: f32,
}

impl Vec3 {
    /// The zero vector `(0, 0, 0)`. Additive identity.
    pub const ZERO: Self = Self { x: 0.0, y: 0.0, z: 0.0 };

    /// The vector `(1, 1, 1)`. Multiplicative identity for component-wise scaling.
    pub const ONE: Self = Self { x: 1.0, y: 1.0, z: 1.0 };

    /// Creates a new [`Vec3`] from `x`, `y`, and `z` components.
    pub fn new(x: f32, y: f32, z: f32) -> Self {
        Self { x, y, z }
    }

    /// Returns the squared Euclidean length (`x² + y² + z²`).
    ///
    /// Prefer this over [`length`](Self::length) when only comparing
    /// magnitudes, since it avoids a `sqrt`.
    pub fn length_square(&self) -> f32 {
        self.x * self.x + self.y * self.y + self.z * self.z
    }

    /// Returns the Euclidean length (`√(x² + y² + z²)`).
    pub fn length(&self) -> f32 {
        (self.x * self.x + self.y * self.y + self.z * self.z).sqrt()
    }

    /// Returns a unit vector pointing in the same direction.
    ///
    /// Returns [`Vec3::ZERO`] if the length is below `1e-6` to avoid
    /// division by near-zero.
    pub fn normalize(&self) -> Self {
        let len = self.length();
        if len <= 1e-6 {
            return Self::ZERO;
        }
        Self {
            x: self.x / len,
            y: self.y / len,
            z: self.z / len,
        }
    }

    /// Normalizes this vector in place.
    ///
    /// Sets the vector to [`Vec3::ZERO`] if the length is below `1e-6`.
    pub fn normalize_in_place(&mut self) {
        let len = self.length();
        if len <= 1e-6 {
            self.x = 0f32;
            self.y = 0f32;
            self.z = 0f32;
            return;
        }
        *self /= len;
    }

    /// Returns a vector with the same direction but a specific magnitude.
    ///
    /// Returns [`Vec3::ZERO`] if the current length is below `1e-6`.
    pub fn set_mag(&self, mag: f32) -> Self {
        let len = self.length();
        if len < 1e-6 {
            return Self::ZERO;
        }
        Self {
            x: (self.x / len) * mag,
            y: (self.y / len) * mag,
            z: (self.z / len) * mag,
        }
    }

    /// Sets the magnitude of this vector in place, preserving its direction.
    ///
    /// Sets the vector to `(0, 0, 0)` if the current length is below `1e-6`.
    pub fn set_mag_in_place(&mut self, mag: f32) {
        let len = self.length();
        if len < 1e-6 {
            self.x = 0f32;
            self.y = 0.0;
            self.z = 0f32;
            return;
        }
        self.x = (self.x / len) * mag;
        self.y = (self.y / len) * mag;
        self.z = (self.z / len) * mag;
    }

    /// Converts the vector to a plain `(x, y, z)` tuple.
    pub fn to_tuple(&self) -> (f32, f32, f32) {
        (self.x, self.y, self.z)
    }

    /// Creates a [`Vec3`] from a plain `(x, y, z)` tuple.
    pub fn from_tuple(t: (f32, f32, f32)) -> Self {
        Self { x: t.0, y: t.1, z: t.2 }
    }

    /// Returns the dot product `self · rhs` (`x₁x₂ + y₁y₂ + z₁z₂`).
    ///
    /// Equals `|self| |rhs| cos θ`, where `θ` is the angle between the vectors.
    /// Returns `0` when the vectors are perpendicular, positive when they point
    /// in the same general direction, negative when opposed.
    pub fn dot(&self, rhs: &Vec3) -> f32 {
        self.x * rhs.x + self.y * rhs.y + self.z * rhs.z
    }

    /// Returns the Euclidean distance between `self` and `rhs`.
    pub fn distance(&self, rhs: &Vec3) -> f32 {
        (*self - *rhs).length()
    }

    /// Returns the squared Euclidean distance between `self` and `rhs`.
    ///
    /// Avoids a `sqrt`; prefer over [`distance`](Self::distance) for comparisons.
    pub fn distance_squared(&self, rhs: &Vec3) -> f32 {
        (*self - *rhs).length_square()
    }

    /// Returns the 3-D cross product `self × rhs`.
    ///
    /// The result is perpendicular to both input vectors, with magnitude
    /// `|self| |rhs| sin θ`. Direction follows the right-hand rule.
    /// Returns the zero vector when `self` and `rhs` are parallel or either is zero.
    pub fn cross(&self, rhs: &Vec3) -> Vec3 {
        Vec3::new(
            self.y * rhs.z - self.z * rhs.y,
            self.z * rhs.x - self.x * rhs.z,
            self.x * rhs.y - self.y * rhs.x,
        )
    }

    /// Returns the angle of this vector projected onto the XY plane, measured
    /// from the positive X-axis toward Y, in [`Radians`].
    ///
    /// Equivalent to `atan2(y, x)`. Range is `(−π, π]`. The Z component is ignored.
    pub fn angle_xy(&self) -> Radians {
        Radians(self.y.atan2(self.x))
    }

    /// Returns the angle of this vector projected onto the XZ plane, measured
    /// from the positive X-axis toward Z, in [`Radians`].
    ///
    /// Equivalent to `atan2(z, x)`. Range is `(−π, π]`. The Y component is ignored.
    pub fn angle_xz(&self) -> Radians {
        Radians(self.z.atan2(self.x))
    }

    /// Returns the angle of this vector projected onto the YZ plane, measured
    /// from the positive Y-axis toward Z, in [`Radians`].
    ///
    /// Equivalent to `atan2(z, y)`. Range is `(−π, π]`. The X component is ignored.
    pub fn angle_yz(&self) -> Radians {
        Radians(self.z.atan2(self.y))
    }

    /// Creates a unit direction vector from a yaw and pitch angle.
    ///
    /// Uses a Y-up convention:
    /// - `yaw` rotates around the Y-axis (left/right).
    /// - `pitch` tilts above/below the XZ plane (up/down).
    ///
    /// Both angles accept any type convertible to [`Radians`].
    ///
    /// # Example
    /// ```
    /// // Facing along +X with no tilt
    /// let forward = Vec3::from_yaw_pitch(Degrees(0.0), Degrees(0.0));
    /// ```
    pub fn from_yaw_pitch(yaw: impl Into<Radians>, pitch: impl Into<Radians>) -> Self {
        let Radians(y) = yaw.into();
        let Radians(p) = pitch.into();
        Vec3 {
            x: y.cos() * p.cos(),
            y: p.sin(),
            z: y.sin() * p.cos(),
        }
    }

    /// Rotates this vector around `axis` by `angle` using Rodrigues' rotation formula.
    ///
    /// `axis` does not need to be pre-normalized; it is normalized internally.
    /// Returns `*self` unchanged if `axis` has near-zero length (below `1e-6`).
    ///
    /// Accepts any angle type convertible to [`Radians`].
    ///
    /// # Example
    /// ```
    /// // Rotate +X 90° around the Z-axis → +Y
    /// let v = Vec3::new(1.0, 0.0, 0.0);
    /// let up = Vec3::new(0.0, 0.0, 1.0);
    /// let result = v.rotate(up, Degrees(90.0));
    /// ```
    pub fn rotate(&self, axis: Vec3, angle: impl Into<Radians>) -> Self {
        let axis_len_sq = axis.length_square();
        if axis_len_sq < 1e-6 {
            return *self;
        }
        let axis = axis / axis_len_sq.sqrt();

        let Radians(theta) = angle.into();
        let axis = axis.normalize();
        let (s, c) = theta.sin_cos();
        let v = self;
        v * c + axis.cross(v) * s + axis * (axis.dot(v) * (1.0 - c))
    }

    /// Returns the right-hand perpendicular to `self` given a world `up` vector.
    ///
    /// Computes `self × up`, which points to the *right* of `self` in a
    /// right-handed coordinate system. The result is not normalized.
    ///
    /// Useful for constructing camera or orientation bases.
    pub fn right_hand_perp(&self, up: Vec3) -> Vec3 {
        self.cross(&up)
    }

    /// Linearly interpolates between `self` and `rhs` by factor `t`.
    ///
    /// - `t = 0.0` → `self`
    /// - `t = 1.0` → `rhs`
    /// - Values outside `[0, 1]` extrapolate beyond the endpoints.
    pub fn lerp(&self, rhs: &Vec3, t: f32) -> Self {
        *self + (*rhs - *self) * t
    }

    /// Returns the vector clamped to a maximum length of `max`.
    ///
    /// If the current length is already ≤ `max` the vector is returned unchanged.
    pub fn clamp_length(&self, max: f32) -> Self {
        let len_sq = self.length_square();
        if len_sq > max * max {
            *self * (max / len_sq.sqrt())
        } else {
            *self
        }
    }
}

// ── Operator macros ──────────────────────────────────────────────────────────
//
// These generate trait impls for all combinations of Vec3, &Vec3, and &mut Vec3
// so callers are never forced to insert extra clones or derefs.


macro_rules! impl_vec3_binop {
    ($trait:ident, $method:ident, $op:tt) => {
        impl $trait<Vec3> for Vec3 {
            type Output = Vec3;
            fn $method(self, rhs: Vec3) -> Vec3 {
                Vec3 { x: self.x $op rhs.x, y: self.y $op rhs.y, z: self.z $op rhs.z }
            }
        }
        impl $trait<&Vec3> for Vec3 {
            type Output = Vec3;
            fn $method(self, rhs: &Vec3) -> Vec3 {
                Vec3 { x: self.x $op rhs.x, y: self.y $op rhs.y, z: self.z $op rhs.z }
            }
        }
        impl $trait<&mut Vec3> for Vec3 {
            type Output = Vec3;
            fn $method(self, rhs: &mut Vec3) -> Vec3 {
                Vec3 { x: self.x $op rhs.x, y: self.y $op rhs.y, z: self.z $op rhs.z }
            }
        }
        impl $trait<Vec3> for &Vec3 {
            type Output = Vec3;
            fn $method(self, rhs: Vec3) -> Vec3 {
                Vec3 { x: self.x $op rhs.x, y: self.y $op rhs.y, z: self.z $op rhs.z }
            }
        }
        impl $trait<&Vec3> for &Vec3 {
            type Output = Vec3;
            fn $method(self, rhs: &Vec3) -> Vec3 {
                Vec3 { x: self.x $op rhs.x, y: self.y $op rhs.y, z: self.z $op rhs.z }
            }
        }
        impl $trait<&mut Vec3> for &Vec3 {
            type Output = Vec3;
            fn $method(self, rhs: &mut Vec3) -> Vec3 {
                Vec3 { x: self.x $op rhs.x, y: self.y $op rhs.y, z: self.z $op rhs.z }
            }
        }
        impl $trait<Vec3> for &mut Vec3 {
            type Output = Vec3;
            fn $method(self, rhs: Vec3) -> Vec3 {
                Vec3 { x: self.x $op rhs.x, y: self.y $op rhs.y, z: self.z $op rhs.z }
            }
        }
        impl $trait<&Vec3> for &mut Vec3 {
            type Output = Vec3;
            fn $method(self, rhs: &Vec3) -> Vec3 {
                Vec3 { x: self.x $op rhs.x, y: self.y $op rhs.y, z: self.z $op rhs.z }
            }
        }
        impl $trait<&mut Vec3> for &mut Vec3 {
            type Output = Vec3;
            fn $method(self, rhs: &mut Vec3) -> Vec3 {
                Vec3 { x: self.x $op rhs.x, y: self.y $op rhs.y, z: self.z $op rhs.z }
            }
        }
    };
}

macro_rules! impl_vec3_assignop {
    ($trait:ident, $method:ident, $op:tt) => {
        impl $trait<Vec3> for Vec3 {
            fn $method(&mut self, rhs: Vec3) {
                self.x $op rhs.x;
                self.y $op rhs.y;
                self.z $op rhs.z;
            }
        }
        impl $trait<&Vec3> for Vec3 {
            fn $method(&mut self, rhs: &Vec3) {
                self.x $op rhs.x;
                self.y $op rhs.y;
                self.z $op rhs.z;
            }
        }
        impl $trait<&mut Vec3> for Vec3 {
            fn $method(&mut self, rhs: &mut Vec3) {
                self.x $op rhs.x;
                self.y $op rhs.y;
                self.z $op rhs.z;
            }
        }
    };
}

macro_rules! impl_vec3_scalar {
    ($trait:ident, $method:ident, $op:tt) => {
        impl $trait<f32> for Vec3 {
            type Output = Vec3;
            fn $method(self, rhs: f32) -> Vec3 {
                Vec3 { x: self.x $op rhs, y: self.y $op rhs, z: self.z $op rhs }
            }
        }
        impl $trait<f32> for &Vec3 {
            type Output = Vec3;
            fn $method(self, rhs: f32) -> Vec3 {
                Vec3 { x: self.x $op rhs, y: self.y $op rhs, z: self.z $op rhs }
            }
        }
        impl $trait<f32> for &mut Vec3 {
            type Output = Vec3;
            fn $method(self, rhs: f32) -> Vec3 {
                Vec3 { x: self.x $op rhs, y: self.y $op rhs, z: self.z $op rhs }
            }
        }
    };
}

macro_rules! impl_vec3_scalar_assign {
    ($trait:ident, $method:ident, $op:tt) => {
        impl $trait<f32> for Vec3 {
            fn $method(&mut self, rhs: f32) {
                self.x $op rhs;
                self.y $op rhs;
                self.z $op rhs;
            }
        }
    };
}

macro_rules! impl_vec3_neg {
    () => {
        impl Neg for Vec3 {
            type Output = Vec3;
            fn neg(self) -> Vec3 {
                Vec3 {
                    x: -self.x,
                    y: -self.y,
                    z: -self.z
                }
            }
        }
        impl Neg for &Vec3 {
            type Output = Vec3;
            fn neg(self) -> Vec3 {
                Vec3 {
                    x: -self.x,
                    y: -self.y,
                    z: -self.z
                }
            }
        }
        impl Neg for &mut Vec3 {
            type Output = Vec3;
            fn neg(self) -> Vec3 {
                Vec3 {
                    x: -self.x,
                    y: -self.y,
                    z: -self.z
                }
            }
        }
    };
}

// Vec3 ± Vec3, Vec3 * Vec3, Vec3 / Vec3
impl_vec3_binop!(Add, add, +);
impl_vec3_binop!(Sub, sub, -);
impl_vec3_binop!(Mul, mul, *);
impl_vec3_binop!(Div, div, /);

// Vec3 ±= Vec3, etc.
impl_vec3_assignop!(AddAssign, add_assign, +=);
impl_vec3_assignop!(SubAssign, sub_assign, -=);
impl_vec3_assignop!(MulAssign, mul_assign, *=);
impl_vec3_assignop!(DivAssign, div_assign, /=);

// Vec3 * f32, Vec3 / f32
impl_vec3_scalar!(Mul, mul, *);
impl_vec3_scalar!(Div, div, /);

// Vec3 *= f32, Vec3 /= f32
impl_vec3_scalar_assign!(MulAssign, mul_assign, *=);
impl_vec3_scalar_assign!(DivAssign, div_assign, /=);

// -Vec3
impl_vec3_neg!();
