use super::zero;
use super::{QuantumState2, Real};
use nalgebra::Vector2;

use std::vec::Vec;

pub fn rw(n: usize) -> Vec<Vec<Real>> {
    let mut space = vec![vec![0.; 2 * n + 1]; n + 1];
    space[0][n] = 1.;
    for i in 1..(n as usize) + 1 {
        for x in 0..2 * n + 1 {
            if x != 0 {
                space[i][x - 1] += space[i - 1][x] / 2.;
            }
            if x != 2 * n {
                space[i][x + 1] += space[i - 1][x] / 2.;
            }
        }
    }
    space
}

pub fn continuous_rw(n: usize) -> Vec<Real> {
    let discrete = &rw(n)[n];
    let mut continuous: Vec<Real> = vec![0.; 2 * n + 1];
    for i in 1..discrete.len() - 1 {
        continuous[i] = if discrete[i] == 0. {
            (discrete[i - 1] + discrete[i + 1]) / 2.
        } else {
            discrete[i]
        }
    }
    continuous
}

pub fn create_line(n: usize) -> Vec<Vec<QuantumState2>> {
    vec![vec![Vector2::new(zero, zero); 2 * n + 1]; n + 1]
}

pub fn operate_shift(step: usize, states: &Vec<QuantumState2>, original: &mut Vec<QuantumState2>) {
    let len = states.len() as usize;
    let n = (states.len() - 1) / 2;
    let reminder = n - step;
    for x in 0 + reminder..len - reminder {
        if x != len - 1 {
            original[x][0] = states[x + 1][0];
        }
        if x != 0 {
            original[x][1] = states[x - 1][1];
        }
    }
}

pub fn norm_qs(states: &Vec<QuantumState2>) -> Vec<Real> {
    states
        .iter()
        .map(|x| x[0].norm_sqr() + x[1].norm_sqr())
        .collect()
}
