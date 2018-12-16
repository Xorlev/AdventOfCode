use std::fmt::{Debug, Display};
use std::time::Instant;
use std::ops;

pub mod astar;
pub mod input;
pub mod linked_list;
pub mod top_k;

#[derive(Copy, Clone, Debug, Hash, Eq, PartialEq)]
pub struct Point {
    pub x: i32,
    pub y: i32,
}

impl Point {
    pub fn new(x: i32, y: i32) -> Point {
        Point { x, y }
    }

    pub fn neighbors4(&self) -> Vec<Point> {
        vec![
            Point::new(self.x + 1, self.y),
            Point::new(self.x - 1, self.y),
            Point::new(self.x, self.y + 1),
            Point::new(self.x, self.y - 1),
        ]
    }

    pub fn add(&self, rhs: &Point) -> Point {
        Point {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }

    #[inline]
    pub fn manhattan_distance(&self, other: &Point) -> i32 {
        (self.x - other.x).abs() + (self.y - other.y).abs()
    }
}

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