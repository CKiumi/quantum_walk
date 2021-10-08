// #[cfg(test)]
// pub mod seeder {
//     use crate::foundation::number::*;
//     pub fn complex_pair() -> (Complex, Complex) {
//         use rand::Rng;
//         let mut rng = rand::thread_rng();
//         let abs1: f64 = rng.gen_range(0.0..1.);
//         let abs2 = sqrt(1. - abs1.powi(2));
//         let (arg1, arg2) = (rng.gen_range(0.0..3.142), rng.gen_range(0.0..3.142));
//         (abs1 * exp(I * arg1), abs2 * exp(I * arg2))
//     }

//     pub fn real() -> Real {
//         use rand::Rng;
//         let mut rng = rand::thread_rng();
//         rng.gen_range(0.0..3.14)
//     }
//     pub fn real_zero_one() -> Real {
//         use rand::Rng;
//         let mut rng = rand::thread_rng();
//         rng.gen_range(0.0..3.14)
//     }
// }
