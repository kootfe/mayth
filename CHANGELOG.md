# v0.0.1-alpha.5
- **Breaking:** `Vec2` is now `#[repr(C)]` (C-compatible layout guaranteed)
- **Breaking:** `Vec3` is now `#[repr(C)]` (C-compatible layout guaranteed)
- **Breaking:** `Quat` is now `#[repr(C)]` (C-compatible layout guaranteed)
- Added: optional `serde` feature for serialization/deserialization.
- Added: optional `bytemuck` feature for `Pod`/`Zeroable` support on `Matrix4` for functions `as_flat()` and `as_flat_mut()`.
- Docs: Doctest for `Vec2` and `Vec3` `PartialOrd` operations.

# v0.0.1-alpha.4
- **Breaking:** `Matrix4` is now `#[repr(C)]` (C-compatible layout guaranteed)
- Changed: `Matrix4` now uses a SIMD backend (SSE2) where supported, falling back to scalar arithmetic on unsupported targets.
- Added: `PartialOrd` and `PartialEq` for `Vec2` and `Vec3`
- Added: unit tests for `Vec2`
- Added: `SIMD` basics. (`mayth::simd::*`)
- Added: `Quat` (`mayth::quat::Quat`)

# v0.0.1-alpha.3
- Bugfix: Fixed double normalize in Vec3::rotate()
- Docs: Doctest fixes
- Chore: README updates

# v0.0.1-alpha.2
- **Breaking:** Flattened module paths for `Vec2`, `Vec3`, and matrix types
  (e.g. `mayth::vec::vec3::Vec3` → `mayth::vec::Vec3`)
- Added: `Matrix4` base (`mayth::matrix::Matrix4`)

# v0.0.1-alpha.1
- Added: `Vec3` (`mayth::vec::vec3::Vec3`)
- Added: docs.rs documentation for `Vec2`, `Vec3`, and `Angles`

# v0.0.1-alpha
- Added: `Vec2` (`mayth::vec::vec2::Vec2`)
- Added: angle types (`mayth::angle::{Radians, Degrees, Turns, Gradians}`)
