use std::fmt::{Debug, Display};
use std::time::Instant;

pub mod astar;
pub mod input;

pub fn time<S, F, T>(label: S, function: F) -> T
    where F: Fn() -> T,
          S: ToString + Display,
          T: Debug {
    let start_time = Instant::now();
    let result: T = function();
    let end_time = Instant::now();
    println!("{}: {:?}", label, end_time - start_time);
    result
}

pub fn result<S, F, T>(label: S, function: F) -> T
    where F: Fn() -> T,
          S: ToString + Display,
          T: Debug {
    let result = time(label, function);
    println!(" => {:?}", result);
    result
}