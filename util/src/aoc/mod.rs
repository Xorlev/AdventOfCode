use std::fmt::{Debug, Display};
use std::time::Instant;
use std::ops;
use std::str::FromStr;
use std::ops::{Add, Sub};

pub mod astar;
pub mod digits;
pub mod input;
pub mod linked_list;
pub mod top_k;

#[derive(Copy, Clone, Debug, Hash, Eq, PartialEq)]
pub struct Point {
    pub x: i32,
    pub y: i32,
}

impl Point {
    pub const fn new(x: i32, y: i32) -> Point {
        Point { x, y }
    }

    pub const fn zero() -> Point {
        Point::new(0, 0)
    }

    pub fn neighbors4(&self) -> Vec<Point> {
        vec![
            Point::new(self.x + 1, self.y),
            Point::new(self.x - 1, self.y),
            Point::new(self.x, self.y + 1),
            Point::new(self.x, self.y - 1),
        ]
    }

    pub fn neighbors8(&self) -> Vec<Point> {
        vec![
            Point::new(self.x + 1, self.y),
            Point::new(self.x - 1, self.y),
            Point::new(self.x, self.y + 1),
            Point::new(self.x, self.y - 1),
            Point::new(self.x + 1, self.y - 1),
            Point::new(self.x - 1, self.y + 1),
            Point::new(self.x + 1, self.y + 1),
            Point::new(self.x - 1, self.y - 1),
        ]
    }

    /// Creates a square of points, using self as the top-left point.
    pub fn square(&self, side_length: usize) -> Vec<Point> {
        let mut points = Vec::new();

        for x in 0..side_length {
            for y in 0..side_length {
                points.push(Point::new(x as i32, y as i32) + *self);
            }

        }

        points
    }

    #[inline]
    pub fn manhattan_distance(&self, other: &Point) -> i32 {
        (self.x - other.x).abs() + (self.y - other.y).abs()
    }
}

impl Add for Point {
    type Output = Point;

    fn add(self, rhs: Self) -> Self::Output {
        Point {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl Sub for Point {
    type Output = Point;

    fn sub(self, rhs: Self) -> Self::Output {
        Point {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
        }
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

pub trait ParseAs {
    fn parse<F: FromStr>(&self) -> Result<Vec<F>, F::Err>;
}

impl<T: AsRef<str>> ParseAs for Vec<T> {
    fn parse<F: FromStr>(&self) -> Result<Vec<F>, <F as FromStr>::Err> {
        self.iter().map(|val| FromStr::from_str(val.as_ref())).collect()
    }
}