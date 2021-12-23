use std::fmt::{Debug, Display, Formatter};
use std::ops::{Add, AddAssign, Mul, Sub};
use std::str::FromStr;
use std::time::Instant;

pub mod astar;
pub mod digits;
pub mod frequency;
pub mod grid;
pub mod input;
pub mod lines;
pub mod linked_list;
pub mod top_k;

pub type AocResult<T> = std::result::Result<T, failure::Error>;

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

    /// Rotates a point assuming an origin of (0, 0) around a circle, rounded to the nearest integer
    /// values.
    pub fn rotate(&self, degrees: i32) -> Point {
        let x = self.x as f32;
        let y = self.y as f32;
        let t = degrees as f32 * std::f32::consts::PI / 180.0;

        Point {
            x: (x * t.cos() - y * t.sin()).round() as i32,
            y: (y * t.cos() + x * t.sin()).round() as i32,
        }
    }

    #[inline]
    pub fn manhattan_distance(&self, other: &Point) -> i32 {
        (self.x - other.x).abs() + (self.y - other.y).abs()
    }

    #[inline]
    pub fn distance(&self, other: &Point) -> f32 {
        (((other.x - self.x) as f32).powf(2.0) + ((self.y - other.y) as f32).powf(2.0)).sqrt()
    }

    pub fn to_index(&self, x_max: usize) -> usize {
        (self.y * x_max as i32 + self.x) as usize
    }
}

impl Display for Point {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
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

impl AddAssign for Point {
    fn add_assign(&mut self, rhs: Self) {
        self.x += rhs.x;
        self.y += rhs.y;
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

impl Mul<i32> for Point {
    type Output = Point;

    fn mul(self, rhs: i32) -> Self::Output {
        Point {
            x: self.x * rhs,
            y: self.y * rhs,
        }
    }
}

pub struct PointIterator {
    start: Point,
    delta: Point,
}

impl PointIterator {
    pub fn new(start: Point, delta: Point) -> PointIterator {
        PointIterator { start, delta }
    }
}

impl Iterator for PointIterator {
    type Item = Point;

    fn next(&mut self) -> Option<Self::Item> {
        let value = self.start.clone();
        self.start += self.delta;
        Some(value)
    }
}

pub fn time<S, F, T>(label: S, function: F) -> T
where
    F: Fn() -> T,
    S: ToString + Display,
    T: Debug,
{
    let start_time = Instant::now();
    let result: T = function();
    let end_time = Instant::now();
    println!("{}: {:?}", label, end_time - start_time);
    result
}

pub fn result<S, F, T>(label: S, function: F) -> T
where
    F: Fn() -> T,
    S: ToString + Display,
    T: Debug,
{
    let result = time(label, function);
    println!(" => {:?}", result);
    result
}

pub trait ParseAs {
    fn parse<F: FromStr>(&self) -> Result<Vec<F>, F::Err>;
}

impl<T: AsRef<str>> ParseAs for Vec<T> {
    fn parse<F: FromStr>(&self) -> Result<Vec<F>, <F as FromStr>::Err> {
        self.iter()
            .map(|val| FromStr::from_str(val.as_ref()))
            .collect()
    }
}

pub trait SliceExt {
    type Item;

    /// Returns two mutable references from a slice, so long as they're disjoint indices.
    fn get_two_mut(&mut self, a: usize, b: usize) -> (&mut Self::Item, &mut Self::Item);
}

impl<T> SliceExt for [T] {
    type Item = T;

    fn get_two_mut(&mut self, a: usize, b: usize) -> (&mut Self::Item, &mut Self::Item) {
        if a == b {
            panic!(
                "[T]::get_two_mut(): indices must be disjoint, given: (a={}, b={})",
                a, b
            );
        }

        unsafe {
            let ar = &mut *(self.get_unchecked_mut(a) as *mut _);
            let br = &mut *(self.get_unchecked_mut(b) as *mut _);
            (ar, br)
        }
    }
}

// Reduce implementation by dtolnay, until fold_first() is stablized.
pub trait Reduce<T> {
    fn reduce<F>(self, f: F) -> Option<T>
    where
        Self: Sized,
        F: FnMut(T, T) -> T;
}

impl<T, I> Reduce<T> for I
where
    I: Iterator<Item = T>,
{
    #[inline]
    fn reduce<F>(mut self, f: F) -> Option<T>
    where
        Self: Sized,
        F: FnMut(T, T) -> T,
    {
        self.next().map(|first| self.fold(first, f))
    }
}
