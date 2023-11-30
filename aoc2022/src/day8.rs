use itertools::Itertools;
use std::collections::HashSet;
use util::aoc::grid::Grid;
use util::aoc::Point;

#[aoc_generator(day8)]
fn parse(input: &str) -> Grid<u8> {
    Grid::new(
        input
            .lines()
            .map(|line| {
                line.chars()
                    .map(|c| c.to_digit(10).unwrap() as u8)
                    .collect_vec()
            })
            .collect_vec(),
    )
}

#[aoc(day8, part1)]
fn part1(input: &Grid<u8>) -> usize {
    // For each exterior point, scan the row/column.
    let perimeter = input.perimeter_iterator().collect_vec();
    let mut visible_trees: HashSet<Point> = perimeter.iter().map(|&(point, _)| point).collect();
    perimeter
        .iter()
        .flat_map(|(point, _)| {
            let delta_point = match *point {
                Point { x: _, y: 0 } => Point::new(0, 1),
                Point { x, y: _ } if x as usize == input.x_len() - 1 => Point::new(-1, 0),
                Point { x: _, y } if y as usize == input.y_len() - 1 => Point::new(0, -1),
                Point { x: 0, y: _ } => Point::new(1, 0),
                _ => panic!("unmatched"),
            };

            scan(*point, delta_point, input)
        })
        .for_each(|point| {
            visible_trees.insert(point);
        });

    // For each point, scan in each direction.
    // Ignore corners
    // If Y is 0, scan down.
    // If X is X_MAX, scan left.
    // If Y is Y_MAX, scan up.
    // If X is 0, scan right.

    visible_trees.len()
}

#[aoc(day8, part2)]
fn part2(input: &Grid<u8>) -> usize {
    // For each point, scan in each direction.
    // Ignore corners
    // If Y is 0, scan down.
    // If X is X_MAX, scan left.
    // If Y is Y_MAX, scan up.
    // If X is 0, scan right.
    input
        .point_iterator()
        .into_iter()
        .map(|(point, start_height)| {
            vec![
                Point::new(0, 1),
                Point::new(-1, 0),
                Point::new(0, -1),
                Point::new(1, 0),
            ]
            .into_iter()
            .map(|delta_point| {
                let mut view_distance = 0;
                for next_point in point.point_iterator(delta_point).skip(1) {
                    if let Some(height) = input.lookup(&next_point) {
                        if height >= start_height {
                            return view_distance + 1;
                        } else {
                            view_distance += 1
                        }
                    } else {
                        return view_distance;
                    }
                }

                1
            })
            .product::<usize>()
        })
        .max()
        .unwrap_or(0)
}

fn scan(start: Point, delta: Point, input: &Grid<u8>) -> Vec<Point> {
    let mut max_height = input.lookup(&start).unwrap();
    let mut points = vec![];
    for next_point in start.point_iterator(delta) {
        if let Some(height) = input.lookup(&next_point) {
            if height > max_height {
                max_height = height;
                points.push(next_point);
            }
        } else {
            return points;
        }
    }

    points
}

#[cfg(test)]
mod tests {
    use super::*;
    const INPUT: &str = "30373
25512
65332
33549
35390";

    #[test]
    fn p1() {
        assert_eq!(21, part1(&parse(INPUT)));
    }

    #[test]
    fn p2() {
        assert_eq!(8, part2(&parse(INPUT)));
    }
}
