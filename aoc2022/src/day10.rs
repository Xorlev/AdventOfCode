use crate::day10::Instruction::Noop;
use failure::bail;
use itertools::Itertools;
use std::fmt::Write;
use std::str::FromStr;
use Instruction::Addx;

enum Instruction {
    Noop,
    Addx(i32),
}

impl FromStr for Instruction {
    type Err = failure::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.split_whitespace().collect_vec().as_slice() {
            ["noop"] => Ok(Noop),
            ["addx", v] => Ok(Addx(v.parse()?)),
            _ => bail!("Unknown instruction: {}", s),
        }
    }
}

#[aoc_generator(day10)]
fn parse(input: &str) -> Vec<Instruction> {
    input
        .lines()
        .map(|instruction| instruction.parse().unwrap())
        .collect()
}

#[aoc(day10, part1)]
fn part1(input: &[Instruction]) -> i32 {
    let sample_cycles = [20, 60, 100, 140, 180, 220];
    let mut signal_strengths = vec![];
    execute(input, &mut |cycle, x| {
        if sample_cycles.contains(&cycle) {
            signal_strengths.push(cycle * x);
        }
    });

    signal_strengths.iter().sum()
}

#[aoc(day10, part2)]
fn part2(input: &[Instruction]) -> String {
    let mut output = String::new();
    execute(input, &mut |cycle, x| {
        if cycle == 0 {
            return;
        }

        let pixel = (cycle - 1) % 40;
        if pixel == 0 {
            writeln!(output, "").unwrap();
        }

        if (pixel - x).abs() <= 1 || (pixel + x).abs() <= 1 {
            write!(output, "#").unwrap();
        } else {
            write!(output, ".").unwrap();
        }
    });

    output
}

fn execute(input: &[Instruction], sample_fn: &mut impl FnMut(i32, i32)) {
    let mut x = 1;
    let mut cycle = 0;
    let mut pc = 0;
    let mut next_instruction = (Noop, 0);
    loop {
        let (instruction, retire_cycle) = &next_instruction;

        sample_fn(cycle, x);

        if cycle == *retire_cycle {
            match instruction {
                Noop => {}
                Addx(delta) => x += *delta,
            }

            next_instruction = match input[pc] {
                Noop => (Noop, cycle + 1),
                Addx(v) => (Addx(v), cycle + 2),
            };

            pc += 1;
        }

        if pc == input.len() {
            break;
        }

        cycle += 1;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    const INPUT: &str = "addx 15
addx -11
addx 6
addx -3
addx 5
addx -1
addx -8
addx 13
addx 4
noop
addx -1
addx 5
addx -1
addx 5
addx -1
addx 5
addx -1
addx 5
addx -1
addx -35
addx 1
addx 24
addx -19
addx 1
addx 16
addx -11
noop
noop
addx 21
addx -15
noop
noop
addx -3
addx 9
addx 1
addx -3
addx 8
addx 1
addx 5
noop
noop
noop
noop
noop
addx -36
noop
addx 1
addx 7
noop
noop
noop
addx 2
addx 6
noop
noop
noop
noop
noop
addx 1
noop
noop
addx 7
addx 1
noop
addx -13
addx 13
addx 7
noop
addx 1
addx -33
noop
noop
noop
addx 2
noop
noop
noop
addx 8
noop
addx -1
addx 2
addx 1
noop
addx 17
addx -9
addx 1
addx 1
addx -3
addx 11
noop
noop
addx 1
noop
addx 1
noop
noop
addx -13
addx -19
addx 1
addx 3
addx 26
addx -30
addx 12
addx -1
addx 3
addx 1
noop
noop
noop
addx -9
addx 18
addx 1
addx 2
noop
noop
addx 9
noop
noop
noop
addx -1
addx 2
addx -37
addx 1
addx 3
noop
addx 15
addx -21
addx 22
addx -6
addx 1
noop
addx 2
addx 1
noop
addx -10
noop
noop
addx 20
addx 1
addx 2
addx 2
addx -6
addx -11
noop
noop
noop";

    #[test]
    fn p1() {
        assert_eq!(13140, part1(&parse(INPUT)));
    }

    #[test]
    fn p2() {
        println!("{}", part2(&parse(INPUT)));
    }
}
