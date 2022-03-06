use super::number::*;
use nalgebra::{Matrix2, Matrix3};
pub type Coin2 = Matrix2<Complex>;
pub type Coin3 = Matrix3<Complex>;

pub struct CoinParams {
    pub delta: Real,
    pub absa: Real,
    pub arga: Real,
    pub alpha: Complex,
    pub argb: Real,
    pub beta: Complex,
}
impl CoinParams {
    pub fn new(delta: Real, absa: Real, arga: Real, argb: Real) -> CoinParams {
        CoinParams {
            delta,
            absa,
            arga,
            alpha: absa * exp(I * arga),
            argb: absa,
            beta: absa * exp(I * argb),
        }
    }
}
pub trait CoinOperation {
    fn from_param(delta: Real, absa: Real, arga: Real, argb: Real) -> Self;
}

impl CoinOperation for Coin2 {
    fn from_param(delta: Real, absa: Real, arga: Real, argb: Real) -> Self {
        let absb = (1. - absa.powi(2)).sqrt();
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

#[test]
fn test() {}
