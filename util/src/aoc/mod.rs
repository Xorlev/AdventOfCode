use std::time::Instant;

pub mod astar;
pub mod input;

pub fn time<F, T>(function: F) -> T
    where F: Fn() -> T {
    let start_time = Instant::now();
    let result: T = function();
    let end_time = Instant::now();
    println!("=> {:?}", end_time - start_time);
    result
}