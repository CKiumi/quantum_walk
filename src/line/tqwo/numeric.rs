use super::transfer_matrix;
use crate::foundation::number::*;
use crate::foundation::*;
use nalgebra::Matrix2;

fn n_constant(T_o: Matrix2<Complex>, phi: QuantumState2, zeta_p: Complex, zeta_m: Complex) -> f64 {
    let phi_1 = T_o * phi;
    (phi_1[0].norm_sqr() + phi_1[1].norm_sqr()) * (1. / (1. - zeta_p.norm_sqr()))
        + (phi[0].norm_sqr() + phi[1].norm_sqr()) * (zeta_m.norm_sqr() / (zeta_m.norm_sqr() - 1.))
}

pub fn eigenvector(
    n: i32,
    αm: Complex,
    βm: Complex,
    Δm: Real,
    αo: Complex,
    βo: Complex,
    Δo: Real,
    αp: Complex,
    Δp: Real,
    ev: &Complex,
) -> Vec<QuantumState2> {
    let cos_p = (ev * exp(-I * Δp)).re;
    let cos_m = (ev * exp(-I * Δm)).re;
    let sp = if cos_p > 0. { -1 } else { 1 };
    let sm = if cos_m > 0. { 1 } else { -1 };
    let T_o = transfer_matrix::new(αo, βo, Δo, *ev);
    let ζp = transfer_matrix::ev(αp, Δp, *ev, sp);
    let ζm = transfer_matrix::ev(αm, Δm, *ev, sm);
    let φ = transfer_matrix::evec(αm, βm, Δm, *ev, sm);
    let N = n_constant(T_o, φ, ζp, ζm);
    (-n..=n)
        .map(|x| match x {
            x if x < 0 => QuantumState2::new(ζm.powi(x + 1) * φ[0], ζm.powi(x) * φ[1]),
            x if x > 0 => {
                QuantumState2::new(ζp.powi(x) * (T_o * φ)[0], ζp.powi(x - 1) * (T_o * φ)[1])
            }
            _ => QuantumState2::new((T_o * φ)[0], φ[1]),
        })
        .map(|vec| vec.map(|e| e / sqrt(N)))
        .collect()
}

pub fn time_averaged_limit(
    n: i32,
    init_state: QuantumState2,
    eigenvectors: &[Vec<QuantumState2>],
) -> Vec<Real> {
    let limits: Vec<Vec<Real>> = eigenvectors
        .iter()
        .map(|eigenvector| {
            let origin_norm = norm2(eigenvector[n as usize].dot(&init_state));
            let time_limit: Vec<Real> = eigenvector
                .iter()
                .map(|vec| (origin_norm * vec.norm_squared()))
                .collect();
            time_limit
        })
        .collect();
    let mut total_sum = vec![0.; (2 * n + 1) as usize];

    for limit in limits {
        for x in 0..(2 * n + 1) as usize {
            total_sum[x] += limit[x];
        }
    }
    total_sum
}
