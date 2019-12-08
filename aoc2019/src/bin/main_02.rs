use failure::{bail, format_err, Error};
use itertools::Itertools;
use std::collections::hash_set::HashSet;
use std::str::FromStr;
use util::aoc::*;
use aoc2019::intcode::*;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let lines: Vec<String> = input::read(2)?
        .first()
        .map(|f| f.split(",").map(|s| s.to_string()).collect())
        .unwrap_or(Vec::new());
    let memory = lines.parse::<i32>()?;

    result("Part 1", || part1(memory.clone()).unwrap());
    result("Part 2", || part2(memory.clone()).unwrap());

    Ok(())
}

fn part1(memory: Vec<i32>) -> Result<i32, Error> {
    Intcode::init(memory.clone()).execute()
}

fn part2(memory: Vec<i32>) -> Result<i32, Error> {
    for noun in 0..1000 {
        for verb in 0i32..(memory.len() - 1) as i32 {
            let mut new_memory = memory.clone();
            new_memory[1] = noun;
            new_memory[2] = verb;

            let result = Intcode::init(new_memory).execute()?;
            if result == 19690720 {
                return Ok(100 * noun + verb);
            }
        }
    }

    Err(format_err!("Didn't find noun/verb."))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_program() {
        assert_eq!(
            part1(vec![1, 9, 10, 3, 2, 3, 11, 0, 99, 30, 40, 50]).unwrap(),
            3500
        );
    }
}