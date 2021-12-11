use itertools::{Itertools, MinMaxResult};
use std::collections::{HashMap, HashSet};
use std::num::ParseIntError;
use std::str::FromStr;
use util::aoc::frequency::FrequencyMap;
use util::aoc::*;

fn main() -> AocResult<()> {
    let input: Vec<(Vec<String>, Vec<String>)> = parse(input::read(8)?);

    result("Part 1", || part1(&input));
    result("Part 2", || part2(&input));

    Ok(())
}

fn parse(lines: Vec<String>) -> Vec<(Vec<String>, Vec<String>)> {
    lines
        .into_iter()
        .map(|line| {
            let parts: Vec<&str> = line.split(" | ").collect();
            (
                parts[0].split_whitespace().map(|s| s.to_string()).collect(),
                parts[1].split_whitespace().map(|s| s.to_string()).collect(),
            )
        })
        .collect()
}

fn part1(inputs: &[(Vec<String>, Vec<String>)]) -> i32 {
    let mut frequency = FrequencyMap::new();
    inputs.iter().for_each(|(_, output_values)| {
        output_values
            .iter()
            .for_each(|value| frequency.add(value.len()))
    });

    (frequency.count(&2) + frequency.count(&3) + frequency.count(&4) + frequency.count(&7)) as i32
}

fn part2(inputs: &[(Vec<String>, Vec<String>)]) -> i32 {
    inputs
        .iter()
        .map(|(signals, outputs)| decode_line(signals, outputs))
        .sum()
}

fn decode_line(signals: &[String], outputs: &[String]) -> i32 {
    let signals_by_length = signals
        .iter()
        .map(|signal| signal.chars().collect::<HashSet<_>>())
        .filter(|s| s.len() == 2 || s.len() == 4)
        .fold(HashMap::new(), |mut acc, s| {
            acc.insert(s.len(), s);
            acc
        });

    let mut digits = String::new();
    for output in outputs {
        match output.len() {
            2 => digits += "1",
            3 => digits += "7",
            4 => digits += "4",
            7 => digits += "8",
            5 => {
                let chars = output.chars().collect::<HashSet<_>>();
                if intersection_count(&signals_by_length, &chars, 2) == 2 {
                    digits += "3"
                } else if intersection_count(&signals_by_length, &chars, 4) == 2 {
                    digits += "2"
                } else {
                    digits += "5"
                }
            }
            6 => {
                let chars = output.chars().collect::<HashSet<_>>();
                if intersection_count(&signals_by_length, &chars, 2) == 1 {
                    digits += "6"
                } else if intersection_count(&signals_by_length, &chars, 4) == 4 {
                    digits += "9"
                } else {
                    digits += "0"
                }
            }
            _ => panic!("Unexpected length: {}", output),
        }
    }

    digits.parse().expect("expected an i32")
}

fn intersection_count(
    signals_by_length: &HashMap<usize, HashSet<char>>,
    chars: &HashSet<char>,
    length: usize,
) -> usize {
    signals_by_length
        .get(&length)
        .map(|signal| chars.intersection(signal).count())
        .unwrap_or(0)
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE_INPUT: &str = r#"be cfbegad cbdgef fgaecd cgeb fdcge agebfd fecdb fabcd edb | fdgacbe cefdb cefbgd gcbe
edbfga begcd cbg gc gcadebf fbgde acbgfd abcde gfcbed gfec | fcgedb cgb dgebacf gc
fgaebd cg bdaec gdafb agbcfd gdcbef bgcad gfac gcb cdgabef | cg cg fdcagb cbg
fbegcd cbd adcefb dageb afcb bc aefdc ecdab fgdeca fcdbega | efabcd cedba gadfec cb
aecbfdg fbg gf bafeg dbefa fcge gcbea fcaegb dgceab fcbdga | gecf egdcabf bgf bfgea
fgeab ca afcebg bdacfeg cfaedg gcfdb baec bfadeg bafgc acf | gebdcfa ecba ca fadegcb
dbcfg fgd bdegcaf fgec aegbdf ecdfab fbedc dacgb gdcebf gf | cefg dcbef fcge gbcadfe
bdfegc cbegaf gecbf dfcage bdacg ed bedf ced adcbefg gebcd | ed bcgafe cdgba cbgef
egadfb cdbfeg cegd fecab cgb gbdefca cg fgcdab egfdb bfceg | gbdfcae bgc cg cgb
gcafb gcf dcaebfg ecagb gf abcdeg gaef cafbge fdbac fegbdc | fgae cfgab fg bagce"#;

    #[test]
    fn part1_sample() {
        let patterns = parse(SAMPLE_INPUT.lines().map(|s| s.to_string()).collect_vec());

        assert_eq!(26, part1(&patterns));
    }

    #[test]
    fn part2_sample() {
        let patterns = parse(SAMPLE_INPUT.lines().map(|s| s.to_string()).collect_vec());

        assert_eq!(61229, part2(&patterns));
    }

    #[test]
    fn decode_test() {
        let samples = parse(vec![
            "acedgfb cdfbe gcdfa fbcad dab cefabd cdfgeb eafb cagedb ab | cdfeb fcadb cdfeb cdbaf"
                .to_string(),
        ]);

        assert_eq!(5353, decode_line(&samples[0].0, &samples[0].1));
    }
}
