use crate::intcode::Parameter::{Immediate, Position};
use failure::{format_err, Error};
use std::borrow::Cow;
use util::aoc::digits::Digits;

const ADD: i32 = 1;
const MUL: i32 = 2;
const INPUT: i32 = 3;
const OUTPUT: i32 = 4;
const JUMP_IF_TRUE: i32 = 5;
const JUMP_IF_FALSE: i32 = 6;
const LESS_THAN: i32 = 7;
const EQUALS: i32 = 8;
const HALT: i32 = 99;

#[derive(Clone, Debug)]
pub enum IOResult {
    InputRequired,
    Output(i32),
    Halt(i32),
}

pub struct Intcode {
    legacy_output_behavior: bool,
    memory: Vec<i32>,
    pc: usize,
}

impl Intcode {
    pub fn init(memory: Vec<i32>) -> Intcode {
        Intcode {
            legacy_output_behavior: true,
            memory,
            pc: 0,
        }
    }

    pub fn execute(&mut self) -> Result<IOResult, Error> {
        self.resume(None)
    }

    pub fn resume(&mut self, mut resume_input: Option<i32>) -> Result<IOResult, Error> {
        loop {
            let operation = Operation::parse_op(&self.memory[self.pc..])?;
            match operation {
                Operation::Add(left, right, output) => {
                    self.memory[output as usize] = self.m(left) + self.m(right);
                    self.pc += 4;
                }
                Operation::Mul(left, right, output) => {
                    self.memory[output as usize] = self.m(left) * self.m(right);
                    self.pc += 4;
                }
                Operation::Input(input) => match resume_input.take() {
                    Some(resume_input) => {
                        self.memory[input as usize] = resume_input;
                        self.pc += 2;
                    }
                    None => {
                        return Ok(IOResult::InputRequired);
                    }
                },
                Operation::Output(output) => {
                    self.pc += 2;
                    return Ok(IOResult::Output(self.m(output)));
                }
                Operation::JumpIfTrue(left, right) => {
                    if self.m(left) != 0 {
                        self.pc = self.m(right) as usize;
                    } else {
                        self.pc += 3;
                    }
                }
                Operation::JumpIfFalse(left, right) => {
                    if self.m(left) == 0 {
                        self.pc = self.m(right) as usize;
                    } else {
                        self.pc += 3;
                    }
                }
                Operation::LessThan(left, right, output) => {
                    self.memory[output as usize] = if self.m(left) < self.m(right) { 1 } else { 0 };

                    self.pc += 4;
                }
                Operation::Equals(left, right, output) => {
                    self.memory[output as usize] =
                        if self.m(left) == self.m(right) { 1 } else { 0 };

                    self.pc += 4;
                }
                Operation::Halt => {
                    return Ok(IOResult::Halt(self.memory[0]));
                }
            }
        }
    }

    fn m(&self, argument: Parameter) -> i32 {
        match argument {
            Parameter::Position(arg) => self.memory[arg as usize],
            Parameter::Immediate(arg) => arg,
        }
    }
}

#[derive(Debug)]
enum Parameter {
    Position(i32),
    Immediate(i32),
}

impl Parameter {
    fn from(mode: i32, arg: i32) -> Parameter {
        match mode {
            0 => Position(arg),
            1 => Immediate(arg),
            _ => panic!("Bad parameter mode"),
        }
    }
}

#[derive(Debug)]
enum Operation {
    Add(Parameter, Parameter, i32),
    Mul(Parameter, Parameter, i32),
    Input(i32),
    Output(Parameter),
    JumpIfTrue(Parameter, Parameter),
    JumpIfFalse(Parameter, Parameter),
    LessThan(Parameter, Parameter, i32),
    Equals(Parameter, Parameter, i32),
    Halt,
}

impl Operation {
    pub fn parse_op(memory: &[i32]) -> Result<Operation, Error> {
        let opcode = OpCodeWithModes::parse(memory[0]);
        match opcode.opcode {
            ADD => Ok(Operation::Add(
                opcode.l(memory[1]),
                opcode.r(memory[2]),
                memory[3],
            )),
            MUL => Ok(Operation::Mul(
                opcode.l(memory[1]),
                opcode.r(memory[2]),
                memory[3],
            )),
            INPUT => Ok(Operation::Input(memory[1])),
            OUTPUT => Ok(Operation::Output(opcode.l(memory[1]))),
            JUMP_IF_TRUE => Ok(Operation::JumpIfTrue(
                opcode.l(memory[1]),
                opcode.r(memory[2]),
            )),
            JUMP_IF_FALSE => Ok(Operation::JumpIfFalse(
                opcode.l(memory[1]),
                opcode.r(memory[2]),
            )),
            LESS_THAN => Ok(Operation::LessThan(
                opcode.l(memory[1]),
                opcode.r(memory[2]),
                memory[3],
            )),
            EQUALS => Ok(Operation::Equals(
                opcode.l(memory[1]),
                opcode.r(memory[2]),
                memory[3],
            )),
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
    left: i32,
    right: i32,
    opcode: i32,
}

impl OpCodeWithModes {
    fn parse(full_opcode: i32) -> OpCodeWithModes {
        let mut opcode_parts: Vec<i32> =
            Digits::new(full_opcode as u32).map(|d| d as i32).collect();
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

        OpCodeWithModes {
            left,
            right,
            opcode,
        }
    }

    fn l(&self, argument: i32) -> Parameter {
        Parameter::from(self.left, argument)
    }

    fn r(&self, argument: i32) -> Parameter {
        Parameter::from(self.right, argument)
    }

    fn opcode(&self) -> i32 {
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
            opcode: 2,
        };

        assert_eq!(expected, OpCodeWithModes::parse(1002))
    }

    #[test]
    fn parse_opcode_short() {
        let expected = OpCodeWithModes {
            left: 1,
            right: 0,
            opcode: 89,
        };

        assert_eq!(expected, OpCodeWithModes::parse(189))
    }

    #[test]
    fn parse_opcode_short2() {
        let expected = OpCodeWithModes {
            left: 0,
            right: 0,
            opcode: 2,
        };

        assert_eq!(expected, OpCodeWithModes::parse(2))
    }

    #[test]
    fn example_program() {
        let expected = OpCodeWithModes {
            left: 0,
            right: 0,
            opcode: 2,
        };

        assert_eq!(expected, OpCodeWithModes::parse(2))
    }
}
