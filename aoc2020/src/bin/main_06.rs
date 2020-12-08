use std::collections::HashSet;

use failure::bail;
use itertools::{Itertools, MinMaxResult};
use util::aoc::*;

type Answers = HashSet<char>;

fn main() -> AocResult<()> {
    let groups_with_answers: Vec<Group> = input::read_all(6)?
        .split("\n\n")
        .map(|group| {
            group
                .split("\n")
                .map(|answers| {
                    answers
                        .chars()
                        .filter(|c| c.is_alphabetic())
                        .collect::<HashSet<_>>()
                })
                .collect()
        })
        .map(|answers| Group::new(answers))
        .collect();

    result("Part 1", || part1(&groups_with_answers));
    result("Part 2", || part2(&groups_with_answers));

    Ok(())
}

fn part1(group_answers: &[Group]) -> usize {
    group_answers
        .iter()
        .map(|group| group.unioned_responses())
        .sum()
}

fn part2(group_answers: &[Group]) -> usize {
    group_answers
        .iter()
        .map(|group| group.intersected_responses())
        .sum()
}

#[derive(Debug)]
struct Group {
    form_answers: Vec<Answers>,
}

impl Group {
    fn new(form_answers: Vec<Answers>) -> Group {
        Group { form_answers }
    }

    fn unioned_responses(&self) -> usize {
        self.form_answers
            .iter()
            .flat_map(|answers| answers.iter())
            .collect::<HashSet<_>>()
            .len()
    }

    fn intersected_responses(&self) -> usize {
        // There isn't a really elegant way of reducing on a collection of sets without a ton of
        // copies in the standard library. It occurs to me with a cardinality of 26 (a-z), we could
        // also shift these into bitstrings and AND them together. It'd be faster, but 200us on an
        // aging MBP is fine.
        self.form_answers[0]
            .iter()
            .filter(|&f| {
                self.form_answers[1..self.form_answers.len()]
                    .iter()
                    .all(|s| s.contains(f))
            })
            .count()
    }
}
