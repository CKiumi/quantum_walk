use crate::foundation::{number::*, QuantumState2};

pub fn eigenvalues(Δo: Real, Δ: Real, absβ: Real) -> Vec<Complex> {
    let α = sqrt(1. - absβ * absβ);
    let mut ev: Vec<Complex> = vec![zero, zero, zero, zero];
    if absβ * cos(Δo - Δ) - α * sin(Δo - Δ) < absβ {
        let x = absβ * (absβ + I * α) * exp(I * Δo) - exp(I * Δ);
        ev[0] = x / norm(x);
        ev[1] = -x / norm(x);
    }
    if absβ * cos(Δo - Δ) + α * sin(Δo - Δ) < absβ {
        let x = absβ * (absβ - I * α) * exp(I * Δo) - exp(I * Δ);
        ev[2] = x / norm(x);
        ev[3] = -x / norm(x);
    };
    ev
}

pub fn eigenvector_con1(
    n: i32,
    Δo: Real,
    Δ: Real,
    α: Complex,
    β: Complex,
    argαo: Real,
    ev: &Complex,
) -> Vec<QuantumState2> {
    let s: f64 = if (ev * exp(-I * Δ)).re > 0. { 1. } else { -1. };
    let αo = sqrt(1. - norm2(β)) * exp(I * argαo);
    let a = norm(β) * cos(Δo - Δ) - norm(α) * sin(Δo - Δ);
    let b = norm(β) * sin(Δo - Δ) + norm(α) * cos(Δo - Δ);
    let (A, B, C) = (1. - norm(β) * a, norm(β) * (norm(β) - a), norm(β) * b);
    let (ζm, ζp) = (s * sqrt(A + B) / α, s * α.conj() / sqrt(A + B));
    let N = (2. * norm2(β) * (A + B)) / B;
    let block1 = (-I * norm(α) * norm(β)) / (αo * α.conj() * β.conj());
    (-n..=n)
        .map(|x| match x {
            x if x > 0 => QuantumState2::new(α.conj() * (B + I * C), β.conj() * (A + B))
                .map(|e| block1 * e * ζp.powi(x)),
            x if x < 0 => {
                QuantumState2::new(β * (A + B), -α * (B + I * C)).map(|e| (e * ζm.powi(x)) / α)
            }
            _ => QuantumState2::new(I * norm(α) * norm(β), αo * β.conj())
                .map(|e| (-(B + I * C) * e) / (αo * β.conj())),
        })
        .map(|vec| vec.map(|e| (e * s) / (sqrt(N) * sqrt(A + B))))
        .collect()
}

pub fn time_averaged(
    n: i32,
    Δ0: Real,
    Δ: Real,
    α: Complex,
    β: Complex,
    argαo: Real,
    φ: QuantumState2,
) -> Vec<Real> {
    let αo = sqrt(1. - norm2(β)) * exp(I * argαo);
    let evs = eigenvalues(Δ0, Δ, norm(β));
    let imabp1p2 = (αo * β.conj() * φ[0] * φ[1].conj()).im;
    let length = (2 * n + 1) as usize;
    let (mut r1, mut r2) = (vec![0.; length], vec![0.; length]);
    if evs[0] != zero {
        let a1 = norm(β) * cos(Δ0 - Δ) - norm(α) * sin(Δ0 - Δ);
        let (A1, B1) = (1. - norm(β) * a1, norm(β) * (norm(β) - a1));
        let (ζm2_1, ζp2_1) = ((A1 + B1) / norm2(α), norm2(α) / (A1 + B1));
        let μ0_1 = (B1.powi(2) * (norm(α) * norm(β) + 2. * imabp1p2))
            / (norm(α) * norm(β) * (A1 + B1).powi(2));
        let X1 = A1 / norm2(α);
        r1 = (-n..=n)
            .map(|x| match x {
                x if x > 0 => μ0_1 * X1 * ζp2_1.powi(x),
                x if x < 0 => μ0_1 * X1 * ζm2_1.powi(x),
                _ => μ0_1,
            })
            .collect();
    }
    if evs[3] != zero {
        let a2 = norm(β) * cos(Δ0 - Δ) + norm(α) * sin(Δ0 - Δ);
        let (A2, B2) = (1. - norm(β) * a2, norm(β) * (norm(β) - a2));
        let (ζm2_2, ζp2_2) = ((A2 + B2) / norm2(α), norm2(α) / (A2 + B2));
        let μ0_2 = (B2 * B2 * (norm(α) * norm(β) - 2. * imabp1p2))
            / (norm(α) * norm(β) * (A2 + B2).powi(2));
        let X2 = A2 / norm2(α);
        r2 = (-n..=n)
            .map(|x| match x {
                x if x > 0 => μ0_2 * X2 * ζp2_2.powi(x),
                x if x < 0 => μ0_2 * X2 * ζm2_2.powi(x),
                _ => μ0_2,
            })
            .collect();
    }
    let mut our_result = vec![0.; length];
    for x in 0..length as usize {
        our_result[x] = r1[x] + r2[x];
    }
    // println!("{:?}", our_result);
    our_result
}
