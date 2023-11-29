use aoc_runner_derive::{aoc, aoc_generator};
use lazy_static::lazy_static;
use regex::Regex;
use std::ops::RangeInclusive;

lazy_static! {
    static ref RE: Regex = Regex::new("(\\d+)-(\\d+),(\\d+)-(\\d+)").unwrap();
}

#[aoc_generator(day4)]
fn parse(input: &str) -> Vec<(RangeInclusive<i32>, RangeInclusive<i32>)> {
    input
        .lines()
        .map(|pair| {
            let captures = RE.captures(pair).unwrap();

            (
                captures[1].parse().unwrap()..=captures[2].parse().unwrap(),
                captures[3].parse().unwrap()..=captures[4].parse().unwrap(),
            )
        })
        .collect::<Vec<_>>()
}

#[aoc(day4, part1)]
fn part1(assignments: &[(RangeInclusive<i32>, RangeInclusive<i32>)]) -> i32 {
    assignments
        .iter()
        .filter(|(l, r)| {
            (l.start() >= r.start() && l.end() <= r.end())
                || (r.start() >= l.start() && r.end() <= l.end())
        })
        .count() as i32
}

#[aoc(day4, part2)]
fn part2(assignments: &[(RangeInclusive<i32>, RangeInclusive<i32>)]) -> i32 {
    assignments
        .iter()
        .filter(|(l, r)| {
            l.start() <= r.start() && l.end() >= r.start()
                || r.start() <= l.start() && r.end() >= l.start()
        })
        .count() as i32
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn p1() {
        let input = "2-4,6-8
2-3,4-5
5-7,7-9
2-8,3-7
6-6,4-6
2-6,4-8";
        let assignments = parse(input);

        assert_eq!(2, part1(&assignments));
    }

    #[test]
    fn p2() {
        let input = "2-4,6-8
2-3,4-5
5-7,7-9
2-8,3-7
6-6,4-6
2-6,4-8";
        let assignments = parse(input);

        assert_eq!(4, part2(&assignments));
    }
}
