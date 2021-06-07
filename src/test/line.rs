#[cfg(test)]
mod tqwo {
    use super::super::tools::seeder;
    use crate::foundation::coin::*;
    use crate::foundation::number::*;
    use crate::foundation::quantumstate::*;
    use crate::line;
    #[test]
    fn model1() {
        let (x, y) = seeder::complex_pair();
        let initial = QuantumState2::new(x, y);
        let (α, β) = seeder::complex_pair();
        let (αo, βo) = seeder::complex_pair();
        let Δ = seeder::real();
        let coin = Coin2::from_param(Δ, norm(α), arg(α), arg(β));
        let coin_o = Coin2::from_param(Δ, norm(αo), arg(αo), arg(βo));
        let (n, n2) = (3, 1000);
        let evs = line::tqwo::model1::eigenvalues(Δ, β, βo);
        let eigenvectors: Vec<Vec<QuantumState2>> = evs
            .into_iter()
            .map(|ev| line::tqwo::numeric::eigenvector(n, α, β, Δ, αo, βo, Δ, α, Δ, &ev))
            .collect();
        println!(
            "{:?} \n {:?}",
            line::tqwo::model1::time_averaged(n, Δ, α, β, αo, βo, initial),
            line::tqwo::numeric::time_averaged_limit(n, initial, &eigenvectors),
        );
        let his = line::tqwo::walk::run(n2 as usize, initial, coin, coin_o, coin);
        print_local(n2, time_averaged_limit_distribution(his));
    }
    #[test]
    fn model2() {
        let (x, y) = seeder::complex_pair();
        let initial = QuantumState2::new(x, y);
        let (α, β) = seeder::complex_pair();
        let αo = norm(α) * exp(I * seeder::real());
        let (Δ, Δo) = (seeder::real(), seeder::real());
        let coin = Coin2::from_param(Δ, norm(α), arg(α), arg(β));
        let coin_o = Coin2::from_param(Δo, norm(αo), arg(αo), arg(β));
        let (n, n2) = (3, 1000);
        let evs = line::tqwo::model2::eigenvalues(Δo, Δ, norm(β));
        let result_eigenvector =
            line::tqwo::model2::eigenvector_con1(n, Δo, Δ, α, β, arg(αo), &evs[0]);
        let eigenvectors: Vec<Vec<QuantumState2>> = evs
            .into_iter()
            .map(|ev| line::tqwo::numeric::eigenvector(n, α, β, Δ, αo, β, Δo, α, Δ, &ev))
            .collect();

        println!(
            "{:?} \n {:?}",
            result_eigenvector[n as usize + 1],
            eigenvectors[0][n as usize + 1] // line::tqwo::model2::time_averaged(n, Δo, Δ, α, β, arg(αo), initial),
                                            // line::tqwo::numeric::time_averaged_limit(n, initial, &eigenvectors)
        );
        let mut sum = 0.;
        for test in result_eigenvector {
            let tmp = norm2(test[0]) + norm2(test[1]);
            sum += tmp
        }
        println!("evs sums{}", sum);
        let mut sum = 0.;
        for test in &eigenvectors[0] {
            let tmp = norm2(test[0]) + norm2(test[1]);
            sum += tmp
        }
        println!("evs sums{}", sum);
        // let his = line::tqwo::walk::run(n2 as usize, initial, coin, coin_o, coin);
        // print_local(n2, time_averaged_limit_distribution(his));
    }

    fn print_local(n: usize, prob: Vec<Real>) {
        println!(
            "{},{},{},{},{},{},{}",
            prob[n - 3],
            prob[n - 2],
            prob[n - 1],
            prob[n],
            prob[n + 1],
            prob[n + 2],
            prob[n + 3],
        );
    }
    fn time_averaged_limit_distribution(histories: Vec<Vec<Real>>) -> Vec<Real> {
        let n = histories.len() - 1;
        let mut timeav = vec![0.; histories[0].len() as usize];
        for history in histories {
            for i in 0..history.len() as usize {
                timeav[i] += history[i];
            }
        }
        timeav.iter().map(|x| x / (n as f64)).collect()
    }
}
