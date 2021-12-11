use itertools::{Itertools, MinMaxResult};
use std::collections::HashMap;
use std::num::ParseIntError;
use std::str::FromStr;
use util::aoc::frequency::FrequencyMap;
use util::aoc::*;

fn main() -> AocResult<()> {
    let input: Vec<(Vec<String>, Vec<String>)> = parse(input::read(8)?);

    result("Part 1", || part1(&input));

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

    // 7 letters: 8
    // 4 letters: 4
    // 3 letters: 7
    // 2 letters: 1

    // If we see two letters, we know those are the right-most values, but not top vs. bottom.
    // Three letters can tell us what the top row is, if we have a two-letter.
    // 4 tells us left-upper, right-upper, center, and bottom-right. With a 4 and a 1, we can determine top right + bottom-right.
    // 8 is useless as it lights up the whole panel.
    // All we need is 5+6 letters...

    // Once we have mappings, then we need to turn the signals back into values
}

fn decode_line(input: &[String]) -> Decoder {
    // First, group + sort all signals.
}

struct Decoder {
    key: HashMap<&str, char>,
}

impl Decoder {
    fn decode(&self, digits: &str) -> i32 {
        digits.split_whitespace()
            .map(|digit| digit.chars().sorted().collect::<String>())

        0
    }
}

enum Signal {
    Top,
    UpperLeft,
    UpperRight,
    Middle,
    BottomLeft,
    BottomRight,
    Bottom,
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
}
