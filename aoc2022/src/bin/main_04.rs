use lazy_static::lazy_static;
use regex::Regex;
use std::hash::Hash;
use std::ops::RangeInclusive;
use util::aoc::*;

lazy_static! {
    static ref RE: Regex = Regex::new("(\\d+)-(\\d+),(\\d+)-(\\d+)").unwrap();
}

fn main() -> AocResult<()> {
    let lines: Vec<String> = input::read(4)?;
    let assignments = parse(lines);

    result("Part 1", || part1(&assignments));
    result("Part 2", || part2(&assignments));

    Ok(())
}

fn part1(assignments: &[(RangeInclusive<i32>, RangeInclusive<i32>)]) -> i32 {
    assignments
        .iter()
        .filter(|(l, r)| {
            (l.start() >= r.start() && l.end() <= r.end())
                || (r.start() >= l.start() && r.end() <= l.end())
        })
        .count() as i32
}

fn part2(assignments: &[(RangeInclusive<i32>, RangeInclusive<i32>)]) -> i32 {
    assignments
        .iter()
        .filter(|(l, r)| {
            l.start() <= r.start() && l.end() >= r.start()
                || r.start() <= l.start() && r.end() >= l.start()
        })
        .count() as i32
}

fn parse(input: Vec<String>) -> Vec<(RangeInclusive<i32>, RangeInclusive<i32>)> {
    input
        .iter()
        .map(|pair| {
            let captures = RE.captures(pair).unwrap();

            (
                captures[1].parse().unwrap()..=captures[2].parse().unwrap(),
                captures[3].parse().unwrap()..=captures[4].parse().unwrap(),
            )
        })
        .collect::<Vec<_>>()
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
2-6,4-8"
            .lines()
            .map(|s| s.to_string())
            .collect();
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
2-6,4-8"
            .lines()
            .map(|s| s.to_string())
            .collect();
        let assignments = parse(input);

        assert_eq!(4, part2(&assignments));
    }
}
