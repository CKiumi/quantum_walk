use crate::foundation::number::*;
pub fn eigenvalues(am: Real, ap: Real, p: Real, γ: Real) -> Vec<Real> {
    let bm = sqrt(1. - am * am);
    let bp = sqrt(1. - ap * ap);
    let q = sqrt(1. - p * p);
    let x = (p / q) * cosh(2. * γ);
    let mut evs = vec![0., 0., 0., 0.];
    let M = am / bm;
    let P = ap / bp;
    if M < x && x < P {
        evs[0] = p * sinh(2. * γ) + sqrt(1. + (p * sinh(2. * γ)).powi(2))
    }
    if -P < x && x < -M {
        evs[1] = p * sinh(2. * γ) - sqrt(1. + (p * sinh(2. * γ)).powi(2))
    }
    if P < x && x < M {
        evs[2] = -p * sinh(2. * γ) + sqrt(1. + (p * sinh(2. * γ)).powi(2))
    }
    if -M < x && x < -P {
        evs[3] = -p * sinh(2. * γ) - sqrt(1. + (p * sinh(2. * γ)).powi(2))
    }
    evs
}
