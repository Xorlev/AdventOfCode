use itertools::Itertools;
use std::collections::hash_map::Entry;
use std::collections::HashMap;
use util::aoc::grid::Grid;
use util::aoc::*;

type OctoGrid = Grid<i32>;

fn main() -> AocResult<()> {
    let input: Vec<Vec<i32>> = input::read(11)?
        .iter()
        .map(|s| {
            s.chars()
                .map(|c| c.to_digit(10).unwrap() as i32)
                .collect_vec()
        })
        .collect();
    let octopi = Grid::new(input);

    result("Part 1", || part1(octopi.clone()));
    result("Part 2", || part2(octopi.clone()));

    Ok(())
}

fn part1(mut octopi: OctoGrid) -> i32 {
    let points = octopi
        .point_iterator()
        .map(|(point, _)| point)
        .collect_vec();

    let mut flashes = 0;
    for step in 0..100 {
        let mut to_check = points.clone();
        while let Some(point) = to_check.pop() {
            if octopi.lookup(&point).is_none() {
                continue;
            }

            let new_energy_level = *octopi.update_fn(&point, |state| state + 1);
            if new_energy_level == 10 {
                flashes += 1;
                to_check.extend(point.neighbors8());
            }
        }

        // Reset all flashed to zero.
        points.iter().for_each(|point| {
            octopi.update_fn(point, |&state| if state > 9 { 0 } else { state });
        })
    }

    flashes
}

fn part2(mut octopi: OctoGrid) -> i32 {
    let points = octopi
        .point_iterator()
        .map(|(point, _)| point)
        .collect_vec();

    let mut step = 0;
    loop {
        step += 1;
        let mut flashed = 0;
        let mut to_check = points.clone();
        while let Some(point) = to_check.pop() {
            if octopi.lookup(&point).is_none() {
                continue;
            }

            let new_energy_level = *octopi.update_fn(&point, |state| state + 1);
            if new_energy_level == 10 {
                flashed += 1;
                to_check.extend(point.neighbors8());
            }
        }

        if flashed == points.len() {
            return step;
        }

        // Reset all flashed to zero.
        points.iter().for_each(|point| {
            octopi.update_fn(point, |&state| if state > 9 { 0 } else { state });
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE_INPUT: &str = r#"5483143223
2745854711
5264556173
6141336146
6357385478
4167524645
2176841721
6882881134
4846848554
5283751526"#;

    #[test]
    fn part1_sample() {
        let inputs = SAMPLE_INPUT
            .lines()
            .map(|s| {
                s.chars()
                    .map(|c| c.to_digit(10).unwrap() as i32)
                    .collect_vec()
            })
            .collect_vec();
        let octopi = Grid::new(inputs);

        assert_eq!(1656, part1(octopi));
    }

    #[test]
    fn part2_sample() {
        let inputs = SAMPLE_INPUT
            .lines()
            .map(|s| {
                s.chars()
                    .map(|c| c.to_digit(10).unwrap() as i32)
                    .collect_vec()
            })
            .collect_vec();
        let octopi = Grid::new(inputs);

        assert_eq!(195, part2(octopi));
    }
}
