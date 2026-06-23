use std::f32::consts::{PI, TAU};

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct Radians(pub f32);

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct Degrees(pub f32);

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct Turns(pub f32);

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct Gradians(pub f32);

impl From<Degrees> for Radians {
    fn from(d: Degrees) -> Self {
        Radians(d.0.to_radians())
    }
}

impl From<Radians> for Degrees {
    fn from(r: Radians) -> Self {
        Degrees(r.0.to_degrees())
    }
}

impl From<Turns> for Radians {
    fn from(t: Turns) -> Self {
        Radians(t.0 * TAU)
    }
}

impl From<Radians> for Turns {
    fn from(r: Radians) -> Self {
        Turns(r.0 / TAU)
    }
}

impl From<Gradians> for Radians {
    fn from(g: Gradians) -> Self {
        Radians(g.0 * PI / 200.0)
    }
}

impl From<Radians> for Gradians {
    fn from(r: Radians) -> Self {
        Gradians(r.0 * 200.0 / PI)
    }
}

impl From<Degrees> for Turns {
    fn from(d: Degrees) -> Self {
        Radians::from(d).into()
    }
}

impl From<Turns> for Degrees {
    fn from(t: Turns) -> Self {
        Radians::from(t).into()
    }
}

impl From<Degrees> for Gradians {
    fn from(d: Degrees) -> Self {
        Radians::from(d).into()
    }
}

impl From<Gradians> for Degrees {
    fn from(g: Gradians) -> Self {
        Radians::from(g).into()
    }
}

impl From<Turns> for Gradians {
    fn from(t: Turns) -> Self {
        Radians::from(t).into()
    }
}

impl From<Gradians> for Turns {
    fn from(g: Gradians) -> Self {
        Radians::from(g).into()
    }
}

impl From<Radians> for f32 {
    fn from(r: Radians) -> f32 {
        r.0
    }
}

impl From<f32> for Radians {
    fn from(f: f32) -> Radians {
        Radians(f)
    }
}
