use crate::foundation::{number::*, QuantumState2};

pub fn eigenvalues(Δ: Real, β: Complex, βo: Complex) -> Vec<Complex> {
    let rbb0 = (β * βo.conj()).re;
    if norm2(β) > rbb0 {
        let ev: Complex =
            ((rbb0 - 1.0) + sqrt(norm2(β) - rbb0 * rbb0) * I) / sqrt(1.0 + norm2(β) - 2.0 * rbb0);
        let evs = vec![ev, -ev, ev.conj(), -ev.conj()];
        evs.iter().map(|x| exp(I * Δ) * x).collect()
    } else {
        vec![zero, zero, zero, zero]
    }
}

pub fn time_averaged(
    n: i32,
    Δ: Real,
    α: Complex,
    β: Complex,
    αo: Complex,
    βo: Complex,
    φ: QuantumState2,
) -> Vec<Real> {
    let evs = eigenvalues(Δ, β, βo);
    if evs[0] == zero {
        return vec![0.; (2 * n + 1) as usize];
    }
    let Complex { re: rbb0, im: ibb0 } = β * βo.conj();
    let (A, B, C) = (1. - rbb0, norm2(β) - rbb0, norm2(β) - rbb0.powi(2));
    let μ0 = 2. * ((B / (A + B)).powi(2));
    let imabp1p2 = (αo * β.conj() * φ[0] * φ[1].conj()).im;
    let P = (A / (norm2(α) * C))
        * (norm2(αo) * norm2(β) + 2. * ibb0.powi(2) * norm2(φ[0]) + 2. * ibb0 * imabp1p2);
    let M = (A / (norm2(α) * C))
        * (norm2(αo) * norm2(β) + 2. * ibb0.powi(2) * norm2(φ[1]) - 2. * ibb0 * imabp1p2);
    let (ζp2, ζm2) = (norm2(α) / (A + B), (A + B) / norm2(α));
    let our_result: Vec<Real> = (-n..=n)
        .map(|x| match x {
            x if x > 0 => μ0 * P * ζp2.powi(x),
            x if x < 0 => μ0 * M * ζm2.powi(x),
            _ => μ0,
        })
        .collect();
    // println!("{:?}", our_result);
    our_result
}
