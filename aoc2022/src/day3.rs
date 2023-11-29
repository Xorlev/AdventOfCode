use aoc_runner_derive::{aoc, aoc_generator};
use bit_set::BitSet;
use itertools::Itertools;

#[derive(Debug)]
struct Rucksack {
    contents: BitSet,
    compartment_one: BitSet,
    compartment_two: BitSet,
}

#[aoc_generator(day3)]
fn parse(input: &str) -> Vec<Rucksack> {
    input
        .lines()
        .map(|line| {
            let (one, two) = line.split_at(line.len() / 2);

            let bs_one = build_bitset(one);
            let bs_two = build_bitset(two);
            Rucksack {
                contents: bs_one.union(&bs_two).collect(),
                compartment_one: bs_one,
                compartment_two: bs_two,
            }
        })
        .collect()
}

#[aoc(day3, part1)]
fn part1(rucksacks: &[Rucksack]) -> i32 {
    rucksacks
        .iter()
        .map(|rucksack| sum_intersected_priorities(rucksack))
        .sum()
}

#[aoc(day3, part2)]
fn part2(rucksacks: &[Rucksack]) -> i32 {
    rucksacks
        .iter()
        .chunks(3)
        .into_iter()
        .map(intersect_groups)
        .sum()
}

fn build_bitset(string: &str) -> BitSet {
    string.chars().map(|c| priority(c) as usize).collect()
}

fn intersect_groups<'a>(group: impl Iterator<Item = &'a Rucksack>) -> i32 {
    let groups = group.collect_vec();
    let common_item: BitSet =
        groups
            .iter()
            .fold(groups[0].contents.clone(), |mut acc, rucksack| {
                acc.intersect_with(&rucksack.contents);
                acc
            });

    common_item.iter().next().unwrap() as i32
}

fn sum_intersected_priorities(rucksack: &Rucksack) -> i32 {
    rucksack
        .compartment_one
        .intersection(&rucksack.compartment_two)
        .map(|item| item as i32)
        .sum()
}

fn priority(c: char) -> i32 {
    if c.is_ascii_uppercase() {
        c as i32 - 38
    } else {
        c as i32 - 96
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn priority_test() {
        assert_eq!(1, priority('a'));
        assert_eq!(26, priority('z'));
        assert_eq!(27, priority('A'));
        assert_eq!(52, priority('Z'));
    }

    #[test]
    fn rucksack_reorg_p1() {
        let input = "vJrwpWtwJgWrhcsFMMfFFhFp
jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
PmmdzqPrVvPwwTWBwg
wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
ttgJtRGJQctTZtZT
CrZsJsPPZsGzwwsLwLmpwMDw";
        let rucksacks = parse(input);

        assert_eq!(157, part1(&rucksacks));
    }

    #[test]
    fn rucksack_reorg_p2() {
        let input = "vJrwpWtwJgWrhcsFMMfFFhFp
jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
PmmdzqPrVvPwwTWBwg
wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
ttgJtRGJQctTZtZT
CrZsJsPPZsGzwwsLwLmpwMDw";
        let rucksacks = parse(input);

        assert_eq!(70, part2(&rucksacks));
    }
}
