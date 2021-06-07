use super::{coin::Coin2, number::*};
use nalgebra::Vector2;
pub type QuantumState2 = Vector2<Complex>;

pub trait QSOperation {
    fn norm2(&self) -> Real;
    fn operate_coin(&self, coin: &Coin2) -> Self;
    fn inner(&self, phi: QuantumState2) -> Complex;
}

impl QSOperation for QuantumState2 {
    fn norm2(&self) -> Real {
        self[0].norm_sqr() + &self[1].norm_sqr()
    }
    fn inner(&self, phi: QuantumState2) -> Complex {
        self[0].conj() * phi[0] + self[1].conj() * phi[1]
    }
    fn operate_coin(&self, coin: &Coin2) -> Self {
        Self::new(
            coin[(0, 0)] * self[0] + coin[(0, 1)] * self[1],
            coin[(1, 0)] * self[0] + coin[(1, 1)] * self[1],
        )
    }
}

pub fn print_line(vector: &Vec<QuantumState2>) {
    for i in vector {
        print!("{:.1},{:.1}   ", i[0], i[1]);
    }
}

pub fn print_line_space(vector: Vec<Vec<QuantumState2>>) {
    for x in vector {
        println!();
        print_line(&x);
    }
}

pub fn print_line_prob(vector: &Vec<Real>) {
    for i in vector {
        print!("{:.1} ", i);
    }
}
pub fn print_line_space_prob(vector: &Vec<Vec<Real>>) {
    for x in vector {
        println!();
        print_line_prob(&x);
    }
}
