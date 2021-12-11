use itertools::{Itertools, MinMaxResult};
use std::num::ParseIntError;
use util::aoc::*;

fn main() -> AocResult<()> {
    let input: Vec<i32> = input::read_all(7)?
        .split(',')
        .map(|value| value.parse().map_err(|e: ParseIntError| e.into()))
        .collect::<AocResult<_>>()?;

    result("Part 1", || part1(&input));
    result("Part 2", || part2(&input));

    Ok(())
}

fn part1(inputs: &[i32]) -> i32 {
    solve(inputs, |position, input| (position - input).abs())
}

fn part2(inputs: &[i32]) -> i32 {
    solve(inputs, |position, input| {
        let n = (position - input).abs();
        n * (n + 1) / 2
    })
}

fn solve(inputs: &[i32], cost_fn: fn(&i32, &i32) -> i32) -> i32 {
    let (min, max) = match inputs.iter().minmax() {
        MinMaxResult::NoElements => panic!("No elements"),
        MinMaxResult::OneElement(_) => panic!("One element"),
        MinMaxResult::MinMax(min, max) => (*min, *max),
    };

    (min..=max)
        .map(|position| {
            inputs
                .iter()
                .map(|input| cost_fn(&position, input))
                .sum::<i32>()
        })
        .min()
        .unwrap_or(0)
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE_INPUT: &str = r#"16,1,2,0,4,2,7,1,2,14"#;

    #[test]
    fn part1_sample() {
        let positions = SAMPLE_INPUT
            .split(',')
            .map(|value| value.parse().map_err(|e: ParseIntError| e.into()))
            .collect::<AocResult<Vec<_>>>()
            .unwrap();

        assert_eq!(37, part1(&positions));
    }

    #[test]
    fn part2_sample() {
        let positions = SAMPLE_INPUT
            .split(',')
            .map(|value| value.parse().map_err(|e: ParseIntError| e.into()))
            .collect::<AocResult<Vec<_>>>()
            .unwrap();

        assert_eq!(168, part2(&positions));
    }
}
