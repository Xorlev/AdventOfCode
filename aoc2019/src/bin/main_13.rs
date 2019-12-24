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
    let lines: Vec<String> = input::read(13)?
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
    let mut outputs = Vec::new();
    loop {
        match computer.resume(None)? {
            IOResult::InputRequired => panic!("Input!?"),
            IOResult::Output(o) => outputs.push(o as i32),
            IOResult::Halt(_) => break,
        }
    }

    let outputs = outputs.chunks(3)
        .map(|o| (Point::new(o[0], o[1]), o[2]))
        .collect::<Vec<_>>();

    let blocks = outputs
        .iter()
        .filter(|&&(_, t)| t == 2)
        .map(|(pt, _)| pt)
        .unique()
        .count();

    Ok(blocks)
}