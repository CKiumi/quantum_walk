use crate::foundation::number::*;
use crate::foundation::*;
use crate::line::walk::create_line;
use crate::line::walk::norm_qs;
use crate::line::walk::operate_shift;

#[test]
fn test() {}

pub fn homogeneous_run(n: usize, initial: QuantumState2, angle: f64) -> Vec<Vec<Real>> {
    let mut space = create_line(n);
    space[0][n] = initial;
    for i in 1..n + 1 {
        operate_shift(
            i,
            &operate_homo_coin(angle, &mut space[i - 1]),
            &mut space[i],
        );
        // dbg!(space[i][n].norm2());
    }
    space.iter().map(|line| norm_qs(&line)).collect()
}

fn operate_homo_coin(angle: f64, states: &Vec<QuantumState2>) -> Vec<QuantumState2> {
    let n = (states.len() - 1) / 2;
    states
        .iter()
        .enumerate()
        .map(|(index, state)| {
            if state[0] == zero && state[1] == zero {
                *state
            } else {
                let theta = (index as f64 - n as f64) * angle;
                let coin = Coin2::new(
                    Complex::new(cos(theta), 0.),
                    Complex::new(sin(theta), 0.),
                    Complex::new(sin(theta), 0.),
                    Complex::new(-cos(theta), 0.),
                );
                state.operate_coin(&coin)
            }
        })
        .collect()
}

pub fn run(
    n: usize,
    initial: QuantumState2,
    coin_m1: Coin2,
    coin_m2: Coin2,
    coin_o: Coin2,
    coin_p1: Coin2,
    coin_p2: Coin2,
) -> Vec<Vec<Real>> {
    let mut space = create_line(n);
    space[0][n] = initial;
    for i in 1..n + 1 {
        operate_shift(
            i,
            &operate_coin(
                &coin_m1,
                &coin_m2,
                &coin_o,
                &coin_p1,
                &coin_p2,
                &space[i - 1],
            ),
            &mut space[i],
        );
    }
    space.iter().map(|line| norm_qs(&line)).collect()
}

fn operate_coin(
    coin_m1: &Coin2,
    coin_m2: &Coin2,
    coin_o: &Coin2,
    coin_p1: &Coin2,
    coin_p2: &Coin2,
    states: &Vec<QuantumState2>,
) -> Vec<QuantumState2> {
    let n = (states.len() - 1) / 2;
    states
        .iter()
        .enumerate()
        .map(|(index, state)| {
            if state[0] == zero && state[1] == zero {
                *state
            } else {
                match index {
                    index if index < n && n % 2 == 0 => state.operate_coin(&coin_m1),
                    index if index < n && n % 2 == 1 => state.operate_coin(&coin_m2),
                    index if index > n && n % 2 == 1 => state.operate_coin(&coin_p1),
                    index if index > n && n % 2 == 0 => state.operate_coin(&coin_p2),
                    _ => state.operate_coin(&coin_o),
                }
            }
        })
        .collect()
}
