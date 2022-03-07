#![allow(clippy::many_single_char_names)]
#![allow(clippy::too_many_arguments)]
#![allow(confusable_idents)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
pub mod foundation;
pub mod line;
pub use line::tqwo::model1;

pub mod plot;

#[test]
fn plot_walks() {
    use foundation::{Coin, QuantumState2, State};
    use line::walk::homogeneous;
    let coin = Coin::from_param(0.2, 0.74, 0.213, 0.123);
    let probs = homogeneous(100, QuantumState2::from_arr([[0.5, 0.], [0., 0.5]]), coin);
    plot::plotter::plot_1d(&probs[probs.len() - 1], "image/test_homo.png").unwrap();
}
