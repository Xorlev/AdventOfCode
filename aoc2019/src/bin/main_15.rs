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

use crate::State::Wall;
use aoc2019::intcode::*;
use std::ops::Add;
use std::slice::Iter;
use failure::_core::time::Duration;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let lines: Vec<String> = input::read(15)?
        .first()
        .map(|f| f.split(",").map(|s| s.to_string()).collect())
        .unwrap_or(Vec::new());
    let memory = lines.parse::<i64>()?;

    result("Part 1", || part1(memory.clone()));
    //    result("Part 2", || part2(memory.clone()));

    Ok(())
}

fn part1(memory: Vec<i64>) -> Result<usize, Error> {
    let mut computer = Computer::init(memory);
    let mut grid: HashMap<Point, State> = HashMap::new();
    let mut queue: VecDeque<(Point, Direction)> = VecDeque::new();
    let mut path: VecDeque<(Point, Direction)> = VecDeque::new();
    queue.extend(candidate_points(Point::zero()));
    grid.insert(Point::zero(), State::Empty);
    loop {
        print_grid(&mut grid);
        std::thread::sleep(Duration::from_millis(100));
        if let Some(step) = queue.pop_front() {
            println!("Executing step: {:?}", step);
            let state = match computer.resume(Some(step.1.to_input()))? {
                IOResult::InputRequired => panic!("Input!?"),
                IOResult::Output(o) => State::from_output(o),
                IOResult::Halt(_) => break,
            };
            println!("Found: {:?}", state);
            grid.insert(step.0, state);

            // If state == Wall, we haven't moved and therefore should try a different direction.
            if state != Wall {
                path.push_front(step);

                // Add unexplored points.
                candidate_points(step.0)
                    .into_iter()
                    .filter(|&p| !grid.contains_key(&p.0))
                    .collect()
                    .peekable()
                    .inspect(|p| println!("Exploring: {:?}", p))
                    .for_each(|c| queue.push_front(c))
            }

        // If state == wall,
        } else {
            // No more places to go, backtrack.
            if let Some(step) = path.pop_front() {
                println!("Ran out of places to go, backtracking: {:?}", step);
                queue.push_back((step.0, step.1.reverse()));
            } else {
                // Nowhere to go.
                println!("Nowhere left to backtrack.");
                break;
            }
        }
    }

    print_grid(&mut grid)

    Ok(0)
}

fn print_grid(grid: &mut HashMap<Point, State>) {
    let x_mm = if let MinMaxResult::MinMax(x_min, x_max) = grid.keys().map(|p| p.x).minmax() {
        (x_min, x_max)
    } else {
        (0, 0)
    };
    let y_mm = if let MinMaxResult::MinMax(y_min, y_max) = grid.keys().map(|p| p.y).minmax() {
        (y_min, y_max)
    } else {
        (0, 0)
    };
    for y in (y_mm.0..=y_mm.1).rev() {
        for x in x_mm.0..=x_mm.1 {
            let point = Point::new(x, y);

            let output = match grid.get(&point) {
                None => " ",
                Some(s) => match s {
                    State::Empty => ".",
                    State::Oxygen => "O",
                    State::Wall => "X",
                },
            };
            print!("{}", output);
        }
        println!();
    }
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
enum Direction {
    North,
    South,
    West,
    East,
}

impl Direction {
    fn to_input(&self) -> i64 {
        match self {
            Direction::North => 1,
            Direction::South => 2,
            Direction::West => 3,
            Direction::East => 4,
        }
    }

    fn reverse(&self) -> Direction {
        match self {
            Direction::North => Direction::South,
            Direction::South => Direction::North,
            Direction::West => Direction::East,
            Direction::East => Direction::West,
        }
    }

    fn iter() -> Iter<'static, Direction> {
        static DIRECTIONS: [Direction; 4] = [
            Direction::North,
            Direction::South,
            Direction::West,
            Direction::East,
        ];

        DIRECTIONS.iter()
    }
}

fn candidate_points(base: Point) -> Vec<(Point, Direction)> {
    Direction::iter().map(|d| (base + *d, *d)).collect()
}

impl Add<Direction> for Point {
    type Output = Point;

    fn add(self, rhs: Direction) -> Self::Output {
        match rhs {
            Direction::North => self + Point::new(0, 1),
            Direction::South => self + Point::new(0, -1),
            Direction::West => self + Point::new(-1, 0),
            Direction::East => self + Point::new(1, 0),
        }
    }
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
enum State {
    Empty,
    Oxygen,
    Wall,
}

impl State {
    fn from_output(input: i64) -> State {
        match input {
            0 => State::Wall,
            1 => State::Empty,
            2 => State::Oxygen,
            _ => panic!("Unrecognized output: {}", input),
        }
    }
}
