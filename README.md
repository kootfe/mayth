# Mayth

![Rust](https://img.shields.io/badge/rust-%23000000.svg?style=for-the-badge\&logo=rust\&logoColor=white)
![Crates.io Total Downloads](https://img.shields.io/crates/d/mayth?style=for-the-badge\&label=Crates.io%20Downloads)
![Crates.io Version](https://img.shields.io/crates/v/mayth?style=for-the-badge)

*A type-safe 'maythical' math library for games and physics, written in Rust.*
We aim to provide simple and practical math types for game development to physics applications, and even for your cat to calculate the perfect angle for pushing items off a table.

## Installing

```bash
cargo add mayth
```

or

```toml
[dependencies]
mayth = "0.0.1-alpha.6"
```

## Example

```rs
use mayth::vec::Vec3;
use mayth::angle::Degrees;

let v = Vec3::new(1.0, 0.0, 0.0);

// rotate v around the Y axis.
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
Every unit implements `From` for every other unit, so you can convert freely with `.into()` or `Type::from(x)`.

## Current Progress (0.0.1-alpha)

* [x] Angle system
* [x] Vec2
* [x] Vec3
* [ ] Vec4
* [x] Quaternions
* [ ] Matrix2
* [ ] Matrix3
* [~] Matrix4 (in progress)


## Current Internal Progress (0.0.1-alpha)
* [~] SIMD (SSE2) (curretly just for Matrix4)
* [ ] SIMD (NEON)
* [ ] SIMD (AVX2)
* [ ] SIMD (RVV)

## Status
This project is in early development. APIs may change frequently.

