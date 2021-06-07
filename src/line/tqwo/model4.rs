use crate::foundation::{number::*, QuantumState2};

pub fn eigenvalues(Δ: Real, βp: Complex, βm: Complex) -> Vec<Complex> {
    let (absαp, absαm) = (sqrt(1. - norm2(βp)), sqrt(1. - norm2(βm)));
    let Complex { re: rbmp, im: ibmp } = βm * βp.conj();
    if (rbmp - norm2(βp)) * (rbmp - norm2(βm)) > 0. {
        let x = sqrt((rbmp + absαm * absαp - 1.) * (rbmp - absαm * absαp - 1.)) + I * ibmp;
        let ev = (x * exp(I * Δ)) / norm(βp - βm);
        vec![ev, -ev]
    } else {
        vec![zero, zero]
    }
}

pub fn time_averaged(
    n: i32,
    Δ: Real,
    αp: Complex,
    αm: Complex,
    βm: Complex,
    βp: Complex,
    φ: QuantumState2,
) -> Vec<Real> {
    let evs = eigenvalues(Δ, βp, βm);
    if evs[0] == zero {
        return vec![0.; (2 * n + 1) as usize];
    }
    let (rebmbp, imbmbp) = ((βm * βp.conj()).re, (βm * βp.conj()).im);
    let A = norm2(βm) - 2. * rebmbp + norm2(βp);
    let root = (A - imbmbp.powi(2)).sqrt();
    let (p, m) = (rebmbp - norm2(βp), rebmbp - norm2(βm));
    let reamp1p2 = (αp * βm.conj() * φ[0] * φ[1].conj()).re;
    let imamp1p2 = (αp * βm.conj() * φ[0] * φ[1].conj()).im;
    let ζp2 =
        (-2. * imbmbp.powi(2) + (1. + βp.norm_sqr()) * A + 2. * p * root) / (αp.norm_sqr() * A);
    let ζm2 =
        (-2. * imbmbp.powi(2) + (1. + βm.norm_sqr()) * A - 2. * m * root) / (αm.norm_sqr() * A);
    let X = ((p.powi(2) * m.powi(2))
        * (norm2(αp) * norm2(βm) * A
            + 2. * (norm2(βm) * p * norm2(φ[0]) + m * reamp1p2 - imbmbp * imamp1p2) * (p + root)))
        / (norm2(αp) * norm2(βm) * root * A.powi(4));
    let our_result: Vec<Real> = (-n..=n)
        .map(|x| match x {
            x if x >= 0 => (4. * X * (p + root) * ζp2.powi(x)) / norm2(αp),
            _ => (4. * X * (-m + root) * ζm2.powi(x)) / norm2(αm),
        })
        .collect();
    // println!("{:?}", our_result);
    our_result
}
