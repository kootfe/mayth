use std::ops::{
    Add, AddAssign, Div, DivAssign, Index, IndexMut, Mul, MulAssign, Neg, Sub, SubAssign,
};

use crate::{angle::Radians, vec::Vec3};

#[derive(Clone, Copy, Debug)]
pub struct Matrix4 {
    pub cols: [[f32; 4]; 4],
}

impl Matrix4 {
    pub const IDENTITY: Matrix4 = Matrix4 {
        cols: [
            [1.0, 0.0, 0.0, 0.0],
            [0.0, 1.0, 0.0, 0.0],
            [0.0, 0.0, 1.0, 0.0],
            [0.0, 0.0, 0.0, 1.0],
        ],
    };

    pub fn from_translation(t: Vec3) -> Self {
        let mut tmp = Self::IDENTITY;
        tmp[3] = [t.x, t.y, t.z, 1.0];
        tmp
    }

    pub fn from_scale(s: Vec3) -> Self {
        let mut tmp = Self::IDENTITY;
        tmp[0][0] = s.x;
        tmp[1][1] = s.y;
        tmp[2][2] = s.z;
        tmp
    }

    pub fn from_rotation_z(angle: impl Into<Radians>) -> Self {
        let Radians(rad) = angle.into();
        let mut tmp = Self::IDENTITY;

        let c = rad.cos();
        let s = rad.sin();
        tmp[0][0] = c;
        tmp[0][1] = s;
        tmp[1][0] = -s;
        tmp[1][1] = c;
        tmp
    }

    pub fn from_rotation_x(angle: impl Into<Radians>) -> Self {
        let Radians(rad) = angle.into();
        let mut tmp = Self::IDENTITY;

        let c = rad.cos();
        let s = rad.sin();
        tmp[1][1] = c;
        tmp[1][2] = s;
        tmp[2][1] = -s;
        tmp[2][2] = c;
        tmp
    }

    pub fn from_rotation_y(angle: impl Into<Radians>) -> Self {
        let Radians(rad) = angle.into();
        let mut tmp = Self::IDENTITY;

        let c = rad.cos();
        let s = rad.sin();
        tmp[0][0] = c;
        tmp[0][2] = -s;
        tmp[2][0] = s;
        tmp[2][2] = c;
        tmp
    }

    pub fn from_axis_angle(axis: Vec3, angle: impl Into<Radians>) -> Self {
        let Radians(rad) = angle.into();
        let mut tmp = Self::IDENTITY;
        let c = rad.cos();
        let s = rad.sin();
        let t = 1.0 - c;
        let x = axis.x;
        let y = axis.y;
        let z = axis.z;

        tmp[0][0] = t * x * x + c;
        tmp[0][1] = t * x * y + s * z;
        tmp[0][2] = t * x * z - s * y;

        tmp[1][0] = t * x * y - s * z;
        tmp[1][1] = t * y * y + c;
        tmp[1][2] = t * y * z + s * x;

        tmp[2][0] = t * x * z + s * y;
        tmp[2][1] = t * y * z - s * x;
        tmp[2][2] = t * z * z + c;

        tmp
    }

    fn mul_impl(a: &Matrix4, b: &Matrix4) -> Matrix4 {
        let mut out = Matrix4 {
            cols: [[0.0; 4]; 4],
        };
        for c in 0..4 {
            for r in 0..4 {
                let mut sum = 0.0;
                for k in 0..4 {
                    sum += a.cols[k][r] * b.cols[c][k];
                }
                out.cols[c][r] = sum;
            }
        }
        out
    }
}

macro_rules! impl_mat4_binop {
    ($trait:ident, $method:ident, $op:tt) => {
        impl $trait<Matrix4> for Matrix4 {
            type Output = Matrix4;
            fn $method(self, rhs: Matrix4) -> Matrix4 {
                let mut out = Matrix4 { cols: [[0.0; 4]; 4] };
                for c in 0..4 {
                    for r in 0..4 {
                        out.cols[c][r] = self.cols[c][r] $op rhs.cols[c][r];
                    }
                }
                out
            }
        }
        impl $trait<&Matrix4> for Matrix4 {
            type Output = Matrix4;
            fn $method(self, rhs: &Matrix4) -> Matrix4 {
                let mut out = Matrix4 { cols: [[0.0; 4]; 4] };
                for c in 0..4 {
                    for r in 0..4 {
                        out.cols[c][r] = self.cols[c][r] $op rhs.cols[c][r];
                    }
                }
                out
            }
        }
        impl $trait<&mut Matrix4> for Matrix4 {
            type Output = Matrix4;
            fn $method(self, rhs: &mut Matrix4) -> Matrix4 {
                let mut out = Matrix4 { cols: [[0.0; 4]; 4] };
                for c in 0..4 {
                    for r in 0..4 {
                        out.cols[c][r] = self.cols[c][r] $op rhs.cols[c][r];
                    }
                }
                out
            }
        }
        impl $trait<Matrix4> for &Matrix4 {
            type Output = Matrix4;
            fn $method(self, rhs: Matrix4) -> Matrix4 {
                let mut out = Matrix4 { cols: [[0.0; 4]; 4] };
                for c in 0..4 {
                    for r in 0..4 {
                        out.cols[c][r] = self.cols[c][r] $op rhs.cols[c][r];
                    }
                }
                out
            }
        }
        impl $trait<&Matrix4> for &Matrix4 {
            type Output = Matrix4;
            fn $method(self, rhs: &Matrix4) -> Matrix4 {
                let mut out = Matrix4 { cols: [[0.0; 4]; 4] };
                for c in 0..4 {
                    for r in 0..4 {
                        out.cols[c][r] = self.cols[c][r] $op rhs.cols[c][r];
                    }
                }
                out
            }
        }
        impl $trait<&mut Matrix4> for &Matrix4 {
            type Output = Matrix4;
            fn $method(self, rhs: &mut Matrix4) -> Matrix4 {
                let mut out = Matrix4 { cols: [[0.0; 4]; 4] };
                for c in 0..4 {
                    for r in 0..4 {
                        out.cols[c][r] = self.cols[c][r] $op rhs.cols[c][r];
                    }
                }
                out
            }
        }
        impl $trait<Matrix4> for &mut Matrix4 {
            type Output = Matrix4;
            fn $method(self, rhs: Matrix4) -> Matrix4 {
                let mut out = Matrix4 { cols: [[0.0; 4]; 4] };
                for c in 0..4 {
                    for r in 0..4 {
                        out.cols[c][r] = self.cols[c][r] $op rhs.cols[c][r];
                    }
                }
                out
            }
        }
        impl $trait<&Matrix4> for &mut Matrix4 {
            type Output = Matrix4;
            fn $method(self, rhs: &Matrix4) -> Matrix4 {
                let mut out = Matrix4 { cols: [[0.0; 4]; 4] };
                for c in 0..4 {
                    for r in 0..4 {
                        out.cols[c][r] = self.cols[c][r] $op rhs.cols[c][r];
                    }
                }
                out
            }
        }
        impl $trait<&mut Matrix4> for &mut Matrix4 {
            type Output = Matrix4;
            fn $method(self, rhs: &mut Matrix4) -> Matrix4 {
                let mut out = Matrix4 { cols: [[0.0; 4]; 4] };
                for c in 0..4 {
                    for r in 0..4 {
                        out.cols[c][r] = self.cols[c][r] $op rhs.cols[c][r];
                    }
                }
                out
            }
        }
    };
}

macro_rules! impl_mat4_assignop {
    ($trait:ident, $method:ident, $op:tt) => {
        impl $trait<Matrix4> for Matrix4 {
            fn $method(&mut self, rhs: Matrix4) {
                for c in 0..4 {
                    for r in 0..4 {
                        self.cols[c][r] $op rhs.cols[c][r];
                    }
                }
            }
        }
        impl $trait<&Matrix4> for Matrix4 {
            fn $method(&mut self, rhs: &Matrix4) {
                for c in 0..4 {
                    for r in 0..4 {
                        self.cols[c][r] $op rhs.cols[c][r];
                    }
                }
            }
        }
        impl $trait<&mut Matrix4> for Matrix4 {
            fn $method(&mut self, rhs: &mut Matrix4) {
                for c in 0..4 {
                    for r in 0..4 {
                        self.cols[c][r] $op rhs.cols[c][r];
                    }
                }
            }
        }
    };
}

macro_rules! impl_mat4_scalar {
    ($trait:ident, $method:ident, $op:tt) => {
        impl $trait<f32> for Matrix4 {
            type Output = Matrix4;
            fn $method(self, rhs: f32) -> Matrix4 {
                let mut out = Matrix4 { cols: [[0.0; 4]; 4] };
                for c in 0..4 {
                    for r in 0..4 {
                        out.cols[c][r] = self.cols[c][r] $op rhs;
                    }
                }
                out
            }
        }
        impl $trait<f32> for &Matrix4 {
            type Output = Matrix4;
            fn $method(self, rhs: f32) -> Matrix4 {
                let mut out = Matrix4 { cols: [[0.0; 4]; 4] };
                for c in 0..4 {
                    for r in 0..4 {
                        out.cols[c][r] = self.cols[c][r] $op rhs;
                    }
                }
                out
            }
        }
        impl $trait<f32> for &mut Matrix4 {
            type Output = Matrix4;
            fn $method(self, rhs: f32) -> Matrix4 {
                let mut out = Matrix4 { cols: [[0.0; 4]; 4] };
                for c in 0..4 {
                    for r in 0..4 {
                        out.cols[c][r] = self.cols[c][r] $op rhs;
                    }
                }
                out
            }
        }
    };
}

macro_rules! impl_mat4_scalar_assign {
    ($trait:ident, $method:ident, $op:tt) => {
        impl $trait<f32> for Matrix4 {
            fn $method(&mut self, rhs: f32) {
                for c in 0..4 {
                    for r in 0..4 {
                        self.cols[c][r] $op rhs;
                    }
                }
            }
        }
    };
}

macro_rules! impl_mat4_neg {
    () => {
        impl Neg for Matrix4 {
            type Output = Matrix4;
            fn neg(self) -> Matrix4 {
                let mut out = Matrix4 {
                    cols: [[0.0; 4]; 4],
                };
                for c in 0..4 {
                    for r in 0..4 {
                        out.cols[c][r] = -self.cols[c][r];
                    }
                }
                out
            }
        }
        impl Neg for &Matrix4 {
            type Output = Matrix4;
            fn neg(self) -> Matrix4 {
                let mut out = Matrix4 {
                    cols: [[0.0; 4]; 4],
                };
                for c in 0..4 {
                    for r in 0..4 {
                        out.cols[c][r] = -self.cols[c][r];
                    }
                }
                out
            }
        }
        impl Neg for &mut Matrix4 {
            type Output = Matrix4;
            fn neg(self) -> Matrix4 {
                let mut out = Matrix4 {
                    cols: [[0.0; 4]; 4],
                };
                for c in 0..4 {
                    for r in 0..4 {
                        out.cols[c][r] = -self.cols[c][r];
                    }
                }
                out
            }
        }
    };
}

impl_mat4_binop!(Add, add, +);
impl_mat4_binop!(Sub, sub, -);

impl_mat4_assignop!(AddAssign, add_assign, +=);
impl_mat4_assignop!(SubAssign, sub_assign, -=);

impl_mat4_scalar!(Mul, mul, *);
impl_mat4_scalar!(Div, div, /);

impl_mat4_scalar_assign!(MulAssign, mul_assign, *=);
impl_mat4_scalar_assign!(DivAssign, div_assign, /=);

impl_mat4_neg!();

//I hate this...
macro_rules! impl_mat4_mul {
    () => {
        impl Mul<Matrix4> for Matrix4 {
            type Output = Matrix4;
            fn mul(self, rhs: Matrix4) -> Matrix4 {
                Matrix4::mul_impl(&self, &rhs)
            }
        }
        impl Mul<&Matrix4> for Matrix4 {
            type Output = Matrix4;
            fn mul(self, rhs: &Matrix4) -> Matrix4 {
                Matrix4::mul_impl(&self, rhs)
            }
        }
        impl Mul<&mut Matrix4> for Matrix4 {
            type Output = Matrix4;
            fn mul(self, rhs: &mut Matrix4) -> Matrix4 {
                Matrix4::mul_impl(&self, rhs)
            }
        }
        impl Mul<Matrix4> for &Matrix4 {
            type Output = Matrix4;
            fn mul(self, rhs: Matrix4) -> Matrix4 {
                Matrix4::mul_impl(self, &rhs)
            }
        }
        impl Mul<&Matrix4> for &Matrix4 {
            type Output = Matrix4;
            fn mul(self, rhs: &Matrix4) -> Matrix4 {
                Matrix4::mul_impl(self, rhs)
            }
        }
        impl Mul<&mut Matrix4> for &Matrix4 {
            type Output = Matrix4;
            fn mul(self, rhs: &mut Matrix4) -> Matrix4 {
                Matrix4::mul_impl(self, rhs)
            }
        }
        impl Mul<Matrix4> for &mut Matrix4 {
            type Output = Matrix4;
            fn mul(self, rhs: Matrix4) -> Matrix4 {
                Matrix4::mul_impl(self, &rhs)
            }
        }
        impl Mul<&Matrix4> for &mut Matrix4 {
            type Output = Matrix4;
            fn mul(self, rhs: &Matrix4) -> Matrix4 {
                Matrix4::mul_impl(self, rhs)
            }
        }
        impl Mul<&mut Matrix4> for &mut Matrix4 {
            type Output = Matrix4;
            fn mul(self, rhs: &mut Matrix4) -> Matrix4 {
                Matrix4::mul_impl(self, rhs)
            }
        }
    };
}

impl_mat4_mul!();

impl Index<usize> for Matrix4 {
    type Output = [f32; 4];
    fn index(&self, c: usize) -> &[f32; 4] {
        &self.cols[c]
    }
}

impl IndexMut<usize> for Matrix4 {
    fn index_mut(&mut self, c: usize) -> &mut [f32; 4] {
        &mut self.cols[c]
    }
}
