use crate::foundation::{number::*, QuantumState2};

pub fn eigenvalues(Δ: Real, absβ: Real, c: Real) -> Vec<Complex> {
    let mut ev = vec![zero, zero, zero, zero];
    if sin(Δ - c) < absβ {
        let x = exp(I * Δ) - I * absβ * exp(I * c);
        ev[0] = x / norm(x);
        ev[1] = -x / norm(x);
    }
    if sin(Δ - c) > -absβ {
        let x = exp(I * Δ) + (I * absβ) * exp(I * c);
        ev[2] = x / norm(x);
        ev[3] = -x / norm(x);
    }
    ev
}

pub fn result(
    n: i32,
    Δ: Real,
    Δ0: Real,
    absβ: Real,
    argαo: Real,
    argβm: Real,
    argβp: Real,
    φ: QuantumState2,
) -> Vec<Real> {
    let C = Δ0 + (argβp - argβm) / 2.;
    let (absα, αo) = (sqrt(1. - absβ.powi(2)), exp(I * argαo));
    let βm = absβ * exp(I * argβm);
    let evs = eigenvalues(Δ, absβ, C);
    let imabp1p2 = (exp(I * (Δ0 - C)) * αo * βm.conj() * φ[0] * φ[1].conj()).im;
    let mut r1 = vec![0.; (2 * n + 1) as usize];
    let mut r2 = vec![0.; (2 * n + 1) as usize];
    if evs[0] != zero {
        let A1 = 1. - 2. * absβ * (Δ - C).sin() + absβ.powi(2);
        let N1 = (2. * absβ * A1) / (absβ - (Δ - C).sin());
        let μ0_1 = (4. * absβ.powi(3) * (absβ + 2. * imabp1p2)) / (N1 * N1);
        let X_1 = (1. - (absβ * (Δ - C).sin())) / absα.powi(2);
        let (ζp2_1, ζm2_1) = (absα.powi(2) / A1, A1 / absα.powi(2));
        r1 = (-n..=n)
            .map(|x| match x {
                x if x > 0 => μ0_1 * X_1 * ζp2_1.powi(x),
                x if x < 0 => μ0_1 * X_1 * ζm2_1.powi(x),
                _ => μ0_1,
            })
            .collect();
    }
    if evs[2] != zero {
        let A2 = 1. + 2. * absβ * (Δ - C).sin() + absβ.powi(2);
        let N2 = (2. * absβ * A2) / ((Δ - C).sin() + absβ);
        let μ0_2 = (4. * absβ.powi(3) * (absβ - 2. * imabp1p2)) / (N2 * N2);
        let X_2 = (1. + (absβ * (Δ - C).sin())) / absα.powi(2);
        let (ζp2_2, ζm2_2) = (absα.powi(2) / A2, A2 / absα.powi(2));
        r2 = (-n..=n)
            .map(|x| match x {
                x if x > 0 => μ0_2 * X_2 * ζp2_2.powi(x),
                x if x < 0 => μ0_2 * X_2 * ζm2_2.powi(x),
                _ => μ0_2,
            })
            .collect();
    }
    let mut our_result = vec![0.; (2 * n + 1) as usize];
    for x in 0..2 * n + 1 {
        our_result[x as usize] = r1[x as usize] + r2[x as usize];
    }
    // println!("{:?}", our_result);
    our_result
}
