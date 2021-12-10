use failure::{bail, Error};
use std::num::ParseIntError;
use std::str::FromStr;
use util::aoc::frequency::FrequencyMap;
use util::aoc::*;

fn main() -> AocResult<()> {
    let input: Vec<i32> = input::read_all(6)?
        .split(',')
        .map(|value| value.parse().map_err(|e: ParseIntError| e.into()))
        .collect::<AocResult<_>>()?;

    result("Part 1", || part1(&input));
    result("Part 2", || part2(&input));

    Ok(())
}

fn part1(input: &[i32]) -> u64 {
    solve(input, 80)
}

fn part2(input: &[i32]) -> u64 {
    solve(input, 256)
}

fn solve(input: &[i32], days: i32) -> u64 {
    let frequency_map = FrequencyMap::from_iter(input);
    let mut fish = (0..9)
        .map(|day| frequency_map.count(&&day))
        .collect::<Vec<_>>();

    for day in 0..days {
        fish[(day as usize + 7) % 9] += fish[day as usize % 9];
    }

    fish.iter().sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE_INPUT: &str = r#"3,4,3,1,2"#;

    #[test]
    fn part1_sample() {
        let fish = SAMPLE_INPUT
            .split(',')
            .map(|value| value.parse().map_err(|e: ParseIntError| e.into()))
            .collect::<AocResult<_>>()
            .unwrap();

        assert_eq!(5934, part1(fish));
    }
}
