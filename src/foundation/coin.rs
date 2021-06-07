use super::number::*;
use nalgebra::Matrix2;
pub type Coin2 = Matrix2<Complex>;

pub struct CoinParams {
    delta: Real,
    absa: Real,
    arga: Real,
    alpha: Complex,
    argb: Real,
    beta: Complex,
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

#[test]
fn test() {}
