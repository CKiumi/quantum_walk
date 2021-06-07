use crate::foundation::number::*;
static π: f64 = std::f64::consts::PI;

pub fn limit(x: f64, Δ: f64, φ1: f64, φ2: f64, φ3: f64, φ4: f64) -> f64 {
    let p = (cos(Δ) - sin(Δ)) / sqrt(2.);
    let q = (cos(Δ) + sin(Δ)) / sqrt(2.);
    let d0 = abs(φ1 - φ2).powi(2) + abs(φ3 - φ4).powi(2);
    let d1 = p * (abs(φ2 - φ4).powi(2) - abs(φ1 - φ3).powi(2))
        + q * (abs(φ2 + φ3).powi(2) - abs(φ1 + φ4).powi(2));
    let d2 = p * q * (abs(φ1 + φ2).powi(2) - abs(φ3 + φ4).powi(2))
        + 2.0 * p * p * ((φ1 - φ4) * (φ2 - φ3))
        + 2.0 * q * q * ((φ1 + φ3) * (φ2 + φ4));
    (sqrt(1.0 - p * p) * (p * p * d0 + p * d1 * x + d2 * x * x))
        / (2.0 * π * p * p * sqrt(p * p - x * x) * (1.0 - x * x))
}
