use super::number::*;
use nalgebra::{Matrix2, Matrix3};
pub type Coin2 = Matrix2<Complex>;
pub type Coin3 = Matrix3<Complex>;
use nalgebra::{Vector2, Vector3};
pub type QuantumState2 = Vector2<Complex>;
pub type QuantumState3 = Vector3<Complex>;

pub trait Coin {
    fn from_param(delta: Real, absa: Real, arga: Real, argb: Real) -> Self;
}

impl Coin for Coin2 {
    fn from_param(delta: Real, absa: Real, arga: Real, argb: Real) -> Self {
        let absb = sqrt(1. - absa.powi(2));
        let a: Complex = absa * exp(I * arga);
        let b: Complex = absb * exp(I * argb);
        Matrix2::new(a, b, -b.conj(), a.conj()).map(|x| exp(I * delta) * x)
    }
}

pub trait State<P> {
    fn from_arr(param: P) -> Self;
    fn from_param(absa: Real, arga: Real, argb: Real) -> Self;
}

impl State<[[f64; 2]; 2]> for QuantumState2 {
    fn from_arr(param: [[f64; 2]; 2]) -> Self {
        QuantumState2::new(
            Complex::new(param[0][0], param[0][1]),
            Complex::new(param[1][0], param[1][1]),
        )
    }
    fn from_param(absa: Real, arga: Real, argb: Real) -> Self {
        QuantumState2::new(
            absa * exp(I * arga),
            sqrt(1. - absa.powi(2)) * exp(I * argb),
        )
    }
}

pub fn create_machida_coin(theta: Real, delta: Real) -> Coin3 {
    let (c, s) = (cos(theta), sin(theta));
    let (m1, m2, m3) = (-(1. + c) / 2., (1. - c) / 2., s / sqrt(2.));
    Matrix3::new(m1, m3, m2, m3, c, m3, m2, m3, m1).map(|x| exp(I * delta) * x)
}

pub fn norm_qs(states: &[QuantumState2]) -> Vec<Real> {
    states.iter().map(|x| x.norm_squared()).collect()
}
