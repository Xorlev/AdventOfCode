use aoc_runner_derive::{aoc, aoc_generator};
use itertools::Itertools;
use lazy_static::lazy_static;
use regex::Regex;
use std::collections::VecDeque;

lazy_static! {
    static ref RE: Regex = Regex::new("move (\\d+) from (\\d+) to (\\d+)").unwrap();
}
#[derive(Clone, Debug)]
struct Move {
    quantity: usize,
    from_index: usize,
    to_index: usize,
}

#[derive(Clone, Debug)]
struct Input {
    stacks: Vec<VecDeque<char>>,
    move_ops: Vec<Move>,
}

impl Input {
    fn top_of_stacks(&self) -> String {
        let mut top_crates = "".to_string();
        for stack in &self.stacks {
            if let Some(crate_value) = stack.front() {
                top_crates.push(*crate_value)
            }
        }

        top_crates
    }
}

#[aoc_generator(day5)]
fn parse(input: &str) -> Input {
    let parts = input.split("\n\n").collect_vec();
    Input {
        stacks: parse_stacks(parts[0]),
        move_ops: parse_move_ops(parts[1]),
    }
}

#[aoc(day5, part1)]
fn part1(input: &Input) -> String {
    let mut input = input.clone();
    for move_op in &input.move_ops {
        for _ in 0..move_op.quantity {
            if let Some(crate_value) = input.stacks[move_op.from_index].pop_front() {
                input.stacks[move_op.to_index].push_front(crate_value)
            }
        }
    }

    input.top_of_stacks()
}

#[aoc(day5, part2)]
fn part2(input: &Input) -> String {
    let mut input = input.clone();
    for move_op in &input.move_ops {
        let picked_crates = input.stacks[move_op.from_index]
            .drain(0..move_op.quantity)
            .rev()
            .collect_vec();
        picked_crates
            .into_iter()
            .for_each(|c| input.stacks[move_op.to_index].push_front(c));
    }

    input.top_of_stacks()
}

fn parse_stacks(input: &str) -> Vec<VecDeque<char>> {
    let mut stacks = vec![];
    let stack_contents = input
        .lines()
        .flat_map(|line| {
            line.chars().enumerate().filter_map(|(i, ch)| {
                if ch.is_alphabetic() {
                    Some((ch, i / 4))
                } else {
                    None
                }
            })
        })
        .collect_vec();

    for (ch, stack) in stack_contents {
        if stacks.len() < stack + 1 {
            stacks.resize(stack + 1, VecDeque::new())
        }

        stacks[stack].push_back(ch);
    }

    stacks
}

fn parse_move_ops(input: &str) -> Vec<Move> {
    input
        .lines()
        .filter_map(|line| RE.captures(line))
        .map(|captures| Move {
            quantity: captures[1].parse::<usize>().unwrap(),
            from_index: captures[2].parse::<usize>().unwrap() - 1,
            to_index: captures[3].parse::<usize>().unwrap() - 1,
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "    [D]    
[N] [C]    
[Z] [M] [P]
 1   2   3 

move 1 from 2 to 1
move 3 from 1 to 3
move 2 from 2 to 1
move 1 from 1 to 2";

    #[test]
    fn p1() {
        assert_eq!("CMZ", part1(&parse(INPUT)));
    }

    #[test]
    fn p2() {
        assert_eq!("MCD", part2(&parse(INPUT)));
    }
}
