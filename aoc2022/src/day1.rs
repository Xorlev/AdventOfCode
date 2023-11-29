use aoc_runner_derive::{aoc, aoc_generator};
use itertools::Itertools;
use util::aoc::top_k::TopK;

#[aoc_generator(day1)]
fn parse(input: &str) -> Vec<Vec<i32>> {
    input
        .lines()
        .batching(|line_iter| {
            let elf_calories: Vec<i32> = line_iter
                .take_while(|line| !line.is_empty())
                .map(|line| line.parse().unwrap())
                .collect();
            if elf_calories.len() > 0 {
                Some(elf_calories)
            } else {
                None
            }
        })
        .collect()
}

#[aoc(day1, part1)]
fn part1(carried_calories: &[Vec<i32>]) -> i32 {
    carried_calories
        .iter()
        .map(|food| food.iter().sum())
        .max()
        .unwrap_or(-1)
}

#[aoc(day1, part2)]
fn part2(carried_calories: &[Vec<i32>]) -> i32 {
    carried_calories
        .iter()
        .map(|food| food.iter().sum())
        .topk(3)
        .iter()
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn calorie_counting_p1() {
        let carried_calories = vec![
            vec![1000, 2000, 3000],
            vec![4000],
            vec![5000, 6000],
            vec![7000, 8000, 9000],
            vec![10_000],
        ];

        assert_eq!(24000, part1(&carried_calories));
    }

    #[test]
    fn calorie_counting_p2() {
        let carried_calories = vec![
            vec![1000, 2000, 3000],
            vec![4000],
            vec![5000, 6000],
            vec![7000, 8000, 9000],
            vec![10_000],
        ];

        assert_eq!(45000, part2(&carried_calories));
    }
}
