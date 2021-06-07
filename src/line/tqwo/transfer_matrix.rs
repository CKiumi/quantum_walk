use crate::foundation::number::*;
use crate::foundation::QuantumState2;
use nalgebra::Matrix2;

pub fn new(alpha: Complex, beta: Complex, delta: Real, ev: Complex) -> Matrix2<Complex> {
    Matrix2::new(
        ev * exp(I * -delta),
        -beta,
        -beta.conj(),
        (ev * exp(I * -delta)).conj(),
    )
    .map(|x| x / alpha)
}

pub fn ev(alpha: Complex, delta: Real, ev: Complex, sign: i8) -> Complex {
    let cosld = (ev * exp(I * -delta)).re;
    (cosld + (sign as f64) * (cosld.powi(2) - norm2(alpha)).sqrt()) / alpha
}

pub fn evec(alpha: Complex, beta: Complex, delta: Real, ev: Complex, sign: i8) -> QuantumState2 {
    let cosld = (ev * exp(I * -delta)).re;
    let sinld = (ev * exp(I * -delta)).im;
    QuantumState2::new(
        beta,
        I * sinld - (sign as f64) * sqrt(cosld.powi(2) - norm2(alpha)),
    )
}
