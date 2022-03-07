use crate::foundation::number::*;
use crate::foundation::*;
use crate::line::walk::walk;

pub fn periodic_walk(
    steps: usize,
    initial: QuantumState2,
    coin_m1: Coin2,
    coin_m2: Coin2,
    coin_o: Coin2,
    coin_p1: Coin2,
    coin_p2: Coin2,
) -> Vec<Vec<Real>> {
    let operator = |x| match x {
        x if x < steps as i32 && steps % 2 == 0 => coin_m1,
        x if x < steps as i32 && steps % 2 == 1 => coin_m2,
        x if x > steps as i32 && steps % 2 == 1 => coin_p1,
        x if x > steps as i32 && steps % 2 == 0 => coin_p2,
        _ => coin_o,
    };
    walk(steps, &operator, initial)
}

#[test]
fn unit_test() {
    use crate::foundation::QuantumState2;
    use crate::foundation::{seed::seeder::Seed, Coin2};
    let probs = periodic_walk(
        100,
        QuantumState2::seed(),
        Coin2::seed(),
        Coin2::seed(),
        Coin2::seed(),
        Coin2::seed(),
        Coin2::seed(),
    );
    let sum: f64 = probs[probs.len() - 1].iter().sum();
    assert!((sum - 1.).abs() < 0.0001);
}
