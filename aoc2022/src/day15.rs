use itertools::Itertools;
use lazy_static::lazy_static;
use std::fmt::{Debug, Display, Formatter};
use util::aoc::grid::HashGrid;
use util::aoc::lines::LineSegment;
use util::aoc::Point;

lazy_static! {
    static ref RE: Regex = Regex::new(
        "Sensor at x=([0-9-]+), y=([0-9-]+) closest beacon is at x=([0-9-]+), y=([0-9-]+)"
    )
    .unwrap();
}

enum Item {
    Sensor,
    Beacon,
}

#[aoc_generator(day15)]
fn parse(input: &str) -> HashGrid<Item> {
    // Turn into LineSegments.
    let segments = input
        .lines()
        .flat_map(|line| {
            line.split(" -> ")
                .tuple_windows()
                .map(|(start, end)| LineSegment::new(start.parse().unwrap(), end.parse().unwrap()))
        })
        .collect_vec();

    // Draw points with Lines.
    let mut grid = HashGrid::new();
    segments
        .iter()
        .flat_map(|s| s.point_iterator())
        .for_each(|point| {
            grid.update(point, Item::Rock);
        });

    grid
}

#[aoc(day15, part1)]
fn part1(input: &HashGrid<Item>) -> usize {
    let mut grid = input.clone();
    let mut iterations = 0;
    loop {
        if simulate_particle(&mut grid) {
            // grid.print_grid();
            // True: sand fell into the abyss, terminate simulation.
            break;
        } else {
            // grid.print_grid();
            iterations += 1;
        }
    }

    iterations
}

#[aoc(day15, part2)]
fn part2(input: &HashGrid<Item>) -> usize {
    let mut grid = input.clone();
    let mut iterations = 0;
    loop {
        iterations += 1;
        if simulate_particle2(&mut grid, input.max_y.unwrap() + 2) {
            // grid.print_grid();
            // True: sand fell into the abyss, terminate simulation.
            break;
        }
    }

    iterations
}

fn simulate_particle(grid: &mut HashGrid<Item>) -> bool {
    let mut point = ORIGIN;
    // Sand keeps moving as long as it is able to do so,
    // at each step trying to move down,
    // then down-left,
    // then down-right.
    // If all three possible destinations are blocked, the unit of sand comes to rest
    while let Some(next_point) = next_move(point)
        .into_iter()
        .find(|candidate| matches!(grid.get(candidate).unwrap_or(&Item::Air), Item::Air))
    {
        point = next_point;
        if Some(point.y) >= grid.max_y {
            return true;
        }
    }

    let _ = grid.update(point, Item::Sand);

    false
}

fn simulate_particle2(grid: &mut HashGrid<Item>, max_y: i32) -> bool {
    let mut point = ORIGIN;
    // Sand keeps moving as long as it is able to do so,
    // at each step trying to move down,
    // then down-left,
    // then down-right.
    // If all three possible destinations are blocked, the unit of sand comes to rest
    while let Some(next_point) = next_move(point).iter().find(|candidate| {
        // We pretend there's a floor at max_y + 2.
        if candidate.y == max_y {
            return false;
        }

        matches!(grid.get(candidate).unwrap_or(&Item::Air), Item::Air)
    }) {
        point = *next_point;
    }

    grid.update(point, Item::Sand);

    point == ORIGIN
}

fn next_move(point: Point) -> [Point; 3] {
    [
        point + Point::new(0, 1),  // down
        point + Point::new(-1, 1), // down-left
        point + Point::new(1, 1),  // down-right
    ]
}

#[cfg(test)]
mod tests {
    use super::*;
    const INPUT: &str = "Sensor at x=2, y=18: closest beacon is at x=-2, y=15
Sensor at x=9, y=16: closest beacon is at x=10, y=16
Sensor at x=13, y=2: closest beacon is at x=15, y=3
Sensor at x=12, y=14: closest beacon is at x=10, y=16
Sensor at x=10, y=20: closest beacon is at x=10, y=16
Sensor at x=14, y=17: closest beacon is at x=10, y=16
Sensor at x=8, y=7: closest beacon is at x=2, y=10
Sensor at x=2, y=0: closest beacon is at x=2, y=10
Sensor at x=0, y=11: closest beacon is at x=2, y=10
Sensor at x=20, y=14: closest beacon is at x=25, y=17
Sensor at x=17, y=20: closest beacon is at x=21, y=22
Sensor at x=16, y=7: closest beacon is at x=15, y=3
Sensor at x=14, y=3: closest beacon is at x=15, y=3
Sensor at x=20, y=1: closest beacon is at x=15, y=3";

    #[test]
    fn p1() {
        assert_eq!(24, part1(&parse(INPUT)));
    }

    #[test]
    fn p2() {
        assert_eq!(
            31722,
            part2(&parse(util::aoc::input::read_all(14).unwrap().as_str()))
        );
    }
}
