# v0.0.1-alpha.4
- **Breaking:** `Matrix4` is now `#[repr(C)]` (C-compatible layout guaranteed)
- Change: `Matrix4` now uses a SIMD backend (SSE2) where supported, falling back to scalar arithmetic on unsupported targets.
- Added: `PartialOrd` and `PartialEq` for `Vec2` and `Vec3`
- Added: unit tests for `Vec2`
- Added: `SIMD` basics. (`mayth::simd::*`)
- Added: `Quat` (`mayth::quat::Quat`)

# v0.0.1-alpha.3
- Bugfix: Fixed double normalize in Vec3::rotate()
- Doctest fixes
- README updates

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
