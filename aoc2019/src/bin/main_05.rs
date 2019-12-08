use failure::{bail, format_err, Error};
use itertools::Itertools;
use std::collections::hash_set::HashSet;
use std::str::FromStr;
use util::aoc::*;
use aoc2019::intcode::*;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let lines: Vec<String> = input::read(5)?
        .first()
        .map(|f| f.split(",").map(|s| s.to_string()).collect())
        .unwrap_or(Vec::new());
    let memory = lines.parse::<i32>()?;

    Intcode::init(memory).execute();
    Ok(())
}