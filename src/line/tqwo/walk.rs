use crate::foundation::number::*;
use crate::foundation::*;
use crate::line::walk::create_line;
use crate::line::walk::norm_qs;
use crate::line::walk::operate_shift;
use std::time::Instant;

pub fn run(
    n: usize,
    initial: QuantumState2,
    coin_m: Coin2,
    coin_o: Coin2,
    coin_p: Coin2,
) -> Vec<Vec<Real>> {
    let mut space = create_line(n);
    space[0][n] = initial;
    let start = Instant::now();
    for i in 1..n + 1 {
        operate_shift(
            i,
            &operate_coin_tqwo(&coin_m, &coin_o, &coin_p, space[i - 1].clone()),
            &mut space[i],
        );
    }
    print_duration(start.elapsed());
    space.iter().map(|line| norm_qs(&line)).collect()
}

fn operate_coin_tqwo(
    coin_m: &Coin2,
    coin_o: &Coin2,
    coin_p: &Coin2,
    states: Vec<QuantumState2>,
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
                    index if index < n => state.operate_coin(&coin_m),
                    index if index > n => state.operate_coin(&coin_p),
                    _ => state.operate_coin(&coin_o),
                }
            }
        })
        .collect()
}
