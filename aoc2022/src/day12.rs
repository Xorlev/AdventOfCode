use itertools::Itertools;
use util::aoc::astar::AStarResult;
use util::aoc::grid::Grid;
use util::aoc::{astar, Point};

#[derive(Copy, Clone, Debug)]
enum GridSquare {
    Start,
    End,
    Height(i32),
}

impl GridSquare {
    fn height(&self) -> i32 {
        match self {
            GridSquare::Start => 'a' as i32,
            GridSquare::End => 'z' as i32,
            GridSquare::Height(height) => *height,
        }
    }
}

#[aoc_generator(day12)]
fn parse(input: &str) -> Grid<GridSquare> {
    Grid::new(
        input
            .lines()
            .map(|line| {
                line.chars()
                    .map(|c| match c {
                        'S' => GridSquare::Start,
                        'E' => GridSquare::End,
                        c => GridSquare::Height(c as i32),
                    })
                    .collect()
            })
            .collect(),
    )
}

#[aoc(day12, part1)]
fn part1(input: &Grid<GridSquare>) -> usize {
    let (start, _) = input
        .point_iterator()
        .find(|(_, &height)| matches!(height, GridSquare::Start))
        .unwrap();
    let (end, _) = input
        .point_iterator()
        .find(|(_, &height)| matches!(height, GridSquare::End))
        .unwrap();

    compute_path(input, &start, &end).unwrap_or(0)
}

#[aoc(day12, part2)]
fn part2(input: &Grid<GridSquare>) -> usize {
    let (end, _) = input
        .point_iterator()
        .find(|(_, &height)| matches!(height, GridSquare::End))
        .unwrap();

    input
        .point_iterator()
        .filter(|(_, &height)| height.height() == GridSquare::Start.height())
        .filter_map(|(start, _)| compute_path(input, &start, &end))
        .min()
        .unwrap_or(0)
}

fn compute_path(input: &Grid<GridSquare>, start: &Point, end: &Point) -> Option<usize> {
    let path = astar::search(
        start,
        |candidate| {
            // This essentially makes it equivalent to Dijkstra's shortest path.
            if candidate.manhattan_distance(end) == 0 {
                0
            } else {
                1
            }
        },
        |_, _| 1,
        |current| {
            current
                .neighbors4()
                .into_iter()
                .filter(|destination| {
                    let current_height = input.lookup(current).unwrap();
                    if let Some(destination_height) = input.lookup(destination) {
                        // Allow points that are one higher or infinitely lower
                        destination_height.height() - current_height.height() <= 1
                    } else {
                        false
                    }
                })
                .collect_vec()
        },
    );

    if let AStarResult::Success(path, _) = path {
        Some(path.len() - 1)
    } else {
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    const INPUT: &str = "Sabqponm
abcryxxl
accszExk
acctuvwj
abdefghi";

    #[test]
    fn p1() {
        // assert_eq!(31, part1(&parse(INPUT)));
        assert_eq!(
            447,
            part1(&parse(util::aoc::input::read_all(12).unwrap().as_str()))
        );
    }

    #[test]
    fn p2() {
        assert_eq!(29, part2(&parse(INPUT)));
    }
}
