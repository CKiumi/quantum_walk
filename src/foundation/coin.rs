use super::number::*;
use nalgebra::{Matrix2, Matrix3};
pub type Coin2 = Matrix2<Complex>;
pub type Coin3 = Matrix3<Complex>;
use nalgebra::{Vector2, Vector3};
pub type QuantumState2 = Vector2<Complex>;
pub type QuantumState3 = Vector3<Complex>;

pub trait CoinOperation {
    fn from_param(delta: Real, absa: Real, arga: Real, argb: Real) -> Self;
}

impl CoinOperation for Coin2 {
    fn from_param(delta: Real, absa: Real, arga: Real, argb: Real) -> Self {
        let absb = sqrt(1. - absa.powi(2));
        let a: Complex = absa * exp(I * arga);
        let b: Complex = absb * exp(I * argb);
        Matrix2::new(a, b, -b.conj(), a.conj()).map(|x| exp(I * delta) * x)
    }
}

pub fn create_machida_coin(theta: Real, delta: Real) -> Coin3 {
    let (c, s) = (cos(theta), sin(theta));
    let (m1, m2, m3) = (-(1. + c) / 2., (1. - c) / 2., s / sqrt(2.));
    Matrix3::new(m1, m3, m2, m3, c, m3, m2, m3, m1).map(|x| exp(I * delta) * x)
}
