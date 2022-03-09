use super::zero;
use super::{QuantumState2, Real};
use crate::foundation::{norm_qs, Coin2};
use std::vec::Vec;

pub fn homogeneous(steps: usize, initial: QuantumState2, coin: Coin2) -> Vec<Vec<Real>> {
    walk(steps, &|_| coin, initial)
}

pub fn walk(
    steps: usize,
    operator: &dyn Fn(i32) -> Coin2,
    initial: QuantumState2,
) -> Vec<Vec<Real>> {
    let mut space = create_line(steps);
    let len = space[0].len();
    space[0][steps] = initial;
    for i in 0..steps {
        for j in 0..len {
            if space[i + 1][j] != space[i][j] {
                space[i][j] = operator(j as i32 - steps as i32) * space[i][j];
            };
        }
        for j in 0..len {
            if j != len - 1 {
                space[i + 1][j][0] = space[i][j + 1][0];
            }
            if j != 0 {
                space[i + 1][j][1] = space[i][j - 1][1];
            }
        }
    }
    space.iter().map(|line| norm_qs(&line)).collect()
}

pub fn create_line(n: usize) -> Vec<Vec<QuantumState2>> {
    vec![vec![QuantumState2::new(zero, zero); 2 * n + 1]; n + 1]
}

pub fn get_averaged_limit(probs: Vec<Vec<Real>>) -> Vec<Real> {
    let n = probs.len() - 1;
    dbg!(n);
    let mut time_averaged = vec![0.; (2 * n + 1) as usize];
    for prob in probs {
        for x in 0..(2 * n + 1) as usize {
            time_averaged[x] += prob[x] / n as f64;
        }
    }
    time_averaged
}

#[test]
fn unit_test() {
    use super::QuantumState2;
    use crate::foundation::{Coin2, Seed};
    let probs = walk(4, &|_| Coin2::seed(), QuantumState2::seed());
    let sum: f64 = probs[probs.len() - 1].iter().sum();
    assert!((sum - 1.).abs() < 0.0001);
}
