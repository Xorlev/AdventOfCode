use failure::{bail, format_err, Error};
use itertools::Itertools;
use std::collections::hash_set::HashSet;
use std::collections::HashMap;
use std::str::FromStr;
use util::aoc::digits::Digits;
use util::aoc::*;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let ranges: Vec<u32> = input::read(4)?[0]
        .split("-")
        .collect::<Vec<_>>()
        .parse::<u32>()?;
    let (lo, hi) = (ranges[0], ranges[1]);

    result("Part 1", || part1(lo, hi));
    result("Part 2", || part2(lo, hi));

    Ok(())
}

fn part1(lo: u32, hi: u32) -> u32 {
    count_combinations(lo, hi, |c| c >= 2)
}

fn part2(lo: u32, hi: u32) -> u32 {
    count_combinations(lo, hi, |c| c == 2)
}

fn count_combinations(lo: u32, hi: u32, predicate: impl Fn(u32) -> bool) -> u32 {
    let mut valid_combinations = 0;
    'outer: for i in lo..hi + 1 {
        if check_code(i, &predicate) {
            valid_combinations += 1
        }
    }
    valid_combinations
}

fn check_code(i: u32, predicate: impl Fn(u32) -> bool) -> bool {
    let digits = Digits::new(i);
    let mut consecutive_digits = 0;
    let mut has_two_digit_group = false;
    let mut last_digit = 0;
    for digit in digits {
        if digit < last_digit {
            return false;
        }
        if digit == last_digit || consecutive_digits == 0 {
            consecutive_digits += 1;
        } else if digit != last_digit {
            has_two_digit_group |= predicate(consecutive_digits);
            consecutive_digits = 1;
        }

        last_digit = digit;
    }

    has_two_digit_group || predicate(consecutive_digits)
}
