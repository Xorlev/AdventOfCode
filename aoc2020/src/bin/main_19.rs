use std::collections::HashMap;

use itertools::Itertools;
use pest::iterators::{Pair, Pairs};
use pest::Parser;
use pest::prec_climber::{Assoc, Operator, PrecClimber};
use pest_derive::Parser;
use regex::Regex;
use util::aoc::*;

// 0: 4 1 5
// 1: 2 3 | 3 2
// 2: 4 4 | 5 5
// 3: 4 5 | 5 4
// 4: "a"
// 5: "b"
//
// ababbb
// bababa
// abbbab
// aaabbb
// aaaabbb
fn main() -> AocResult<()> {
    let input = input::read_all(19)?;
    let input = input.split("\n\n").collect_vec();
    let mut rules: HashMap<usize, Vec<String>> = input[0]
        .lines()
        .map(|l| {
            let parts = l.split(": ").collect_vec();

            let rule = parts[0].parse().unwrap();
            let sub_rules = parts[1].replace('"', "").split(" ").map(|v| v.to_string()).collect_vec();
            (rule, sub_rules)
        }).collect();

    println!("rules: {:?}", rules);
    let regex = vec!["^", replace_rules2(&rules, &rules[&0]).as_str(), "$"].join("");
    println!("rules: {:?}", regex);
    let re = Regex::new(&regex)?;

    let lines = input[1].lines().collect_vec();


    // recurse into each number token, look up the underlying pattern. Apply parens.
    // a((aa|bb)|(45|54)| 3 2)b


    result("Part 1", || part1(&re, &lines));
    // result("Part 2", || part2(input.clone()));

    Ok(())
}

fn part1(re: &Regex, input: &[&str]) -> usize {
    input.iter().filter(|f| {
        re.is_match(f)
    }).count()
}

fn replace_rules(rules: &[Vec<char>], starting_rule: &[char]) -> String {
    let mut rule = starting_rule.clone().to_vec();
    while rule.iter().any(|v| v.is_numeric()) {
        let mut rule_part = Vec::new();
        for part in rule {
            if part.is_numeric() {
                rule_part.push('(');
                rule_part.extend(rules[part.to_digit(10).unwrap() as usize].clone());
                rule_part.push(')');
            } else {
                rule_part.push(part)
            }
        }
        rule = rule_part;
    }

    rule.iter().join("")
}

fn replace_rules2(rules: &HashMap<usize, Vec<String>>, starting_rule: &[String]) -> String {
    let mut regex = String::new();
    for c in starting_rule {
        match c.parse::<usize>() {
            Ok(v) => {
                regex.push('(');
                regex.extend(replace_rules2(rules, &rules[&v]).chars());
                regex.push(')');
            },
            Err(_) => regex.extend(c.chars()),
        }
    }

    regex
}