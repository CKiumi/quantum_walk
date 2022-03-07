#![allow(clippy::many_single_char_names)]
#![allow(clippy::too_many_arguments)]
#![allow(confusable_idents)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
pub mod foundation;
pub mod line;
pub use line::tqwo::model1;

use crate::line::tqwo;

pub mod plot;

#[test]
fn plot_walks() {
    use foundation::{Coin, QuantumState2, Seed};
    use line::walk::homogeneous;
    let coin = Coin::from_param(0.2, 0.74, 0.213, 0.123);
    let coin_m = Coin::from_param(0.2, 0.5, 0.211, 0.124);
    let coin_o = Coin::from_param(0.2, 0.84, 0.113, 0.123);
    let coin_p = Coin::from_param(0.2, 0.5, 0.211, 0.124);
    let probs = homogeneous(100, QuantumState2::seed(), coin);
    plot::plotter::plot_1d(&probs[probs.len() - 1], "image/test_homo.png").unwrap();
    let probs = tqwo::walk::run(100, QuantumState2::seed(), coin_m, coin_o, coin_p);
    plot::plotter::plot_1d(&probs[probs.len() - 1], "image/test_tqwo.png").unwrap();
}
