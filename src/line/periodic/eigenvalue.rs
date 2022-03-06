use crate::foundation::number::*;

pub fn one_defect(Δ1: Real, Δ2: Real, absβ: Real, argβ1: Real, argβ2: Real) -> Vec<Complex> {
    let β1 = absβ * exp(I * argβ1);
    let β2 = absβ * exp(I * argβ2);
    let ib1b2 = (β1 * β2.conj()).im;
    let Bm = (absβ + sqrt(absβ * absβ - ib1b2 * ib1b2)) / (2. * absβ);
    let Bp = (absβ - sqrt(absβ * absβ - ib1b2 * ib1b2)) / (2. * absβ);
    let evp: Complex = exp((I * (Δ1 + Δ2)) / 2.) * (sqrt(Bp) + I * sqrt(Bm));
    let evm: Complex = exp((I * (Δ1 + Δ2)) / 2.) * (-sqrt(Bp) + I * sqrt(Bm));
    match ib1b2 {
        x if x < 0. => vec![evp, -evp, I * evm, -I * evm],
        _ => vec![evm, -evm, I * evp, -I * evp],
    }
}
pub fn two_phase1(Δ1: Real, Δ2: Real, βm: Complex, βp: Complex) -> Vec<Complex> {
    let rbmbp = (βm * βp.conj()).re;
    let ibmbp = (βm * βp.conj()).im;
    let Bp = (norm(βm - βp) + sqrt(norm2(βm - βp) - ibmbp * ibmbp)) / (2. * norm(βm - βp));
    let Bm = (norm(βm - βp) - sqrt(norm2(βm - βp) - ibmbp * ibmbp)) / (2. * norm(βm - βp));
    if rbmbp < norm2(βm) && rbmbp < norm2(βp) {
        let evp: Complex = exp((I * (Δ1 + Δ2)) / 2.) * (sqrt(Bp) + I * sqrt(Bm));
        let evm: Complex = exp((I * (Δ1 + Δ2)) / 2.) * (sqrt(Bp) - I * sqrt(Bm));
        match ibmbp {
            x if x < 0. => vec![evm, -evm, I * evm, -I * evm],
            _ => vec![evp, -evp, I * evp, -I * evp],
        }
    } else {
        vec![zero, zero, zero, zero]
    }
}

pub fn two_phase2(Δ1: Real, Δ2: Real, βm: Complex, βp: Complex) -> Vec<Complex> {
    let rbb0 = (βm * βp.conj()).re;
    let ibb0 = (βm * βp.conj()).im;
    let cosθ = 1. - (2. * ibb0 * ibb0) / norm2(βm - βp);
    let condm = (rbb0 - norm2(βm)) * (cosθ + norm2(βm));
    let condp = (rbb0 - norm2(βp)) * (cosθ + norm2(βp));
    if condm < 0. && condp < 0. {
        let ev: Complex = exp((I * (Δ1 + Δ2)) / 2.) * (cosθ + I * sqrt(1. - cosθ * cosθ));
        vec![ev, -ev]
    } else if condm > 0. && condp > 0. {
        let ev: Complex = exp((I * (Δ1 + Δ2)) / 2.) * (cosθ - I * sqrt(1. - cosθ * cosθ));
        vec![ev, -ev]
    } else {
        vec![zero, zero]
    }
}
