#![allow(non_upper_case_globals)]
use nalgebra::ComplexField;
use nalgebra::{Complex as comp, Normed};
pub type Real = f64;
pub type Complex = comp<Real>;
pub static I: Complex = comp::new(0., 1.);
pub static zero: Complex = comp::new(0., 0.);
pub static pi: Real = std::f64::consts::PI;
pub const sqrt: fn(Real) -> Real = Real::sqrt;
pub const cos: fn(Real) -> Real = Real::cos;
pub const sin: fn(Real) -> Real = Real::sin;
pub const cosh: fn(Real) -> Real = Real::cosh;
pub const sinh: fn(Real) -> Real = Real::sinh;
pub const abs: fn(Real) -> Real = Real::abs;
pub const arg: fn(Complex) -> Real = comp::argument;
pub const exp: fn(Complex) -> Complex = comp::exp;
pub fn norm2(x: Complex) -> Real {
    x.norm_sqr()
}

pub fn norm(x: Complex) -> Real {
    x.norm()
}
