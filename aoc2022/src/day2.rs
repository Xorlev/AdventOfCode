use aoc_runner_derive::{aoc, aoc_generator};
use failure::bail;
use itertools::Itertools;
use std::str::FromStr;
use util::aoc::*;

#[aoc_generator(day2)]
fn parse(input: &str) -> Result<Vec<(Move, String)>, failure::Error> {
    input
        .lines()
        .map(|line| match line.split_whitespace().collect_vec()[..] {
            [l, r] => Ok((l.parse::<Move>()?, r.to_string())),
            _ => bail!("Unsupported move pair: {:?}", line),
        })
        .collect::<Result<Vec<_>, failure::Error>>()
}

#[aoc(day2, part1)]
fn part1(strategy_guide: &[(Move, String)]) -> i32 {
    strategy_guide
        .iter()
        .map(|(opponent_move, player_move)| evaluate_move_pair_p1(player_move, *opponent_move))
        .sum::<AocResult<_>>()
        .unwrap()
}

#[aoc(day2, part2)]
fn part2(strategy_guide: &[(Move, String)]) -> i32 {
    strategy_guide
        .iter()
        .map(|(opponent_move, round_outcome)| evaluate_move_pair_p2(round_outcome, *opponent_move))
        .sum::<AocResult<_>>()
        .unwrap()
}

fn evaluate_move_pair_p1(player_move: &str, opponent_move: Move) -> AocResult<i32> {
    let player_move: Move = match player_move {
        "X" => Move::Rock,
        "Y" => Move::Paper,
        "Z" => Move::Scissors,
        _ => bail!("Unrecognized move: {}", player_move),
    };

    Ok(score_move_pair(&opponent_move, player_move))
}

fn evaluate_move_pair_p2(desired_outcome: &str, opponent_move: Move) -> AocResult<i32> {
    let desired_outcome: Outcome = desired_outcome.parse()?;
    let player_move = desired_outcome.outcome_to_move(&opponent_move);

    Ok(score_move_pair(&opponent_move, player_move))
}

fn score_move_pair(opponent_move: &Move, player_move: Move) -> i32 {
    player_move.shape_score()
        + match player_move.move_to_outcome(opponent_move) {
            Outcome::Win => 6,
            Outcome::Loss => 0,
            Outcome::Draw => 3,
        }
}

#[derive(Copy, Clone, Debug)]
enum Outcome {
    Win,
    Loss,
    Draw,
}

impl Outcome {
    fn outcome_to_move(&self, opponent_move: &Move) -> Move {
        match self {
            Outcome::Win => opponent_move.successor(),
            Outcome::Loss => opponent_move.successor().successor(),
            Outcome::Draw => *opponent_move,
        }
    }
}

impl FromStr for Outcome {
    type Err = failure::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "X" => Ok(Self::Loss),
            "Y" => Ok(Self::Draw),
            "Z" => Ok(Self::Win),
            _ => bail!("Unknown letter: {}", s),
        }
    }
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
enum Move {
    Rock,
    Paper,
    Scissors,
}

impl Move {
    fn shape_score(&self) -> i32 {
        match self {
            Move::Rock => 1,
            Move::Paper => 2,
            Move::Scissors => 3,
        }
    }

    fn successor(&self) -> Move {
        match self {
            Move::Rock => Move::Paper,
            Move::Paper => Move::Scissors,
            Move::Scissors => Move::Rock,
        }
    }

    fn move_to_outcome(&self, other: &Move) -> Outcome {
        if self == other {
            Outcome::Draw
        } else if self.successor() == *other {
            Outcome::Loss
        } else {
            Outcome::Win
        }
    }
}

impl FromStr for Move {
    type Err = failure::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "A" => Ok(Self::Rock),
            "B" => Ok(Self::Paper),
            "C" => Ok(Self::Scissors),
            _ => bail!("Unknown letter: {}", s),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn rps_p1() {
        let input = "A Y
B X
C Z";

        let vec = parse(input).unwrap();
        assert_eq!(15, part1(&vec));
    }

    #[test]
    fn rps_p2() {
        let input = "A Y
B X
C Z";

        let vec = parse(input).unwrap();
        assert_eq!(12, part2(&vec));
    }
}
