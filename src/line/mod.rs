pub mod walk;
use super::foundation;
mod plotter;
use foundation::number::*;
use foundation::QuantumState2;
pub mod dqw;
pub mod tqwo;

// #[test]
// pub fn test() {
//     use foundation::CoinOperation;
//     use plotter::plot;

//     let coin_p = Coin2::from_param(0.5, 0.82, 0.5, 0.5);
//     let coin_o = Coin2::from_param(0.3, 0.9, 0.5, 0.5);
//     let coin_m = Coin2::from_param(0.5, 0.82, 0.5, 0.5);
//     let initial = QuantumState2::new(
//         Complex::new(1. / (2_f64).sqrt(), 0.),
//         Complex::new(0., 1. / (2_f64).sqrt()),
//     );
//     let n = 200;
//     plot(&tqwo::walk::run(n, initial, coin_m, coin_o, coin_p)[n]);
// }
