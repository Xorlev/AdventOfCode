use aoc2019::intcode::*;
use failure::{bail, format_err, Error};
use itertools::Itertools;
use permutohedron::Heap;
use std::borrow::Borrow;
use std::collections::hash_set::HashSet;
use std::collections::HashMap;
use std::str::FromStr;
use util::aoc::*;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let lines: Vec<String> = input::read(7)?
        .first()
        .map(|f| f.split(",").map(|s| s.to_string()).collect())
        .unwrap_or(Vec::new());
    let memory = lines.parse::<i64>()?;

    result("Part 1", || part1(memory.clone()));
    result("Part 2", || part2(memory.clone()));

    Ok(())
}

fn part1(memory: Vec<i64>) -> Result<i64, Error> {
    let mut phase_settings = vec![4, 3, 2, 1, 0];
    Heap::new(&mut phase_settings)
        .map(|phase_settings| {
            let mut signal = 0;
            for phase_setting in phase_settings {
                signal = amplify(memory.clone(), phase_setting, signal).unwrap();
            }
            signal
        })
        .max()
        .ok_or_else(|| format_err!("Failed to find max."))
}

fn part2(memory: Vec<i64>) -> i64 {
    let mut phase_settings = vec![9, 8, 7, 6, 5];

    Heap::new(&mut phase_settings)
        .map(|phase_settings| amplify_with_feedback(memory.clone(), phase_settings).unwrap())
        .max()
        .unwrap()
}

fn amplify(memory: Vec<i64>, phase_setting: i64, signal: i64) -> Result<i64, Error> {
    let mut amplifier = Amplifier::init(memory);
    let mut iter = vec![phase_setting, signal].into_iter();

    let mut status = amplifier.execute()?;
    loop {
        match status {
            IOResult::InputRequired => status = amplifier.resume(iter.next())?,
            IOResult::Output(output) => return Ok(output),
            IOResult::Halt(halt) => return Ok(halt),
        };
    }
}

fn amplify_with_feedback(memory: Vec<i64>, phase_settings: Vec<i64>) -> Result<i64, Error> {
    let num_amplifiers = phase_settings.len();
    let mut amplifiers = phase_settings
        .into_iter()
        .map(|phase_setting| {
            let mut amp = Amplifier::init(memory.clone());
            amp.resume(Some(phase_setting)).unwrap();
            amp
        })
        .collect_vec();

    let mut signal = 0;
    let mut current_amplifier = 0;
    loop {
        match amplifiers[current_amplifier % num_amplifiers].resume(Some(signal))? {
            IOResult::InputRequired => {}
            IOResult::Output(output) => {
                signal = output;
            }
            IOResult::Halt(halt) => break,
        };
        current_amplifier += 1;
    }

    Ok(signal)
}

struct Amplifier {
    computer: Computer,
    halted: bool,
    state: Option<IOResult>,
}

impl Amplifier {
    fn init(memory: Vec<i64>) -> Self {
        Amplifier {
            computer: Computer::init(memory),
            halted: false,
            state: None,
        }
    }

    fn execute(&mut self) -> Result<IOResult, Error> {
        self.resume(None)
    }

    fn resume(&mut self, input: Option<i64>) -> Result<IOResult, Error> {
        let result = self.computer.resume(input)?;
        if let IOResult::Halt(_) = result {
            self.halted = true;
        }

        self.state = Some(result);
        Ok(self.state.clone().unwrap())
    }
}
