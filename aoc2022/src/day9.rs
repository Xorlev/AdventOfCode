use itertools::Itertools;
use std::collections::HashSet;
use util::aoc::grid::Direction;
use util::aoc::{Point, SliceExt};

#[aoc_generator(day9)]
fn parse(input: &str) -> Vec<(u32, Direction)> {
    input
        .lines()
        .map(|instruction| {
            let parts = instruction.split_whitespace().collect_vec();
            (parts[1].parse().unwrap(), to_direction(parts[0]))
        })
        .collect()
}

fn to_direction(part: &str) -> Direction {
    match part {
        "U" => Direction::Up,
        "D" => Direction::Down,
        "L" => Direction::Left,
        "R" => Direction::Right,
        _ => panic!("unmatched direction: {}", part),
    }
}

#[aoc(day9, part1)]
fn part1(input: &[(u32, Direction)]) -> usize {
    simulate_rope(input, 2).len()
}

#[aoc(day9, part2)]
fn part2(input: &[(u32, Direction)]) -> usize {
    simulate_rope(input, 10).len()
}

fn simulate_rope(input: &[(u32, Direction)], rope_size: usize) -> HashSet<Point> {
    let mut knots = vec![Point::new(0, 0); rope_size];
    let mut visited_points = HashSet::new();
    let knot_count = knots.len();
    for (steps, direction) in input {
        for _ in 0..*steps {
            for i in 0..knot_count - 1 {
                let (head, tail) = knots.get_two_mut(i, i + 1);
                if i == 0 {
                    *head += direction.delta();
                }

                if head.distance(tail) as i32 > 1 {
                    std::mem::swap(tail, &mut move_tail(head, tail));
                }

                if i + 2 == knot_count {
                    visited_points.insert(*tail);
                }
            }
        }
    }
    visited_points
}

fn move_tail(head: &Point, tail: &Point) -> Point {
    tail.neighbors8()
        .into_iter()
        .min_by_key(|next_point| {
            let mut distance = head.distance(next_point) as i32;
            if next_point.x == head.x || next_point.y == head.y {
                // Prioritize same-column or same-row moves.
                distance -= 1;
            }

            distance
        })
        .unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;
    const INPUT: &str = "R 4
U 4
L 3
D 1
R 4
D 1
L 5
R 2";
    const INPUT2: &str = "R 5
U 8
L 8
D 3
R 17
D 10
L 25
U 20";

    #[test]
    fn p1() {
        assert_eq!(13, part1(&parse(INPUT)));
    }

    #[test]
    fn p2() {
        assert_eq!(1, part2(&parse(INPUT)));
        assert_eq!(36, part2(&parse(INPUT2)));
    }
}
