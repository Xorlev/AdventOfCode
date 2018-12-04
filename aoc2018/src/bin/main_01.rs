use util::aoc::*;
use std::collections::hash_set::HashSet;

fn main() ->  Result<(), Box<std::error::Error>> {
    let lines: Vec<String> = input::read(1)?;

    let frequency_deltas: Vec<i32> = lines
        .iter()
        .map(|delta| delta.parse::<i32>())
        .collect::<Result<Vec<i32>, std::num::ParseIntError>>()?;

    result("Part 1", || part1(&frequency_deltas));
    result("Part 2", || part2(&frequency_deltas));

    Ok(())
}

fn part1(frequency_deltas: &Vec<i32>) -> i32 {
    frequency_deltas.iter().sum()
}

fn part2(frequency_deltas: &Vec<i32>) -> i32 {
    let mut frequencies_seen: HashSet<i32> = HashSet::new();
    let mut frequency = 0;
    for delta in frequency_deltas.iter().cycle() {
        if !frequencies_seen.insert(frequency) {
            return frequency;
        }

        frequency += delta;
    }

    0
}
