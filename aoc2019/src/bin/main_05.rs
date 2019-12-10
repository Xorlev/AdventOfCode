use aoc2019::intcode::*;
use failure::{bail, format_err, Error};
use itertools::Itertools;
use std::collections::hash_set::HashSet;
use std::io::Write;
use std::str::FromStr;
use util::aoc::*;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let lines: Vec<String> = input::read(5)?
        .first()
        .map(|f| f.split(",").map(|s| s.to_string()).collect())
        .unwrap_or(Vec::new());
    let memory = lines.parse::<i32>()?;

    println!(
        "Output: {:?}",
        Intcode::init_with_io(InputIterator {}, memory).execute()
    );
    Ok(())
}

struct InputIterator {}

impl Iterator for InputIterator {
    type Item = i32;

    fn next(&mut self) -> Option<Self::Item> {
        let mut value = String::new();
        print!("Input: ");
        std::io::stdout().flush().unwrap();
        std::io::stdin()
            .read_line(&mut value)
            .expect("error: unable to read user input");
        value.trim().parse().ok()
    }
}
