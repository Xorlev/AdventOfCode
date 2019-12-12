use aoc2019::intcode::*;
use failure::{bail, format_err, Error};
use itertools::Itertools;
use permutohedron::Heap;
use std::borrow::Borrow;
use std::collections::hash_set::HashSet;
use std::collections::HashMap;
use std::str::FromStr;
use util::aoc::*;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let lines: Vec<String> = input::read(9)?
        .first()
        .map(|f| f.split(",").map(|s| s.to_string()).collect())
        .unwrap_or(Vec::new());
    let memory = lines.parse::<i64>()?;

    result("Part 1", || solve(memory.clone(), 1));
    result("Part 2", || solve(memory.clone(), 2));

    Ok(())
}

fn solve(memory: Vec<i64>, input: i64) -> Result<i64, Error> {
    let mut computer = Computer::init(memory);
    let result = computer.resume(Some(input))?;
    if let IOResult::Output(result) = result {
        Ok(result)
    } else {
        Err(format_err!("Expected output, got: {:?}", result))
    }
}
