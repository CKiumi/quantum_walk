use std::time::Duration;
pub mod coin;
pub mod number;
pub mod quantumstate;
pub use coin::*;
pub use quantumstate::*;

pub fn print_duration(end: Duration) {
    println!(
        "shift{}.{:03}秒経過しました。",
        end.as_secs(),
        end.subsec_nanos() / 1_000_000
    );
}

pub fn print_prob(vector: &Vec<f64>) {
    for i in vector {
        print!("{:.1}   ", i);
    }
}
