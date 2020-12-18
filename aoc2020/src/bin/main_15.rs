use failure::{bail, format_err};
use itertools::Itertools;
use lazy_static::lazy_static;
use regex::Regex;
use std::collections::HashMap;
use std::str::FromStr;
use util::aoc::*;

fn main() -> AocResult<()> {
    let input = input::read(15)?[0]
        .split(",")
        .map(|s| s.parse::<i64>().map_err(|e| e.into()))
        .collect::<AocResult<Vec<i64>>>()?;

    result("Part 1", || part1(&input));
    result("Part 2", || part2(&input));

    Ok(())
}

fn part1(starting_numbers: &[i64]) -> i64 {
    solve(&starting_numbers, 2020)
}

fn part2(starting_numbers: &[i64]) -> i64 {
    solve(&starting_numbers, 30000000)
}

fn solve(starting_numbers: &[i64], end_turn: usize) -> i64 {
    let mut number_at_turn: HashMap<i64, usize> = HashMap::new();
    let mut turn = 1;
    let mut last_spoken = -1;
    while turn <= end_turn {
        let mut to_speak = starting_numbers.get(turn - 1).copied().unwrap_or(0);
        if let Some(previously_spoken_turn) = number_at_turn.insert(last_spoken, turn - 1) {
            to_speak = turn as i64 - 1 - previously_spoken_turn as i64;
        }

        last_spoken = to_speak;
        turn += 1;
    }

    last_spoken
}
