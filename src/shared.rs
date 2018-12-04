use std::time::Instant;
use std::fmt::Display;

pub fn output_and_time<F>(label: &str, f: F) where F: Fn() -> Box<Display> {
    let now = Instant::now();
    let result = f();
    let time_spent = now.elapsed();
    println!("{}: {} (took {:?})", label, result, time_spent);
}
