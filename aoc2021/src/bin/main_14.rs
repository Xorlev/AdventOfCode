use std::collections::HashMap;

use failure::bail;
use itertools::Itertools;
use lazy_static::lazy_static;
use regex::Regex;
use util::aoc::frequency::FrequencyMap;
use util::aoc::*;

type InsertionRules = HashMap<String, String>;

fn main() -> AocResult<()> {
    let (template, insertion_rules) = parse(input::read_all(14)?)?;

    result("Part 1", || part1(template.clone(), &insertion_rules));
    result("Part 2", || part2(template.clone(), &insertion_rules));

    Ok(())
}

fn part1(polymer_template: String, insertion_rules: &InsertionRules) -> i64 {
    solve(polymer_template, insertion_rules, 10)
}

fn part2(polymer_template: String, insertion_rules: &InsertionRules) -> i64 {
    solve(polymer_template, insertion_rules, 40)
}

fn solve(polymer_template: String, insertion_rules: &InsertionRules, steps: usize) -> i64 {
    let mut pair_counts = FrequencyMap::new();
    polymer_template.chars().tuple_windows().for_each(|(a, b)| {
        pair_counts.add(format!("{}{}", a, b));
    });
    pair_counts.add(polymer_template.chars().last().unwrap().to_string());

    for _ in 0..steps {
        let mut new_pair_counts = FrequencyMap::new();
        let pairs: Vec<(String, u64)> = pair_counts
            .entries()
            .map(|(pair, count)| (pair.clone(), *count))
            .collect();
        for (pair, count) in pairs {
            if let Some(rule) = insertion_rules.get(&pair) {
                let new_pair1 = format!("{}{}", pair.chars().nth(0).unwrap(), rule);
                let new_pair2 = format!("{}{}", rule, pair.chars().nth(1).unwrap());
                new_pair_counts.multi_add(new_pair1, count);
                new_pair_counts.multi_add(new_pair2, count);
            } else {
                new_pair_counts.multi_add(pair, count);
            }
        }
        pair_counts = new_pair_counts
    }

    let mut frequencies = FrequencyMap::new();
    pair_counts
        .entries()
        .map(|(pair, count)| (pair.chars().nth(0).unwrap(), *count))
        .for_each(|(c, count)| {
            frequencies.multi_add(c, count);
        });

    if let (Some(min), Some(max)) = (frequencies.min(), frequencies.max()) {
        frequencies.count(max) as i64 - frequencies.count(min) as i64
    } else {
        0
    }
}

lazy_static! {
    static ref RE: Regex = Regex::new("(.+) -> (.+)").unwrap();
}

fn parse(s: String) -> AocResult<(String, InsertionRules)> {
    let parts = s.split("\n\n").collect_vec();

    let mut insertion_rules = HashMap::new();
    for fold in parts[1].lines() {
        if let Some(captures) = RE.captures(fold) {
            insertion_rules.insert(captures[1].to_string(), captures[2].to_string());
        }
    }

    Ok((parts[0].to_string(), insertion_rules))
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE_INPUT: &str = r#"NNCB

CH -> B
HH -> N
CB -> H
NH -> C
HB -> C
HC -> B
HN -> C
NN -> C
BH -> H
NC -> B
NB -> B
BN -> B
BB -> N
BC -> B
CC -> N
CN -> C"#;

    #[test]
    fn part1_sample() {
        let (polymer_template, rules) = parse(SAMPLE_INPUT.to_string()).unwrap();

        assert_eq!(1588, part1(polymer_template, &rules));
    }

    #[test]
    fn part2_sample() {
        let (polymer_template, rules) = parse(SAMPLE_INPUT.to_string()).unwrap();

        assert_eq!(2188189693529, part2(polymer_template, &rules));
    }
}
