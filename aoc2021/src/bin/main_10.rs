use itertools::Itertools;
use util::aoc::*;

fn main() -> AocResult<()> {
    let inputs: Vec<Vec<char>> = input::read(10)?
        .iter()
        .map(|s| s.chars().collect_vec())
        .collect();

    result("Part 1", || part1(&inputs));
    result("Part 2", || part2(&inputs));

    Ok(())
}

fn part1(inputs: &[Vec<char>]) -> i32 {
    inputs
        .iter()
        .filter_map(|input| find_first_illegal_close(input))
        .map(|illegal_char| match illegal_char {
            ')' => 3,
            ']' => 57,
            '}' => 1197,
            '>' => 25137,
            _ => 0,
        })
        .sum()
}

fn part2(inputs: &[Vec<char>]) -> i64 {
    let scores = inputs
        .iter()
        .filter_map(|input| complete_line(input))
        .map(|completion| {
            let mut score = 0;
            for c in completion {
                score *= 5;
                score += match c {
                    ')' => 1,
                    ']' => 2,
                    '}' => 3,
                    '>' => 4,
                    _ => 0,
                }
            }

            score
        })
        .sorted()
        .collect::<Vec<_>>();

    println!("Scores: {:?}", scores);

    scores[scores.len() / 2]
}

fn find_first_illegal_close(input: &[char]) -> Option<char> {
    let mut stack = Vec::new();
    for &c in input {
        if is_opening_character(c) {
            stack.push(c);
        } else if let Some(open_character) = stack.pop() {
            if c != closing_character(open_character) {
                return Some(c);
            }
        }
    }

    None
}

fn complete_line(input: &[char]) -> Option<Vec<char>> {
    let mut stack = Vec::new();
    for &c in input {
        if is_opening_character(c) {
            stack.push(c);
        } else if let Some(open_character) = stack.pop() {
            if c != closing_character(open_character) {
                return None;
            }
        }
    }

    Some(stack.into_iter().rev().map(closing_character).collect())
}

fn is_opening_character(c: char) -> bool {
    c == '(' || c == '{' || c == '<' || c == '['
}

fn closing_character(c: char) -> char {
    match c {
        '(' => ')',
        '{' => '}',
        '<' => '>',
        '[' => ']',
        _ => panic!("Unrecognzied character: {}", c),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE_INPUT: &str = r#"[({(<(())[]>[[{[]{<()<>>
[(()[<>])]({[<{<<[]>>(
{([(<{}[<>[]}>{[]{[(<()>
(((({<>}<{<{<>}{[]{[]{}
[[<[([]))<([[{}[[()]]]
[{[{({}]{}}([{[{{{}}([]
{<[[]]>}<{[{[{[]{()[[[]
[<(<(<(<{}))><([]([]()
<{([([[(<>()){}]>(<<{{
<{([{{}}[<[[[<>{}]]]>[]]"#;

    #[test]
    fn test_find_first_illegal_close() {
        let input = "{([(<{}[<>[]}>{[]{[(<()>".chars().collect_vec();

        assert_eq!(Some('}'), find_first_illegal_close(&input))
    }

    #[test]
    fn part1_sample() {
        let inputs = SAMPLE_INPUT
            .lines()
            .map(|s| s.chars().collect_vec())
            .collect_vec();

        assert_eq!(26397, part1(&inputs));
    }

    #[test]
    fn part2_sample() {
        let inputs = SAMPLE_INPUT
            .lines()
            .map(|s| s.chars().collect_vec())
            .collect_vec();

        assert_eq!(288957, part2(&inputs));
    }
}
