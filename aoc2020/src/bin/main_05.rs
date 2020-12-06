use itertools::{Itertools, MinMaxResult};

use std::collections::HashSet;

use failure::bail;
use util::aoc::*;

fn main() -> AocResult<()> {
    let occupied_seats: Vec<Seat> = input::read(5)?
        .iter()
        .map(|pass_code| decode_pass(pass_code))
        .collect::<AocResult<Vec<_>>>()?;

    println!("Occupied size: {}", occupied_seats.len());

    result("Part 1", || part1(&occupied_seats));
    result("Part 2", || part2(&occupied_seats))?;

    Ok(())
}

fn part1(occupied_seats: &[Seat]) -> i32 {
    occupied_seats
        .iter()
        .map(|s| s.seat_id())
        .max()
        .unwrap_or(0)
}

fn part2(occupied_seats: &[Seat]) -> AocResult<i32> {
    if let MinMaxResult::MinMax(min, max) = occupied_seats.iter().minmax_by_key(|k| k.seat_id()) {
        let possible_seats: HashSet<_> = (0..128)
            .into_iter()
            .flat_map(|row| {
                let row = row.clone();
                (0..8).into_iter().map(move |column| Seat::new(row, column))
            })
            .filter(|seat| seat.seat_id() >= min.seat_id() && seat.seat_id() <= max.seat_id())
            .collect();
        let occupied_seats: HashSet<_> = occupied_seats.into_iter().cloned().collect();
        let seat_candidates: Vec<_> = possible_seats.difference(&occupied_seats).collect_vec();

        if seat_candidates.len() == 1 {
            Ok(seat_candidates[0].seat_id())
        } else {
            bail!("did not find seat, candidates: {:?}", seat_candidates)
        }
    } else {
        bail!("unable to find min/max seat")
    }
}

fn decode_pass(pass: &str) -> AocResult<Seat> {
    let chars = pass.chars().collect_vec();
    let row = decode_chunk(&chars[..7], 'F', 'B', 127)?;
    let column = decode_chunk(&chars[7..10], 'L', 'R', 7)?;

    Ok(Seat { row, column })
}

fn decode_chunk(chars: &[char], lower_char: char, upper_char: char, max_init: i32) -> AocResult<i32> {
    chars
        .iter()
        .try_fold((0, max_init, 0), |(min, max, _last), char| {
            let difference = ((max - min) as f32 / 2.0).ceil() as i32;
            match *char {
                c if c == lower_char => Ok((min, max - difference, min)),
                c if c == upper_char => Ok((min + difference, max, max)),
                _ => Err(failure::format_err!(
                    "unexpected character in chunk: {}",
                    char
                )),
            }
        })
        .map(|(_, _, last)| last)
}

#[derive(Debug, Eq, PartialEq, Hash, Clone)]
struct Seat {
    row: i32,
    column: i32,
}

impl Seat {
    fn new(row: i32, column: i32) -> Seat {
        Seat { row, column }
    }

    fn seat_id(&self) -> i32 {
        self.row * 8 + self.column
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_p1() {
        assert_eq!(Seat::new(44, 5), decode_pass("FBFBBFFRLR").unwrap());
        assert_eq!(Seat::new(70, 7), decode_pass("BFFFBBFRRR").unwrap());
        assert_eq!(Seat::new(14, 7), decode_pass("FFFBBBFRRR").unwrap());
        assert_eq!(Seat::new(102, 4), decode_pass("BBFFBBFRLL").unwrap());
        assert_eq!(true, decode_pass("BABFFBBFRLL").is_err());
    }
}
