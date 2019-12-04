use failure::{bail, format_err, Error};
use itertools::Itertools;
use std::collections::hash_set::HashSet;
use std::str::FromStr;
use util::aoc::*;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let lines: Vec<String> = input::read(2)?
        .first()
        .map(|f| f.split(",").map(|s| s.to_string()).collect())
        .unwrap_or(Vec::new());
    let memory = lines.parse::<i32>()?;

    result("Part 1", || part1(memory.clone()).unwrap());
    result("Part 2", || part2(memory.clone()).unwrap());

    Ok(())
}

fn part1(memory: Vec<i32>) -> Result<i32, Error> {
    execute_program(memory.clone())
}

fn part2(memory: Vec<i32>) -> Result<i32, Error> {
    for noun in 0..1000 {
        for verb in 0..memory.len() - 1 {
            let mut new_memory = memory.clone();
            new_memory[1] = noun;
            new_memory[2] = verb as i32;

            let result = execute_program(new_memory)?;
            if result == 19690720 {
                return Ok(100 * noun + verb as i32);
            }
        }
    }

    Err(format_err!("Didn't find noun/verb."))
}

fn execute_program(mut memory: Vec<i32>) -> Result<i32, Error> {
    let ops: Vec<Operation> = memory
        .chunks(4)
        .filter(|chunk| chunk.len() == 4)
        .map(Operation::parse_op)
        .collect::<Result<Vec<Operation>, _>>()?;
    for op in ops {
        match op {
            Operation::Add(l, r, out) => {
                let left = memory[l as usize];
                let right = memory[r as usize];

                memory[out as usize] = left + right;
            }
            Operation::Mul(l, r, out) => {
                let left = memory[l as usize];
                let right = memory[r as usize];

                memory[out as usize] = left * right;
            }
            Operation::Halt => return Ok(memory[0]),
        }
    }
    Ok(memory[0])
}

enum Operation {
    Add(i32, i32, i32),
    Mul(i32, i32, i32),
    Halt,
}

impl Operation {
    pub fn parse_op(op: &[i32]) -> Result<Operation, Error> {
        if op.len() != 4 {
            bail!("Op must be 4 bytes.");
        }

        match op[0] {
            1 => Ok(Operation::Add(op[1], op[2], op[3])),
            2 => Ok(Operation::Mul(op[1], op[2], op[3])),
            99 => Ok(Operation::Halt),
            _ => Err(format_err!("Unrecognized opcode")),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_program() {
        assert_eq!(
            part1(vec![1, 9, 10, 3, 2, 3, 11, 0, 99, 30, 40, 50]).unwrap(),
            3500
        );
    }
}
