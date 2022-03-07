#[cfg(test)]
pub mod seeder {
    use crate::foundation::number::*;
    use crate::foundation::QuantumState2;
    use crate::foundation::{Coin, Coin2};
    use rand;
    pub trait Seed {
        fn seed() -> Self;
    }

    impl Seed for QuantumState2 {
        fn seed() -> Self {
            let a: f64 = rand::random();
            QuantumState2::new(
                a * exp(I * rand::random::<f64>()),
                sqrt(1. - a.powi(2)) * exp(I * rand::random::<f64>()),
            )
        }
    }

    impl Seed for Coin2 {
        fn seed() -> Self {
            Coin2::from_param(
                rand::random(),
                rand::random(),
                rand::random(),
                rand::random(),
            )
        }
    }

    impl Seed for Complex {
        fn seed() -> Self {
            Complex::new(rand::random(), rand::random())
        }
    }
}
