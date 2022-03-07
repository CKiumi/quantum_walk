use crate::foundation::number::*;
use crate::foundation::*;
use crate::line::walk::walk;

pub fn run(
    steps: usize,
    initial: QuantumState2,
    coin_m: Coin2,
    coin_o: Coin2,
    coin_p: Coin2,
) -> Vec<Vec<Real>> {
    let operator = |x| match x {
        x if x < steps as i32 => coin_m,
        x if x > steps as i32 => coin_p,
        _ => coin_o,
    };
    walk(steps, &operator, initial)
}

#[test]
fn unit_test() {
    use crate::foundation::QuantumState2;
    use crate::foundation::{Coin2, Seed};
    let probs = run(
        100,
        QuantumState2::seed(),
        Coin2::seed(),
        Coin2::seed(),
        Coin2::seed(),
    );
    let sum: f64 = probs[probs.len() - 1].iter().sum();
    assert!((sum - 1.).abs() < 0.0001);
}
