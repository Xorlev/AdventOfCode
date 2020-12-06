use lazy_static::lazy_static;
use regex::Regex;
use util::aoc::frequency::FrequencyMap;
use util::aoc::*;

lazy_static! {
    static ref RE: Regex = Regex::new("(\\d+)-(\\d+) ([a-z]): ([a-z]+)").unwrap();
}

fn main() -> AocResult<()> {
    let passwords: Vec<Password> = input::read(2)?
        .into_iter()
        .map(|line| Password::parse(line))
        .collect();

    result("Part 1", || part1(&passwords));
    result("Part 2", || part2(&passwords));

    Ok(())
}

fn part1(passwords: &[Password]) -> i32 {
    passwords
        .iter()
        .filter(|password| {
            let frequency: FrequencyMap<char> = password.password.iter().cloned().collect();
            let letter_count = frequency.count(&password.policy.letter);

            letter_count >= password.policy.min && letter_count <= password.policy.max
        })
        .count() as i32
}

fn part2(passwords: &[Password]) -> i32 {
    passwords
        .iter()
        .filter(|password| {
            (password.password[password.policy.min as usize - 1] == password.policy.letter)
                ^ (password.password[password.policy.max as usize - 1] == password.policy.letter)
        })
        .count() as i32
}

#[derive(Debug)]
struct Password {
    password: Vec<char>,
    policy: PasswordPolicy,
}

impl Password {
    fn parse(line: String) -> Password {
        let captures = RE.captures(line.as_str()).unwrap();
        Password {
            password: captures[4].to_owned().chars().collect(),
            policy: PasswordPolicy {
                letter: captures[3].parse().unwrap(),
                min: captures[1].parse().unwrap(),
                max: captures[2].parse().unwrap(),
            },
        }
    }
}

#[derive(Debug)]
struct PasswordPolicy {
    letter: char,
    min: u64,
    max: u64,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_p1() {
        let passwords = vec![
            Password::parse(String::from("1-3 a: abcde")),
            Password::parse(String::from("1-3 b: cdefg")),
            Password::parse(String::from("2-9 c: ccccccccc")),
        ];

        assert_eq!(2, part1(&passwords));
    }

    #[test]
    fn example_p2() {
        let passwords = vec![
            Password::parse(String::from("1-3 a: abcde")),
            Password::parse(String::from("1-3 b: cdefg")),
            Password::parse(String::from("2-9 c: ccccccccc")),
        ];

        assert_eq!(1, part2(&passwords));
    }
}
