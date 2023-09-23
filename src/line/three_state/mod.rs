pub mod eigenvalue;
pub mod walk;

#[cfg(test)]
mod three_test {
    use crate::foundation::create_fourier_coin;
    use crate::foundation::State3;
    use crate::plot;
    use std::f64::consts::PI;
    #[test]
    fn test_fourier_two_phase() {
        for i in 0..12 {
            let coinm = create_fourier_coin(0.);
            let coinp = create_fourier_coin(PI * i as f64 / 12.);
            let init = crate::foundation::QuantumState3::from_arr([
                [1. / (3_f64).sqrt(), 0.],
                [0., 1. / (3_f64).sqrt()],
                [1. / (3_f64).sqrt(), 0.],
            ]);
            let probs = super::walk::run(200, init, coinm, coinp.clone(), coinp);
            let filename = format!("image/three{}.png", i);
            plot::plotter::plot_1d(&probs[probs.len() - 1], &filename).unwrap();
        }
    }
    #[test]
    fn test_fourier_one_defect() {
        for i in 0..12 {
            let coin = create_fourier_coin(0.);
            let coin0 = create_fourier_coin(PI * i as f64 / 12.);
            let init = crate::foundation::QuantumState3::from_arr([
                [1. / (3_f64).sqrt(), 0.],
                [0., 1. / (3_f64).sqrt()],
                [1. / (3_f64).sqrt(), 0.],
            ]);
            let probs = super::walk::run(200, init, coin.clone(), coin0, coin);
            let filename = format!("image/three{}.png", i);
            plot::plotter::plot_1d(&probs[probs.len() - 1], &filename).unwrap();
        }
    }
}
