use failure::*;
use itertools::Itertools;
use lazy_static::*;
use regex::Regex;
use std::collections::HashMap;
use util::aoc::*;

fn main() -> Result<(), Box<std::error::Error>> {
    let lines: Vec<String> = input::read(12)?;

    let mut state: Vec<char> = lines[0][15..].to_string().chars().collect();
    println!("{:?}", state);

    let mut rules: Vec<Vec<char>> = Vec::new();
    for line in &lines[2..] {
        let rule = &line[0..line.find(" =>").unwrap_or_default()];
        println!("{:?}", rule);
        rules.push(rule.chars().collect());
    }


    let rules = rules;
    for g in 1..=20 {
        let end_index = state.len() - 5;
        let mut new_state = state.clone();
        for i in 0..end_index {
            let slice1 = &state[i..i+5];
            let mut slice2 = &mut new_state[i..i+5];
            for rule in rules.iter() {
                if rule == &slice1.to_vec() {
                    println!("Matched rule: {:?}", rule);
                    slice2[3] = '#';
                }
            }

        }
        state = new_state;
        println!("{}: {}", g, state.iter().collect::<String>());
    }


    Ok(())
}