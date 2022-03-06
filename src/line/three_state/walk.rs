use crate::foundation::number::*;
use crate::foundation::*;
use nalgebra::Vector3;
// use std::time::Instant;
fn norm_qs3(states: &[QuantumState3]) -> Vec<Real> {
    states
        .iter()
        .map(|x| x[0].norm_sqr() + x[1].norm_sqr() + x[2].norm_sqr())
        .collect()
}
pub fn run(
    n: usize,
    initial: QuantumState3,
    coin_m: Coin3,
    coin_o: Coin3,
    coin_p: Coin3,
) -> Vec<Vec<Real>> {
    let mut space = create_line(n);
    space[0][n] = initial;
    for i in 1..n + 1 {
        shift(
            i,
            &operate_coin(&coin_m, &coin_o, &coin_p, &space[i - 1]),
            &mut space[i],
        );
    }
    space.iter().map(|line| norm_qs3(&line)).collect()
}

fn operate_coin(
    coin_m: &Coin3,
    coin_o: &Coin3,
    coin_p: &Coin3,
    states: &[QuantumState3],
) -> Vec<QuantumState3> {
    let n = (states.len() - 1) / 2;
    states
        .iter()
        .enumerate()
        .map(|(index, state)| {
            if state[0] == zero && state[1] == zero && state[2] == zero {
                *state
            } else {
                match index {
                    index if index < n => coin_m * state,
                    index if index > n => coin_p * state,
                    _ => coin_o * state,
                }
            }
        })
        .collect()
}

pub fn create_line(n: usize) -> Vec<Vec<QuantumState3>> {
    vec![vec![Vector3::new(zero, zero, zero); 2 * n + 1]; n + 1]
}

pub fn shift(step: usize, states: &[QuantumState3], original: &mut Vec<QuantumState3>) {
    let len = states.len() as usize;
    let n = (states.len() - 1) / 2;
    let remainder = n - step;
    for x in remainder..len - remainder {
        if x != len - 1 {
            original[x][0] = states[x + 1][0];
        }
        original[x][1] = states[x][1];
        if x != 0 {
            original[x][2] = states[x - 1][2];
        }
    }
}
