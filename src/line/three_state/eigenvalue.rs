use crate::foundation::number::*;

pub fn one_defect(Δ: Real, θ: Real, θ0: Real) -> Vec<Complex> {
    let c = cos(θ);
    let c0 = cos(θ0);
    let x = 1. - c + 2. * c0;
    if c < c0 {
        let ev: Complex = (c + c0 * c0 + I * (1. + c0) * sqrt(x - (c + c0 * c0))) / x;
        vec![exp(I * Δ), ev * exp(I * Δ), ev.conj() * exp(I * Δ)]
    } else {
        vec![exp(I * Δ), zero, zero]
    }
}

pub fn two_phase(Δm: Real, Δp: Real, θ: Real) -> Vec<Complex> {
    let c = cos(θ);
    let x = sin(Δm - Δp) / sqrt(2. * (1. - cos(Δm - Δp)));
    let mut evs = vec![exp(I * Δm), exp(I * Δp), zero, zero];
    if x > -c {
        evs[2] = I * (exp(I * Δp) - exp(I * Δm)) / sqrt(2. * (1. - cos(Δm - Δp)));
    }
    if x < c {
        evs[3] = -I * (exp(I * Δp) - exp(I * Δm)) / sqrt(2. * (1. - cos(Δm - Δp)));
    }
    evs
}
