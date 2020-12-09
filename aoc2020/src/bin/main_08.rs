use std::collections::{HashMap, HashSet, VecDeque};

use failure::_core::str::FromStr;
use failure::bail;
use itertools::Itertools;
use util::aoc::*;

fn main() -> AocResult<()> {
    let instructions: Vec<Instruction> = input::read(8)?
        .iter()
        .map(|line| line.parse())
        .collect::<AocResult<Vec<_>>>()?;

    result("Part 1", || part1(&instructions));
    result("Part 2", || part2(&instructions));

    Ok(())
}

fn part1(instructions: &Vec<Instruction>) -> AocResult<i32> {
    let mut processor = Processor::new(instructions.clone());
    // processor.add_tracer(Box::new(Printer::default()));
    processor.add_tracer(Box::new(LoopDetector::default()));

    match processor.execute() {
        Termination::TracerTerminated(val) => Ok(val),
        Termination::ProgramTerminated(acc) => bail!(
            "program terminated successfully, but was expected to loop, acc: {}",
            acc
        ),
    }
}

fn part2(instructions: &Vec<Instruction>) -> AocResult<i32> {
    // Find candidate instructions to flip.
    let jmp_indices = instructions
        .iter()
        .enumerate()
        .filter_map(|(idx, instruction)| {
            if let Instruction::Jmp(_) = instruction {
                Some(idx)
            } else {
                None
            }
        })
        .collect_vec();

    for idx in jmp_indices {
        let mut modified_instructions = instructions.clone();
        modified_instructions[idx] = Instruction::Nop;

        let mut processor = Processor::new(modified_instructions);
        // processor.add_tracer(Box::new(Printer::default()));
        processor.add_tracer(Box::new(LoopDetector::default()));

        if let Termination::ProgramTerminated(acc) = processor.execute() {
            return Ok(acc);
        }
    }

    bail!("failed to find instruction set that halted")
}

pub trait Tracer {
    /// Returning true will halt execution before executing the instruction at pc.
    fn before_execute(&mut self, pc: i32, acc: i32, instruction: &Vec<Instruction>)
        -> TracerAction;
}

pub enum TracerAction {
    Resume,
    Halt(i32),
}

#[derive(Default)]
struct Printer;

impl Tracer for Printer {
    fn before_execute(
        &mut self,
        pc: i32,
        acc: i32,
        instruction: &Vec<Instruction>,
    ) -> TracerAction {
        println!("{:?}, pc:{}, acc:{}", instruction[pc as usize], pc, acc);

        TracerAction::Resume
    }
}

#[derive(Debug, Default)]
struct LoopDetector {
    pc_set: HashSet<i32>,
}

impl Tracer for LoopDetector {
    fn before_execute(&mut self, pc: i32, acc: i32, _: &Vec<Instruction>) -> TracerAction {
        if !self.pc_set.insert(pc) {
            TracerAction::Halt(acc)
        } else {
            TracerAction::Resume
        }
    }
}

pub struct Processor {
    pc: i32,
    acc: i32,
    instructions: Vec<Instruction>,
    tracers: Vec<Box<dyn Tracer>>,
}

impl Processor {
    pub fn new(instructions: Vec<Instruction>) -> Processor {
        Processor {
            pc: 0,
            acc: 0,
            instructions,
            tracers: Vec::new(),
        }
    }

    fn add_tracer(&mut self, tracer: Box<dyn Tracer>) {
        self.tracers.push(tracer);
    }

    fn execute(&mut self) -> Termination {
        loop {
            if self.terminated() {
                return Termination::ProgramTerminated(self.acc);
            }

            for tracer in &mut self.tracers {
                if let TracerAction::Halt(val) =
                    tracer.before_execute(self.pc, self.acc, &self.instructions)
                {
                    return Termination::TracerTerminated(val);
                }
            }

            let mut pc_delta = 1;
            match self.instructions[self.pc as usize] {
                Instruction::Nop => {}
                Instruction::Acc(value) => self.acc += value,
                Instruction::Jmp(offset) => pc_delta = offset,
            }

            self.pc += pc_delta
        }
    }

    fn terminated(&self) -> bool {
        self.pc == self.instructions.len() as i32
    }
}

#[derive(Debug, Copy, Clone)]
pub enum Termination {
    TracerTerminated(i32),
    ProgramTerminated(i32),
}

#[derive(Debug, Copy, Clone)]
pub enum Instruction {
    Nop,
    Acc(i32),
    Jmp(i32),
}

impl FromStr for Instruction {
    type Err = failure::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut parts = s.split(" ");

        if let (Some(p1), Some(p2)) = (parts.next(), parts.next()) {
            let instruction = match p1 {
                "nop" => Instruction::Nop,
                "acc" => Instruction::Acc(p2.parse()?),
                "jmp" => Instruction::Jmp(p2.parse()?),
                _ => bail!("unexpected instruction: {}", s),
            };

            Ok(instruction)
        } else {
            bail!("bad instruction: {}", s)
        }
    }
}
