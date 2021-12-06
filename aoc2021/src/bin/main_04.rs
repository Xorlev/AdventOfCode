use crate::Spot::{Marked, Unmarked};
use failure::bail;
use itertools::Itertools;
use std::cmp::Ordering;
use std::collections::HashMap;
use std::hash::Hash;
use util::aoc::frequency::FrequencyMap;
use util::aoc::*;

fn main() -> AocResult<()> {
    let input: String = input::read_all(4)?;
    let (drawn_numbers, boards) = parse(&input);

    result("Part 1", || part1(&drawn_numbers, boards.clone()));

    Ok(())
}

fn parse(input: &str) -> (Vec<i32>, Vec<BingoBoard>) {
    let mut parts = input.split("\n\n");
    let drawn_numbers = parts
        .next()
        .unwrap()
        .split(",")
        .map(|s| s.parse().unwrap())
        .collect();
    let bingo_boards = parts
        .map(|input_block| {
            println!("input: {:?}", input_block.split_whitespace().collect_vec());
            let values = input_block.split_whitespace().map(|s| s.parse().unwrap());
            BingoBoard::new(values)
        })
        .collect();

    (drawn_numbers, bingo_boards)
}

fn part1(drawn_numbers: &[i32], mut boards: Vec<BingoBoard>) -> i32 {
    for num in drawn_numbers {
        println!("=== Drawn: {}", num);
        for board in boards.iter_mut() {
            board.display_board();
            if board.mark(*num) {
                return num * board.unmarked_summed();
            }
        }
    }

    0
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
enum Spot {
    Unmarked(i32),
    Marked(i32),
}

impl Spot {
    fn is_marked(&self) -> bool {
        match self {
            Unmarked(_) => false,
            Marked(_) => true,
        }
    }
    fn number(&self) -> i32 {
        match self {
            Unmarked(v) => *v,
            Marked(v) => *v,
        }
    }
}

#[derive(Clone, Debug)]
struct BingoBoard {
    numbers: Vec<Spot>,
}

impl BingoBoard {
    fn new<T: IntoIterator<Item = i32>>(iter: T) -> BingoBoard {
        BingoBoard {
            numbers: iter.into_iter().map(|v| Unmarked(v)).collect(),
        }
    }

    fn mark(&mut self, number: i32) -> bool {
        for i in 0..self.numbers.len() {
            if Unmarked(number) == self.numbers[i] {
                self.numbers[i] = Marked(number);
            }
        }

        self.has_bingo()
    }

    fn unmarked_summed(&self) -> i32 {
        self.numbers
            .iter()
            .filter_map(|spot| match spot {
                Unmarked(v) => Some(*v),
                Marked(_) => None,
            })
            .sum()
    }

    fn has_bingo(&self) -> bool {
        // Check rows
        if self
            .numbers
            .chunks(5)
            .any(|row| row.iter().all(|spot| spot.is_marked()))
        {
            return true;
        }

        // columns, column + 0 -5
        for column in 0..5 {
            // Diagonal: top left to bottom right
            if PointIterator::new(Point::new(column, 0), Point::new(0, 1))
                .take(5)
                .map(|p| self.numbers[p.to_index(5)])
                .all(|spot| spot.is_marked())
            {
                return true;
            }
        }

        false
    }

    fn display_board(&self) {
        for y in 0..5 {
            for x in 0..5 {
                let spot = self.numbers[Point::new(x, y).to_index(5)];
                match spot {
                    Unmarked(v) => print!(" {} ", v),
                    Marked(v) => print!(" X "),
                }
            }
            println!()
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE_INPUT: &'static str = r#"7,4,9,5,11,17,23,2,0,14,21,24,10,16,13,6,15,25,12,22,18,20,8,19,3,26,1

22 13 17 11  0
 8  2 23  4 24
21  9 14 16  7
 6 10  3 18  5
 1 12 20 15 19

 3 15  0  2 22
 9 18 13 17  5
19  8  7 25 23
20 11 10 24  4
14 21 16 12  6

14 21 17 24  4
10 16 15  9 19
18  8 23 26 20
22 11 13  6  5
 2  0 12  3  7"#;

    #[test]
    fn part1_sample() {
        let sample = SAMPLE_INPUT.split("\n").map(|s| s.to_string());
        let (drawn_numbers, boards) = parse(&SAMPLE_INPUT);

        assert_eq!(4512, part1(&drawn_numbers, boards));
    }
}
