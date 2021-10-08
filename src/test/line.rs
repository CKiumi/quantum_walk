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
        // let result_eigenvector =
        // line::tqwo::model2::eigenvector_con1(n, Δo, Δ, α, β, arg(αo), &evs[0]);
        let eigenvectors: Vec<Vec<QuantumState2>> = evs
            .into_iter()
            .map(|ev| line::tqwo::numeric::eigenvector(n, α, β, Δ, αo, β, Δo, α, Δ, &ev))
            .collect();

        println!(
            "{:?} \n {:?}",
            line::tqwo::model2::time_averaged(n, Δo, Δ, α, β, arg(αo), initial),
            line::tqwo::numeric::time_averaged_limit(n, initial, &eigenvectors)
        );

        let his = line::tqwo::walk::run(n2 as usize, initial, coin, coin_o, coin);
        print_local(n2, time_averaged_limit_distribution(his));
    }

    #[test]
    fn model3() {
        let (x, y) = seeder::complex_pair();
        let initial = QuantumState2::new(x, y);
        let (αp, βp) = seeder::complex_pair();
        let βm = seeder::real_zero_one() * exp(I * arg(βp));
        let αm = sqrt(1. - norm2(βm)) * exp(I * seeder::real());
        let (Δp, Δm) = (seeder::real(), seeder::real());
        let coin_m = Coin2::from_param(Δm, norm(αm), arg(αm), arg(βm));
        let coin_p = Coin2::from_param(Δp, norm(αp), arg(αp), arg(βp));
        let (n, n2) = (3, 2000);
        let evs = line::tqwo::model3::eigenvalues(Δp, Δm, norm(βp), norm(βm));
        let eigenvectors: Vec<Vec<QuantumState2>> = evs
            .into_iter()
            .map(|ev| line::tqwo::numeric::eigenvector(n, αm, βm, Δm, αp, βp, Δp, αp, Δp, &ev))
            .collect();

        println!(
            "{:?} \n {:?}",
            line::tqwo::model3::time_averaged(
                n,
                Δp,
                Δm,
                norm(βm),
                norm(βp),
                arg(βp),
                arg(αm),
                arg(αp),
                initial
            ),
            line::tqwo::numeric::time_averaged_limit(n, initial, &eigenvectors)
        );

        let his = line::tqwo::walk::run(n2 as usize, initial, coin_m, coin_p, coin_p);
        print_local(n2, time_averaged_limit_distribution(his));
    }

    #[test]
    fn model4() {
        let (x, y) = seeder::complex_pair();
        let initial = QuantumState2::new(x, y);
        let (αp, βp) = seeder::complex_pair();
        let (αm, βm) = seeder::complex_pair();
        let Δ = seeder::real();
        let coin_m = Coin2::from_param(Δ, norm(αm), arg(αm), arg(βm));
        let coin_p = Coin2::from_param(Δ, norm(αp), arg(αp), arg(βp));
        let (n, n2) = (3, 2000);
        let evs = line::tqwo::model4::eigenvalues(Δ, βp, βm);
        let eigenvectors: Vec<Vec<QuantumState2>> = evs
            .into_iter()
            .map(|ev| line::tqwo::numeric::eigenvector(n, αm, βm, Δ, αp, βp, Δ, αp, Δ, &ev))
            .collect();

        println!(
            "{:?} \n {:?}",
            line::tqwo::model4::time_averaged(n, Δ, αp, αm, βp, βm, initial),
            line::tqwo::numeric::time_averaged_limit(n, initial, &eigenvectors)
        );

        let his = line::tqwo::walk::run(n2 as usize, initial, coin_m, coin_p, coin_p);
        print_local(n2, time_averaged_limit_distribution(his));
    }
    #[test]
    fn model5() {
        let (x, y) = seeder::complex_pair();
        let initial = QuantumState2::new(x, y);
        let βo = zero;
        let αo = exp(I * seeder::real());
        let (αp, βp) = seeder::complex_pair();
        let (αm, βm) = (
            norm(αp) * exp(I * seeder::real()),
            norm(βp) * exp(I * seeder::real()),
        );

        let (Δ, Δo) = (seeder::real(), seeder::real());
        let γ = Δo + (arg(βp) - arg(βm)) / 2.;
        let coin_m = Coin2::from_param(Δ, norm(αm), arg(αm), arg(βm));
        let coin_o = Coin2::from_param(Δo, norm(αo), arg(αo), arg(βo));
        let coin_p = Coin2::from_param(Δ, norm(αp), arg(αp), arg(βp));
        let (n, n2) = (3, 200);
        let evs = line::tqwo::model5::eigenvalues(Δ, norm(βp), γ);
        let eigenvectors: Vec<Vec<QuantumState2>> = evs
            .into_iter()
            .map(|ev| line::tqwo::numeric::eigenvector(n, αm, βm, Δ, αo, βo, Δo, αp, Δ, &ev))
            .collect();

        println!(
            "{:?} \n {:?}",
            line::tqwo::model5::time_averaged(
                n,
                Δ,
                Δo,
                norm(βp),
                arg(αo),
                arg(βm),
                arg(βp),
                initial,
            ),
            line::tqwo::numeric::time_averaged_limit(n, initial, &eigenvectors)
        );

        let his = line::tqwo::walk::run(n2 as usize, initial, coin_m, coin_o, coin_p);
        print_local(n2, time_averaged_limit_distribution(his));
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
