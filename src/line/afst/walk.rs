use crate::{
    foundation::{number::*, *},
    line::walk::{create_line, norm_qs},
};

#[derive(Debug, Clone)]
pub struct SSQW {
    pub a: Real,
    pub argb: Real,
    pub p: Real,
    pub argq: Real,
    pub γ: Real,
}
fn afst_coin(param: SSQW) -> Coin2 {
    let b = sqrt(1. - param.a * param.a) * exp(I * param.argb);
    let a = param.a * exp(I * 0.);
    let γ = Complex::new(param.γ, 0.);
    Coin2::new(exp(-2. * γ) * a, b.conj(), b, exp(2. * γ) * -a)
}
pub fn afst(n: usize, initial: QuantumState2, param_m: SSQW, param_p: SSQW) -> Vec<Vec<Real>> {
    let mut space = create_line(n);
    let coin_m = afst_coin(param_m.clone());
    let coin_p = afst_coin(param_p.clone());
    space[0][n] = initial;
    let qm = sqrt(1. - param_m.p * param_m.p) * exp(I * param_m.argq);
    let qp = sqrt(1. - param_p.p * param_p.p) * exp(I * param_p.argq);
    for i in 0..n {
        for j in 1..space[i].len() {
            if j < n {
                let coined = space[i][j].operate_coin(&coin_m);
                space[i + 1][j - 1][0] += qm * coined[1];
                space[i + 1][j][0] += param_m.p * coined[0];
                space[i + 1][j][1] += -param_m.p * coined[1];
                space[i + 1][j + 1][1] += qm.conj() * coined[0];
            }
            if j == n {
                let coined = space[i][j].operate_coin(&coin_p);
                space[i + 1][j - 1][0] += qm * coined[1];
                space[i + 1][j][0] += param_p.p * coined[0];
                space[i + 1][j][1] += -param_m.p * coined[1];
                space[i + 1][j + 1][1] += qp.conj() * coined[0];
            }
            if n < j && j < 2 * n {
                let coined = space[i][j].operate_coin(&coin_p);
                space[i + 1][j - 1][0] += qp * coined[1];
                space[i + 1][j][0] += param_p.p * coined[0];
                space[i + 1][j][1] += -param_p.p * coined[1];
                space[i + 1][j + 1][1] += qp.conj() * coined[0];
            }
        }
    }
    space.iter().map(|line| norm_qs(&line)).collect()
}
