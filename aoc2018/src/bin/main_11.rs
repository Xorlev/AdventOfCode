use failure::*;
use itertools::Itertools;
use lazy_static::*;
use regex::Regex;
use std::collections::HashMap;
use util::aoc::*;

fn main() -> Result<(), Box<std::error::Error>> {
    result("Part 1", || part1(1723));
    result("Part 2", || part2(1723));

    Ok(())
}

fn part1(serial_number: i32) -> Option<Point> {
    solve(serial_number, 3).map(|s| s.0)
}

fn part2(serial_number: i32) -> Option<(Point, usize)> {
    let mut best_grid_size = 1;
    let mut max_power = 0;
    let mut best_point = None;

    for grid_size in 1..=30 {
        println!("Grid size: {}", grid_size);
        if let Some(solution) = solve(serial_number, grid_size) {
            if solution.1 > max_power {
                best_point = Some(solution.0);
                max_power = solution.1;
                best_grid_size = grid_size;

                println!("Best: {:?}, {}", best_point, grid_size);
            }
        }
    }

    best_point.map(|p| (p, best_grid_size))
}

fn solve(serial_number: i32, grid_side_length: usize) -> Option<(Point, i32)> {
    // We're looking for the largest 3x3 block of fuel cell powers, so it can't be on the edge.
    let mut max_power = 0;
    let mut best_grid_point: Option<Point> = None;

    let min_point = 0;
    let max_point = 300 - grid_side_length as i32;
    let mut powers = HashMap::new();

    for x in 0..300 {
        for y in 0..300 {
            let point = Point::new(x, y);
            powers.insert(point, fuel_cell_power(serial_number, &point));
        }
    }

    for x in min_point..=max_point {
        for y in min_point..=max_point {
            let point = Point::new(x, y);
            let power = point
                    .square(grid_side_length)
                    .iter()
                    .map(|p| powers.get(p).ok_or_else(|| format_err!("Invalid point: {:?}", p)).unwrap())
                    .sum::<i32>();

            if power > max_power {
                max_power = power;
                best_grid_point = Some(point);
            }
        }
    }

    best_grid_point.map(|p| (p, max_power))
}

fn fuel_cell_power(grid_serial_number: i32, coordinate: &Point) -> i32 {
    //    Find the fuel cell's rack ID, which is its X coordinate plus 10.
    //    Begin with a power level of the rack ID times the Y coordinate.
    //    Increase the power level by the value of the grid serial number (your puzzle input).
    //    Set the power level to itself multiplied by the rack ID.
    //    Keep only the hundreds digit of the power level (so 12345 becomes 3; numbers with no hundreds digit become 0).
    //    Subtract 5 from the power level.

    let rack_id = coordinate.x + 10;
    let mut power_level = rack_id * coordinate.y;
    power_level += grid_serial_number;
    power_level *= rack_id;
    let str_power_level = power_level.to_string();
    power_level = str_power_level.chars().rev().nth(2).unwrap_or('0') as i32 - 48;
    power_level -= 5;

    power_level
}

mod test {
    use super::*;

    #[test]
    fn fuel_cell_power() {
        let pt1 = Point::new(122, 79);
        let pt2 = Point::new(217, 196);
        let pt3 = Point::new(101, 153);
        assert_eq!(-5, super::fuel_cell_power(57, &pt1));
        assert_eq!(0, super::fuel_cell_power(39, &pt2));
        assert_eq!(4, super::fuel_cell_power(71, &pt3));
    }
}
