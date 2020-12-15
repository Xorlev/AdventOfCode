use failure::bail;
use itertools::Itertools;
use std::collections::HashMap;
use std::iter::FromIterator;
use std::str::FromStr;
use util::aoc::*;

fn main() -> AocResult<()> {
    let input: Vec<String> = input::read(13)?.parse()?;

    let earliest_timestamp = input[0].parse()?;
    let active_buses_with_offset = input[1]
        .split(",")
        .enumerate()
        .filter_map(|(index, str)| str.parse::<i64>().ok().map(|bus_id| (index, bus_id)))
        .collect_vec();

    result("Part 1", || part1(earliest_timestamp, &active_buses_with_offset))?;
    result("Part 2", || part2(&active_buses_with_offset))?;

    Ok(())
}

fn part1(earliest_timestamp: i64, active_buses: &[(usize, i64)]) -> AocResult<i64> {
    println!(
        "earliest_timestamp: {}, active_buses: {:?}",
        earliest_timestamp, active_buses
    );

    if let Some((bus_id, minutes_to_departure)) = active_buses
        .iter()
        .map(|&(_, bus_id)| {
            let next_departure_time =
                ((earliest_timestamp as f64) / (bus_id as f64)).ceil() as i64 * bus_id;
            let minutes_to_departure = next_departure_time - earliest_timestamp;

            (bus_id, minutes_to_departure)
        })
        .min_by_key(|(_, next)| *next)
    {
        println!(
            "bus_id: {}, minutes_to_departure: {}",
            bus_id, minutes_to_departure
        );
        Ok(bus_id * minutes_to_departure)
    } else {
        bail!("Did not find a bus.");
    }
}

fn part2(bus_lines: &[(usize, i64)]) -> AocResult<i64> {
    // Whoo, chinese remainder theorem.
    // We want to setup a system of linear congruences such that:
    // (time + index) mod bus_id = 0, which we can rewrite as
    // time mod bus_id = bus_id - index. For the CRT to work, all Ns
    // need to be the same (the time), so we move the index to the
    // right side of the equation (modular arithmetic style, of course).
    //
    // ex, for the sample problem: 67,x,7,59,61
    // time = 67 - 0 (mod 67)
    // time = 7 - 2  (mod 7)
    // time = 59 - 3 (mod 59)
    // time = 61 - 4 (mod 61)

    let mut modulii = Vec::new();
    let mut residues = Vec::new();
    for &(index, bus_id) in bus_lines.iter() {
        modulii.push(bus_id);
        residues.push(bus_id - index as i64);
    }

    chinese_remainder(&residues, &modulii)
        .ok_or(failure::format_err!("system is not pairwise coprime"))
}


// I'm not quite motivated enough to write my own CRT impl,
// this is from https://rosettacode.org/wiki/Chinese_remainder_theorem#Rust.
fn egcd(a: i64, b: i64) -> (i64, i64, i64) {
    if a == 0 {
        (b, 0, 1)
    } else {
        let (g, x, y) = egcd(b % a, a);
        (g, y - (b / a) * x, x)
    }
}

fn mod_inv(x: i64, n: i64) -> Option<i64> {
    let (g, x, _) = egcd(x, n);
    if g == 1 {
        Some((x % n + n) % n)
    } else {
        None
    }
}

fn chinese_remainder(residues: &[i64], modulii: &[i64]) -> Option<i64> {
    let prod = modulii.iter().product::<i64>();

    let mut sum = 0;

    for (&residue, &modulus) in residues.iter().zip(modulii) {
        let p = prod / modulus;
        sum += residue * mod_inv(p, modulus)? * p
    }

    Some(sum % prod)
}
