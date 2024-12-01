use aoc_runner_derive::{aoc, aoc_generator};
use itertools::Itertools;
use util::aoc::frequency::FrequencyMap;

#[aoc_generator(day1)]
fn parse(input: &str) -> (Vec<i32>, Vec<i32>) {
    input
        .lines()
        .map(|l| {
            l.split_whitespace()
                .map(|c| c.parse().unwrap())
                .tuples::<(i32, i32)>()
                .next()
                .unwrap()
        })
        .unzip()
}

#[aoc(day1, part1)]
fn part1(lists: &(Vec<i32>, Vec<i32>)) -> i32 {
    let (a, b) = lists;
    let sorted_a = a.iter().cloned().sorted().collect_vec();
    let sorted_b = b.iter().cloned().sorted().collect_vec();

    sorted_a
        .iter()
        .zip(sorted_b.iter())
        .map(|(a, b)| (a - b).abs())
        .sum()
}

#[aoc(day1, part2)]
fn part2(lists: &(Vec<i32>, Vec<i32>)) -> i32 {
    let (a, b) = lists;
    let b_frequency: FrequencyMap<i32> = b.iter().cloned().collect();

    a.iter().map(|v| v * b_frequency.count(v) as i32).sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn list_distances_p1() {
        let lists = "3   4
4   3
2   5
1   3
3   9
3   3";

        assert_eq!(11, part1(&parse(&lists)));
    }

    #[test]
    fn list_distances_p2() {
        let lists = "3   4
4   3
2   5
1   3
3   9
3   3";

        assert_eq!(31, part2(&parse(&lists)));
    }
}
