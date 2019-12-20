use std::borrow::Borrow;
use std::collections::hash_set::HashSet;
use std::collections::{HashMap, VecDeque};
use std::fmt::Write;
use std::str::FromStr;

use failure::_core::fmt::Formatter;
use failure::{bail, format_err, Error};
use itertools::{Itertools, MinMaxResult};
use std::f32;
use util::aoc::*;

use aoc2019::intcode::*;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let lines: Vec<String> = input::read(11)?
        .first()
        .map(|f| f.split(",").map(|s| s.to_string()).collect())
        .unwrap_or(Vec::new());
    let memory = lines.parse::<i64>()?;

    result("Part 1", || part1(memory.clone()));
    result("Part 2", || part2(memory.clone()));

    Ok(())
}

fn part1(memory: Vec<i64>) -> Result<usize, Error> {
    let mut computer = Computer::init(memory);
    let mut panels = HashMap::new();
    run(computer, &mut panels)
}

fn part2(memory: Vec<i64>) -> Result<usize, Error> {
    let mut computer = Computer::init(memory);
    let mut panels = HashMap::new();
    panels.insert(Point::zero(), Color::White);
    run(computer, &mut panels);

    let x_mm = if let MinMaxResult::MinMax(x_min, x_max) = panels
        .keys()
        .map(|p| p.x)
        .minmax() {
        (x_min, x_max)
    } else {
        (0, 0)
    };

    let y_mm = if let MinMaxResult::MinMax(y_min, y_max) = panels
        .keys()
        .map(|p| p.y)
        .minmax() {
        (y_min, y_max)
    } else {
        (0, 0)
    };

    for y in (y_mm.0..=y_mm.1).rev() {
        for x in x_mm.0..=x_mm.1 {
            let point = Point::new(x, y);
            let color = panels.get(&point).unwrap_or(&Color::Black);

            let output = match color {
                Color::Black => " ",
                Color::White => "#",
            };
            print!("{}", output);
        }
        println!();
    }

    Ok(panels.len())
}

fn run(mut computer: Computer, panels: &mut HashMap<Point, Color>) -> Result<usize, Error> {
    let mut point = Point::zero();
    let mut direction = Direction::Up;
    loop {
        let input = panels.get(&point).unwrap_or(&Color::Black);
        let color = match computer.resume(Some(input.as_input()))? {
            IOResult::Output(o) if o == 0 => Color::Black,
            IOResult::Output(o) if o == 1 => Color::White,
            IOResult::Halt(_) => break,
            _ => return Err(format_err!("Not an output")),
        };
        let turn = match computer.resume(None)? {
            IOResult::Output(o) if o == 0 => Turn::Left,
            IOResult::Output(o) if o == 1 => Turn::Right,
            IOResult::Halt(_) => break,
            _ => return Err(format_err!("Not an output")),
        };

        panels.insert(point, color);
        direction = direction.turn(turn);
        point = direction.next_point(point);
    }

    Ok(panels.len())
}

#[derive(Clone, Copy, Debug)]
enum Color {
    Black,
    White,
}

impl Color {
    fn as_input(&self) -> i64 {
        match self {
            Color::Black => 0,
            Color::White => 1,
        }
    }
}

#[derive(Clone, Copy, Debug)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    fn turn(&self, turn: Turn) -> Direction {
        match (self, turn) {
            (Direction::Up, Turn::Left) => Direction::Left,
            (Direction::Down, Turn::Left) => Direction::Right,
            (Direction::Left, Turn::Left) => Direction::Down,
            (Direction::Right, Turn::Left) => Direction::Up,
            (Direction::Up, Turn::Right) => Direction::Right,
            (Direction::Down, Turn::Right) => Direction::Left,
            (Direction::Left, Turn::Right) => Direction::Up,
            (Direction::Right, Turn::Right) => Direction::Down,
        }
    }

    fn next_point(&self, reference: Point) -> Point {
        match self {
            Direction::Up => reference + Point::new(0, 1),
            Direction::Down => reference - Point::new(0, 1),
            Direction::Left => reference - Point::new(1, 0),
            Direction::Right => reference + Point::new(1, 0),
        }
    }
}

#[derive(Clone, Copy, Debug)]
enum Turn {
    Left,
    Right
}
