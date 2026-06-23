//! Angle unit types with lossless conversions between them.
//!
//! All four units ([`Radians`], [`Degrees`], [`Turns`], [`Gradians`]) implement
//! [`From`] for every other unit, so you can convert freely with `.into()` or
//! `Type::from(x)`.
//!
//! # Example
//! ```
//! use angle::{Degrees, Radians, Turns, Gradians};
//!
//! let d = Degrees(180.0);
//! let r: Radians = d.into();   // π
//! let t: Turns   = d.into();   // 0.5
//! let g: Gradians = d.into();  // 200
//! ```

use std::f32::consts::{PI, TAU};

/// An angle measured in **radians**.
///
/// One full rotation is `2π` radians.
///
/// # Example
/// ```
/// let r = Radians(std::f32::consts::PI); // 180°
/// ```
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct Radians(pub f32);

/// An angle measured in **degrees**.
///
/// One full rotation is `360°`.
///
/// # Example
/// ```
/// let d = Degrees(90.0); // quarter turn
/// ```
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct Degrees(pub f32);

/// An angle measured in **turns** (full rotations).
///
/// One full rotation equals `1.0`. Useful when thinking in fractions of a circle.
///
/// # Example
/// ```
/// let t = Turns(0.25); // 90°
/// ```
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct Turns(pub f32);

/// An angle measured in **gradians** (also called *gon*).
///
/// One full rotation is `400` gradians. A right angle is exactly `100` gradians.
///
/// # Example
/// ```
/// let g = Gradians(100.0); // 90°
/// ```
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct Gradians(pub f32);

// ── Radians ↔ others ────────────────────────────────────────────────────────

impl From<Degrees> for Radians {
    /// Converts degrees to radians: `d × π / 180`.
    fn from(d: Degrees) -> Self { Radians(d.0.to_radians()) }
}

impl From<Radians> for Degrees {
    /// Converts radians to degrees: `r × 180 / π`.
    fn from(r: Radians) -> Self { Degrees(r.0.to_degrees()) }
}

impl From<Turns> for Radians {
    /// Converts turns to radians: `t × 2π`.
    fn from(t: Turns) -> Self { Radians(t.0 * TAU) }
}

impl From<Radians> for Turns {
    /// Converts radians to turns: `r / 2π`.
    fn from(r: Radians) -> Self { Turns(r.0 / TAU) }
}

impl From<Gradians> for Radians {
    /// Converts gradians to radians: `g × π / 200`.
    fn from(g: Gradians) -> Self { Radians(g.0 * PI / 200.0) }
}

impl From<Radians> for Gradians {
    /// Converts radians to gradians: `r × 200 / π`.
    fn from(r: Radians) -> Self { Gradians(r.0 * 200.0 / PI) }
}

// ── Cross-conversions (routed through Radians) ───────────────────────────────

impl From<Degrees> for Turns {
    /// Converts degrees → radians → turns.
    fn from(d: Degrees) -> Self { Radians::from(d).into() }
}

impl From<Turns> for Degrees {
    /// Converts turns → radians → degrees.
    fn from(t: Turns) -> Self { Radians::from(t).into() }
}

impl From<Degrees> for Gradians {
    /// Converts degrees → radians → gradians.
    fn from(d: Degrees) -> Self { Radians::from(d).into() }
}

impl From<Gradians> for Degrees {
    /// Converts gradians → radians → degrees.
    fn from(g: Gradians) -> Self { Radians::from(g).into() }
}

impl From<Turns> for Gradians {
    /// Converts turns → radians → gradians.
    fn from(t: Turns) -> Self { Radians::from(t).into() }
}

impl From<Gradians> for Turns {
    /// Converts gradians → radians → turns.
    fn from(g: Gradians) -> Self { Radians::from(g).into() }
}

// ── Radians ↔ f32 ────────────────────────────────────────────────────────────

impl From<Radians> for f32 {
    /// Unwraps the inner `f32` value from [`Radians`].
    fn from(r: Radians) -> f32 { r.0 }
}

impl From<f32> for Radians {
    /// Wraps a raw `f32` value as [`Radians`] without any conversion.
    fn from(f: f32) -> Radians { Radians(f) }
}
