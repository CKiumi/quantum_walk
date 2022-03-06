pub mod walk;
use super::foundation;
mod plotter;
use foundation::number::*;
use foundation::QuantumState2;
pub mod afst;
pub mod dqw;
pub mod stripe;
pub mod three_state;
pub mod tqwo;
#[test]
pub fn test() {
    use foundation::*;
    use plotter::plot;
    let coin_m1 = Coin2::from_param(2.5, 0.59, 2.4, 1.42);
    let coin_m2 = Coin2::from_param(1.5, 0.59, 1.4, 1.42);
    let coin_o = Coin2::from_param(1.5, 0.57, 2.4, 1.72);
    let coin_p1 = Coin2::from_param(2.5, 0.27, 4.4, 2.42);
    let coin_p2 = Coin2::from_param(1.5, 0.27, 2.4, 2.72);
    let initial = QuantumState2::new(Complex::new(1., 0.), Complex::new(0., 0.));
    let n = 6000;
    plot(&stripe::walk::homogeneous_run(n, initial, pi / 3.)[n]);
    // plot(&tqwo::walk::run(n, initial, coin_m1, coin_o, coin_p1)[n]);
}
