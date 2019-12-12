use crate::intcode::Argument::{Immediate, Position, Relative};
use failure::{format_err, Error};
use itertools::repeat_n;
use std::ops::Index;
use util::aoc::digits::Digits;

const ADD: i64 = 1;
const MUL: i64 = 2;
const INPUT: i64 = 3;
const OUTPUT: i64 = 4;
const JUMP_IF_TRUE: i64 = 5;
const JUMP_IF_FALSE: i64 = 6;
const LESS_THAN: i64 = 7;
const EQUALS: i64 = 8;
const ADJUST_BASE: i64 = 9;
const HALT: i64 = 99;

#[derive(Clone, Debug)]
pub enum IOResult {
    InputRequired,
    Output(i64),
    Halt(i64),
}

pub struct Computer {
    memory: Memory,
    pc: usize,
}

impl Computer {
    pub fn init(memory: Vec<i64>) -> Computer {
        Computer {
            memory: Memory {
                memory,
                relative_base: 0,
            },
            pc: 0,
        }
    }

    pub fn execute(&mut self) -> Result<IOResult, Error> {
        self.resume(None)
    }

    pub fn resume(&mut self, mut resume_input: Option<i64>) -> Result<IOResult, Error> {
        loop {
            let operation = Operation::parse_op(&self.memory.get_op(self.pc..))?;
            match operation {
                Operation::Add(left, right, output) => {
                    let value = self.memory.get(left) + self.memory.get(right);
                    self.memory.set(output, value);
                    self.pc += 4;
                }
                Operation::Mul(left, right, output) => {
                    let value = self.memory.get(left) * self.memory.get(right);
                    self.memory.set(output, value);
                    self.pc += 4;
                }
                Operation::Input(input) => match resume_input.take() {
                    Some(resume_input) => {
                        self.memory.set(input, resume_input);
                        self.pc += 2;
                    }
                    None => {
                        return Ok(IOResult::InputRequired);
                    }
                },
                Operation::Output(output) => {
                    self.pc += 2;
                    return Ok(IOResult::Output(self.memory.get(output)));
                }
                Operation::JumpIfTrue(left, right) => {
                    if self.memory.get(left) != 0 {
                        self.pc = self.memory.get(right) as usize;
                    } else {
                        self.pc += 3;
                    }
                }
                Operation::JumpIfFalse(left, right) => {
                    if self.memory.get(left) == 0 {
                        self.pc = self.memory.get(right) as usize;
                    } else {
                        self.pc += 3;
                    }
                }
                Operation::LessThan(left, right, output) => {
                    let value = if self.memory.get(left) < self.memory.get(right) {
                        1
                    } else {
                        0
                    };
                    self.memory.set(output, value);

                    self.pc += 4;
                }
                Operation::Equals(left, right, output) => {
                    let value = if self.memory.get(left) == self.memory.get(right) {
                        1
                    } else {
                        0
                    };
                    self.memory.set(output, value);

                    self.pc += 4;
                }
                Operation::AdjustBase(param) => {
                    self.memory.relative_base += self.memory.get(param);
                    self.pc += 2;
                }
                Operation::Halt => {
                    return Ok(IOResult::Halt(self.memory.get(Position(0))));
                }
            }
        }
    }
}

struct Memory {
    memory: Vec<i64>,
    relative_base: i64,
}

impl Memory {
    fn get(&mut self, arg: Argument) -> i64 {
        match arg {
            Position(addr) => {
                self.extend_if_necessary(addr as usize);
                self.memory[addr as usize]
            }
            Immediate(value) => value,
            Relative(addr) => {
                let addr = (addr + self.relative_base) as usize;
                self.extend_if_necessary(addr as usize);
                self.memory[addr as usize]
            }
        }
    }

    fn get_op(&self, arg: std::ops::RangeFrom<usize>) -> &[i64] {
        self.memory.index(arg)
    }

    fn set(&mut self, arg: Argument, value: i64) {
        match arg {
            Position(addr) => {
                self.extend_if_necessary(addr as usize);
                self.memory[addr as usize] = value;
            }
            Immediate(_) => panic!("Immediate should never be used for writes."),
            Relative(addr) => {
                let addr = (addr + self.relative_base) as usize;
                self.extend_if_necessary(addr);
                self.memory[addr] = value;
            }
        }
    }

    fn extend_if_necessary(&mut self, addr: usize) {
        if addr >= self.memory.len() {
            self.memory
                .extend(repeat_n(0, addr - self.memory.len() + 1))
        }
    }
}

#[derive(Debug)]
enum Argument {
    Position(i64),
    Immediate(i64),
    Relative(i64),
}

impl Argument {
    fn from(mode: i64, arg: i64) -> Argument {
        match mode {
            0 => Position(arg),
            1 => Immediate(arg),
            2 => Relative(arg),
            _ => panic!("Bad parameter mode"),
        }
    }
}

#[derive(Debug)]
enum Operation {
    Add(Argument, Argument, Argument),
    Mul(Argument, Argument, Argument),
    Input(Argument),
    Output(Argument),
    JumpIfTrue(Argument, Argument),
    JumpIfFalse(Argument, Argument),
    LessThan(Argument, Argument, Argument),
    Equals(Argument, Argument, Argument),
    AdjustBase(Argument),
    Halt,
}

impl Operation {
    pub fn parse_op(memory: &[i64]) -> Result<Operation, Error> {
        let opcode = OpCodeWithModes::parse(memory[0]);
        match opcode.opcode {
            ADD => Ok(Operation::Add(
                opcode.m(0, memory[1]),
                opcode.m(1, memory[2]),
                opcode.m(2, memory[3]),
            )),
            MUL => Ok(Operation::Mul(
                opcode.m(0, memory[1]),
                opcode.m(1, memory[2]),
                opcode.m(2, memory[3]),
            )),
            INPUT => Ok(Operation::Input(opcode.m(0, memory[1]))),
            OUTPUT => Ok(Operation::Output(opcode.m(0, memory[1]))),
            JUMP_IF_TRUE => Ok(Operation::JumpIfTrue(
                opcode.m(0, memory[1]),
                opcode.m(1, memory[2]),
            )),
            JUMP_IF_FALSE => Ok(Operation::JumpIfFalse(
                opcode.m(0, memory[1]),
                opcode.m(1, memory[2]),
            )),
            LESS_THAN => Ok(Operation::LessThan(
                opcode.m(0, memory[1]),
                opcode.m(1, memory[2]),
                opcode.m(2, memory[3]),
            )),
            EQUALS => Ok(Operation::Equals(
                opcode.m(0, memory[1]),
                opcode.m(1, memory[2]),
                opcode.m(2, memory[3]),
            )),
            ADJUST_BASE => Ok(Operation::AdjustBase(opcode.m(0, memory[1]))),
            HALT => Ok(Operation::Halt),
            _ => Err(format_err!(
                "Unrecognized opcode: {}, {:?}",
                memory[0],
                opcode
            )),
        }
    }
}

#[derive(Debug, Eq, PartialEq)]
struct OpCodeWithModes {
    modes: Vec<i64>,
    opcode: i64,
}

impl OpCodeWithModes {
    fn parse(full_opcode: i64) -> OpCodeWithModes {
        let mut opcode_parts: Vec<i64> =
            Digits::new(full_opcode as u32).map(|d| d as i64).collect();
        opcode_parts.reverse();

        let opcode = if opcode_parts.len() >= 2 {
            opcode_parts[0] + opcode_parts[1] * 10
        } else {
            opcode_parts[0]
        };

        let left = if opcode_parts.len() >= 3 {
            opcode_parts[2]
        } else {
            0
        };

        let right = if opcode_parts.len() >= 4 {
            opcode_parts[3]
        } else {
            0
        };

        let output = if opcode_parts.len() >= 5 {
            opcode_parts[4]
        } else {
            0
        };

        OpCodeWithModes {
            modes: vec![left, right, output],
            opcode,
        }
    }

    fn m(&self, offset: usize, argument: i64) -> Argument {
        Argument::from(self.modes[offset], argument)
    }

    fn opcode(&self) -> i64 {
        self.opcode
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_opcode() {
        let expected = OpCodeWithModes {
            left: 0,
            right: 1,
            output: 0,
            opcode: 2,
        };

        assert_eq!(expected, OpCodeWithModes::parse(1002))
    }

    #[test]
    fn parse_opcode_short() {
        let expected = OpCodeWithModes {
            left: 1,
            right: 0,
            output: 0,
            opcode: 89,
        };

        assert_eq!(expected, OpCodeWithModes::parse(189))
    }

    #[test]
    fn parse_opcode_short2() {
        let expected = OpCodeWithModes {
            left: 0,
            right: 0,
            output: 0,
            opcode: 2,
        };

        assert_eq!(expected, OpCodeWithModes::parse(2))
    }

    #[test]
    fn example_program() {
        let expected = OpCodeWithModes {
            left: 0,
            right: 0,
            output: 0,
            opcode: 2,
        };

        assert_eq!(expected, OpCodeWithModes::parse(2))
    }
}
