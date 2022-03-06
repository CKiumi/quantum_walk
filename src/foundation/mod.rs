pub mod coin;
pub mod number;
pub mod quantumstate;
pub use coin::*;
pub use quantumstate::*;

pub fn print_prob(vector: &[f64]) {
    for i in vector {
        print!("{:.1}   ", i);
    }
}
