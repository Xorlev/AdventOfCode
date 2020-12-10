use failure::bail;
use itertools::{Itertools, MinMaxResult};
use util::aoc::*;

fn main() -> AocResult<()> {
    let outputs: Vec<i64> = input::read(9)?
        .iter()
        .map(|line| line.parse::<i64>().map_err(|e| e.into()))
        .collect::<AocResult<Vec<_>>>()?;

    result("Part 1", || part1(&outputs))?;
    result("Part 2", || part2(&outputs))?;

    Ok(())
}

fn part1(outputs: &[i64]) -> AocResult<i64> {
    find_weak_number(outputs, 25)
}

fn part2(outputs: &[i64]) -> AocResult<i64> {
    let weak_number = find_weak_number(outputs, 25)?;

    let mut running_total = 0;
    let mut start_idx = 0;
    let mut end_idx = 0;
    while running_total != weak_number && start_idx <= end_idx && end_idx < outputs.len() {
        if running_total > weak_number {
            running_total -= outputs[start_idx];
            start_idx += 1;
        } else {
            running_total += outputs[end_idx];
            end_idx += 1;
        }
    }

    if running_total != weak_number {
        bail!("failed to find contiguous set")
    }

    let contiguous_set = &outputs[start_idx..end_idx];
    if let MinMaxResult::MinMax(min, max) = contiguous_set.iter().cloned().minmax() {
        Ok(min + max)
    } else {
        panic!("output set had less than 2 values: {:?}", contiguous_set);
    }
}

fn find_weak_number(outputs: &[i64], preamble_size: usize) -> AocResult<i64> {
    for i in preamble_size..outputs.len() {
        if let None = find_pair(&outputs[i - preamble_size..i], outputs[i]) {
            return Ok(outputs[i]);
        }
    }

    bail!("didn't find weak number")
}

fn find_pair(outputs: &[i64], target: i64) -> Option<(i64, i64)> {
    for i in 0..outputs.len() {
        for output_j in &outputs[i..] {
            let output_i = outputs[i];
            if output_i != *output_j && output_i + output_j == target {
                return Some((output_i, *output_j));
            }
        }
    }

    None
}
