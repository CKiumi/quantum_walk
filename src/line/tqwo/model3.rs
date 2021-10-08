use crate::foundation::{number::*, QuantumState2};

pub fn eigenvalues(Δp: Real, Δm: Real, absβp: Real, absβm: Real) -> Vec<Complex> {
    let (αp, αm) = (sqrt(1. - absβp * absβp), sqrt(1. - absβm * absβm));
    match cos(Δm - Δp) {
        x if x < absβp * absβm - αp * αm => {
            let x = absβp * exp(I * Δm) - absβm * exp(I * Δp);
            vec![x / norm(x), -x / norm(x)]
        }
        _ => vec![zero, zero],
    }
}

pub fn time_averaged(
    n: i32,
    Δp: Real,
    Δm: Real,
    absβm: Real,
    absβp: Real,
    argβ: Real,
    argαm: Real,
    argαp: Real,
    φ: QuantumState2,
) -> Vec<Real> {
    let evs = eigenvalues(Δp, Δm, absβp, absβm);
    if evs[0] == zero {
        return vec![0.; (2 * n + 1) as usize];
    }
    let (absαm, absαp) = (sqrt(1. - absβm.powi(2)), sqrt(1. - absβp.powi(2)));
    let (βp, βm) = (absβp * exp(I * argβ), absβm * exp(I * argβ));
    let (αm, αp) = (absαm * exp(I * argαm), absαp * exp(I * argαp));
    let (C, S) = (cos(Δp - Δm), sin(Δp - Δm));
    let A = absβp.powi(2) + absβm.powi(2) - 2. * absβm * absβp * C;
    let N = (absβm * A) / (absβp * sqrt(A - S));
    let root = sqrt(A - S.powi(2));
    let (p, m) = (absβm - absβp * C, absβp - absβm * C);
    let ζp2 = (A + absβp.powi(2) * A - 2. * absβp.powi(2) * S.powi(2) - 2. * absβp * p * root)
        / (absαp.powi(2) * A);
    let ζm2 = (A + absβm.powi(2) * A - 2. * absβm.powi(2) * S.powi(2) + 2. * absβm * m * root)
        / (absαm.powi(2) * A);
    let reapbm = (αp * βm.conj() * φ[0] * φ[1].conj()).re;
    let imapbm = (αp * βm.conj() * φ[0] * φ[1].conj()).im;
    let X = ((absβm * absβp.powi(2) * (A - S.powi(2)))
        * (absαp.powi(2) * absβm * A
            - 2. * (-p + absβp * root)
                * ((reapbm - absβm * absβp * norm2(φ[0])) * root - S * imapbm)))
        / (absαp.powi(2) * A.powi(4));
    let our_result: Vec<Real> = (-n..=n)
        .map(|x| match x {
            x if x >= 0 => (4. * X * (-p) * (-p + absβp * root) * ζp2.powi(x)) / (absαp.powi(2)),
            x if x < 0 => (4. * X * m * (m + absβm * root) * ζm2.powi(x)) / (absαm.powi(2)),
            _ => 0.,
        })
        .collect();
    our_result
}
