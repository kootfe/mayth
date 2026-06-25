//! A 2-D vector type for f32 arithmetic, geometry, and angle operations.
//!
//! [`Vec2`] supports all standard arithmetic operators (`+`, `-`, `*`, `/`, unary `-`)
//! against both other vectors and bare `f32` scalars, in owned, `&`, and `&mut` forms.
//!
//! # Example
//! ```
//! use mayth::vec::Vec2;
//! use mayth::angle::Degrees;
//!
//! let v = Vec2::new(3.0, 4.0);
//! assert_eq!(v.length(), 5.0);
//!
//! let rotated = v.rotate(Degrees(90.0));
//! ```

use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Sub, SubAssign};

use crate::angle::Radians;

/// A 2-D vector with `f32` components.
///
/// Implements component-wise arithmetic with other [`Vec2`] values and
/// uniform scaling by `f32`, in all reference combinations.
#[derive(Clone, Copy, Debug)]
pub struct Vec2 {
    /// Horizontal component.
    pub x: f32,
    /// Vertical component.
    pub y: f32,
}

impl Vec2 {
    /// The zero vector `(0, 0)`. Additive identity.
    pub const ZERO: Self = Self { x: 0.0, y: 0.0 };

    /// The vector `(1, 1)`. Multiplicative identity for component-wise scaling.
    pub const ONE: Self = Self { x: 1.0, y: 1.0 };

    /// Creates a new [`Vec2`] from `x` and `y` components.
    pub fn new(x: f32, y: f32) -> Self {
        Self { x, y }
    }

    /// Returns the squared Euclidean length (`x² + y²`).
    ///
    /// Prefer this over [`length`](Self::length) when you only need to compare
    /// magnitudes, since it avoids a `sqrt`.
    pub fn length_square(&self) -> f32 {
        self.x * self.x + self.y * self.y
    }

    /// Returns the Euclidean length (`√(x² + y²)`).
    pub fn length(&self) -> f32 {
        (self.x * self.x + self.y * self.y).sqrt()
    }

    /// Returns a unit vector pointing in the same direction.
    ///
    /// Returns [`Vec2::ZERO`] if the length is below `1e-6` to avoid
    /// division by near-zero.
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

    /// Normalizes this vector in place.
    ///
    /// Sets the vector to [`Vec2::ZERO`] if the length is below `1e-6`.
    pub fn normalize_in_place(&mut self) {
        let len = self.length();
        if len <= 1e-6 {
            self.x = 0f32;
            self.y = 0f32;
            return;
        }
        *self /= len;
    }

    /// Returns a vector with the same direction but a specific magnitude.
    ///
    /// Returns [`Vec2::ZERO`] if the current length is below `1e-6`.
    pub fn set_mag(&self, mag: f32) -> Self {
        let len = self.length();
        if len < 1e-6 {
            return Self::ZERO;
        }
        Self {
            x: (self.x / len) * mag,
            y: (self.y / len) * mag,
        }
    }

    /// Sets the magnitude of this vector in place, preserving its direction.
    ///
    /// Sets the vector to `(0, 0)` if the current length is below `1e-6`.
    pub fn set_mag_in_place(&mut self, mag: f32) {
        let len = self.length();
        if len < 1e-6 {
            self.x = 0f32;
            self.y = 0.0;
            return;
        }
        self.x = (self.x / len) * mag;
        self.y = (self.y / len) * mag;
    }

    /// Converts the vector to a plain `(x, y)` tuple.
    pub fn to_tuple(&self) -> (f32, f32) {
        (self.x, self.y)
    }

    /// Creates a [`Vec2`] from a plain `(x, y)` tuple.
    pub fn from_tuple(t: (f32, f32)) -> Self {
        Self { x: t.0, y: t.1 }
    }

    /// Returns the dot product `self · rhs` (`x₁x₂ + y₁y₂`).
    ///
    /// Equals `|self| |rhs| cos θ`, where `θ` is the angle between the two vectors.
    pub fn dot(&self, rhs: &Vec2) -> f32 {
        self.x * rhs.x + self.y * rhs.y
    }

    /// Returns the Euclidean distance between `self` and `rhs`.
    pub fn distance(&self, rhs: &Vec2) -> f32 {
        (*self - *rhs).length()
    }

    /// Returns the squared Euclidean distance between `self` and `rhs`.
    ///
    /// Avoids a `sqrt`; prefer over [`distance`](Self::distance) for comparisons.
    pub fn distance_squared(&self, rhs: &Vec2) -> f32 {
        (*self - *rhs).length_square()
    }

    /// Returns the 2-D cross product `x₁y₂ − y₁x₂`.
    ///
    /// This is the *z*-component of the 3-D cross product when both vectors
    /// are embedded in the XY plane. Its sign indicates the turn direction
    /// from `self` to `rhs`: positive → counter-clockwise, negative → clockwise.
    pub fn cross(&self, rhs: &Vec2) -> f32 {
        self.x * rhs.y - self.y * rhs.x
    }

    /// Returns the angle of this vector from the positive X-axis, in [`Radians`].
    ///
    /// Equivalent to `atan2(y, x)`. Range is `(−π, π]`.
    pub fn angle(&self) -> Radians {
        Radians(self.y.atan2(self.x))
    }

    /// Creates a unit vector pointing in the given `angle` from the positive X-axis.
    ///
    /// Accepts any type convertible to [`Radians`] (e.g. [`Degrees`](crate::angle::Degrees),
    /// [`Turns`](crate::angle::Turns)).
    ///
    /// # Example
    /// ```
    /// use mayth::angle::Degrees;
    /// use mayth::vec::Vec2;
    /// let right = Vec2::from_angle(Degrees(0.0));   // (1, 0)
    /// let up    = Vec2::from_angle(Degrees(90.0));  // (0, 1)
    /// ```
    pub fn from_angle(angle: impl Into<Radians>) -> Self {
        let Radians(r) = angle.into();
        Self {
            x: r.cos(),
            y: r.sin(),
        }
    }

    /// Returns this vector rotated counter-clockwise by `angle`.
    ///
    /// Accepts any type convertible to [`Radians`].
    pub fn rotate(&self, angle: impl Into<Radians>) -> Self {
        let Radians(r) = angle.into();
        let (s, c) = r.sin_cos();
        Self {
            x: self.x * c - self.y * s,
            y: self.x * s + self.y * c,
        }
    }

    /// Returns the left-hand perpendicular vector `(−y, x)`.
    ///
    /// The result is rotated 90° counter-clockwise and has the same length as `self`.
    pub fn perp(&self) -> Self {
        Self {
            x: -self.y,
            y: self.x,
        }
    }

    /// Linearly interpolates between `self` and `rhs` by factor `t`.
    ///
    /// - `t = 0.0` → `self`
    /// - `t = 1.0` → `rhs`
    /// - Values outside `[0, 1]` extrapolate beyond the endpoints.
    pub fn lerp(&self, rhs: &Vec2, t: f32) -> Self {
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

    /// Returns a vector with the absolute value of each component.
    ///
    /// # Example
    /// ```
    /// use mayth::vec::Vec2;
    /// let v = Vec2::new(-3.0, 4.0);
    /// assert_eq!(v.abs(), Vec2::new(3.0, 4.0));
    /// ```
    pub fn abs(&self) -> Self {
        Self {
            x: self.x.abs(),
            y: self.y.abs(),
        }
    }
}

// ── Operator macros ──────────────────────────────────────────────────────────
//
// These generate trait impls for all combinations of Vec2, &Vec2, and &mut Vec2
// so callers are never forced to insert extra clones or derefs.

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
                Vec2 {
                    x: -self.x,
                    y: -self.y,
                }
            }
        }
        impl Neg for &Vec2 {
            type Output = Vec2;
            fn neg(self) -> Vec2 {
                Vec2 {
                    x: -self.x,
                    y: -self.y,
                }
            }
        }
        impl Neg for &mut Vec2 {
            type Output = Vec2;
            fn neg(self) -> Vec2 {
                Vec2 {
                    x: -self.x,
                    y: -self.y,
                }
            }
        }
    };
}

// Vec2 ± Vec2, Vec2 * Vec2, Vec2 / Vec2
impl_vec2_binop!(Add, add, +);
impl_vec2_binop!(Sub, sub, -);
impl_vec2_binop!(Mul, mul, *);
impl_vec2_binop!(Div, div, /);

// Vec2 ±= Vec2, etc.
impl_vec2_assignop!(AddAssign, add_assign, +=);
impl_vec2_assignop!(SubAssign, sub_assign, -=);
impl_vec2_assignop!(MulAssign, mul_assign, *=);
impl_vec2_assignop!(DivAssign, div_assign, /=);

// Vec2 * f32, Vec2 / f32
impl_vec2_scalar!(Mul, mul, *);
impl_vec2_scalar!(Div, div, /);

// Vec2 *= f32, Vec2 /= f32
impl_vec2_scalar_assign!(MulAssign, mul_assign, *=);
impl_vec2_scalar_assign!(DivAssign, div_assign, /=);

// -Vec2
impl_vec2_neg!();

impl PartialEq for Vec2 {
    fn eq(&self, other: &Self) -> bool {
        self.x == other.x && self.y == other.y
    }

    fn ne(&self, other: &Self) -> bool {
        self.x != other.x || self.y != other.y
    }
}

impl PartialOrd for Vec2 {
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

#[cfg(test)]
mod test {
    use super::*;

    fn approx_eq(a: f32, b: f32) -> bool {
        (a - b).abs() < 1e-6
    }

    #[test]
    fn new_sets_components() {
        let v = Vec2::new(3.0, 4.0);
        assert_eq!(v.x, 3.0);
        assert_eq!(v.y, 4.0);
    }

    #[test]
    fn length_is_pythagorean() {
        let v = Vec2::new(3.0, 4.0);
        assert!(approx_eq(v.length(), 5.0)); // classic 3-4-5 triangle
    }

    #[test]
    fn length_square_avoids_sqrt_but_matches() {
        let v = Vec2::new(3.0, 4.0);
        assert!(approx_eq(v.length_square(), 25.0));
    }

    #[test]
    fn normalize_produces_unit_length() {
        let v = Vec2::new(3.0, 4.0).normalize();
        assert!(approx_eq(v.length(), 1.0));
    }

    #[test]
    fn normalize_zero_vector_returns_zero() {
        let v = Vec2::ZERO.normalize();
        assert_eq!(v, Vec2::ZERO); // PartialEq is derived, exact 0.0 == 0.0 is fine here
    }

    #[test]
    fn dot_of_perpendicular_vectors_is_zero() {
        let a = Vec2::new(1.0, 0.0);
        let b = Vec2::new(0.0, 1.0);
        assert!(approx_eq(a.dot(&b), 0.0));
    }

    #[test]
    fn rotate_90_degrees_maps_x_axis_to_y_axis() {
        use crate::angle::Degrees;
        let v = Vec2::new(1.0, 0.0);
        let rotated = v.rotate(Degrees(90.0));
        assert!(approx_eq(rotated.x, 0.0));
        assert!(approx_eq(rotated.y, 1.0));
    }

    #[test]
    fn lerp_at_t_zero_returns_self() {
        let a = Vec2::new(0.0, 0.0);
        let b = Vec2::new(10.0, 10.0);
        let result = a.lerp(&b, 0.0);
        assert_eq!(result, a);
    }

    #[test]
    fn lerp_at_t_one_returns_other() {
        let a = Vec2::new(0.0, 0.0);
        let b = Vec2::new(10.0, 10.0);
        let result = a.lerp(&b, 1.0);
        assert_eq!(result, b);
    }

    #[test]
    fn add_operator_sums_components() {
        let a = Vec2::new(1.0, 2.0);
        let b = Vec2::new(3.0, 4.0);
        assert_eq!(a + b, Vec2::new(4.0, 6.0));
    }
}
