# Mayth

![Rust](https://img.shields.io/badge/rust-%23000000.svg?style=for-the-badge\&logo=rust\&logoColor=white)
![Crates.io Total Downloads](https://img.shields.io/crates/d/mayth?style=for-the-badge\&label=Crates.io%20Downloads)
![Crates.io Version](https://img.shields.io/crates/v/mayth?style=for-the-badge)

*A type-safe math library for games and physics, written in Rust.*
We aim to provide simple and practical math types for game development and physics applications.

## Installing

```bash
cargo add mayth
```

or

```toml
[dependencies]
mayth = "0.0.1-alpha.2"
```

## Example

```rs
use mayth::vec::Vec3;
use mayth::angle::Degrees;

let v = Vec3::new(1.0, 0.0, 0.0);

let rotated = v.rotate(
    Vec3::new(0.0, 1.0, 0.0),
    Degrees(90.0)
);
```

## Angles

Mayth supports multiple angle units while also providing type safety.

```rs
use mayth::angle::{Degrees, Radians, Turns, Gradians};

let d = Degrees(180.0);

let r: Radians = d.into();
let t: Turns = d.into();
let g: Gradians = d.into();
```

All conversions are lossless and fully inter-compatible.

## Current Progress (0.0.1-alpha)

* [x] Vec2
* [x] Vec3
* [x] Matrix4 (in progress)
* [ ] Matrix3
* [ ] Matrix2
* [ ] Quaternions

## Status

This project is in early development. APIs may change frequently.

