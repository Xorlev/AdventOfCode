use std::collections::HashMap;

use failure::bail;
use itertools::Itertools;
use lazy_static::lazy_static;
use regex::Regex;
use util::aoc::*;

fn main() -> AocResult<()> {
    let (grid, folds) = parse(input::read_all(13)?)?;

    result("Part 1", || part1(&grid, &folds));
    result("Part 2", || part2(grid.clone(), &folds));

    Ok(())
}

fn part1(foldable_grid: &HashGrid, folds: &[Fold]) -> i32 {
    fold_grid(foldable_grid, &folds[0]).grid.keys().len() as i32
}

fn part2(foldable_grid: HashGrid, folds: &[Fold]) -> i32 {
    folds
        .iter()
        .fold(foldable_grid, |grid, fold| fold_grid(&grid, fold))
        .print_grid();

    0
}

fn fold_grid(foldable_grid: &HashGrid, fold: &Fold) -> HashGrid {
    let mut new_grid = HashMap::new();
    for point in foldable_grid.point_iterator() {
        let translation = match *fold {
            Fold::X(limit) => {
                if point.x < limit {
                    Some(Point::new(2 * (point.x - limit).abs(), 0))
                } else {
                    None
                }
            }
            Fold::Y(limit) => {
                if point.y < limit {
                    Some(Point::new(0, 2 * (point.y - limit).abs()))
                } else {
                    None
                }
            }
        };

        if let Some(translation) = translation {
            let translated_point = point + translation;
            for p in vec![point, translated_point] {
                if foldable_grid.grid.contains_key(&p) {
                    new_grid.insert(point, true);
                }
            }
        }
    }

    HashGrid::new(new_grid.keys().cloned().collect())
}

#[derive(Debug, Clone)]
struct HashGrid {
    grid: HashMap<Point, bool>,
    max_x: i32,
    max_y: i32,
}

impl HashGrid {
    fn new(points: Vec<Point>) -> Self {
        let max_x = points.iter().map(|p| p.x).max().unwrap_or(0);
        let max_y = points.iter().map(|p| p.y).max().unwrap_or(0);

        HashGrid {
            grid: points.into_iter().map(|point| (point, true)).collect(),
            max_x,
            max_y,
        }
    }

    fn point_iterator(&self) -> PointIterator {
        PointIterator {
            x: 0,
            y: 0,
            max_x: self.max_x,
            max_y: self.max_y,
        }
    }

    fn print_grid(&self) {
        for y in 0..=self.max_y {
            for x in 0..=self.max_x {
                let point = Point::new(x, y);
                if self.grid.contains_key(&point) {
                    print!("#")
                } else {
                    print!(".")
                }
            }
            println!();
        }
    }
}

struct PointIterator {
    x: i32,
    y: i32,
    max_x: i32,
    max_y: i32,
}

impl Iterator for PointIterator {
    type Item = Point;

    fn next(&mut self) -> Option<Self::Item> {
        let point = Point::new(self.x as i32, self.y as i32);
        if self.x < self.max_x {
            self.x += 1;
        } else if self.y < self.max_y {
            self.x = 0;
            self.y += 1;
        } else {
            return None;
        }

        Some(point)
    }
}

#[derive(Debug, Copy, Clone)]
enum Fold {
    X(i32),
    Y(i32),
}

lazy_static! {
    static ref RE: Regex = Regex::new("fold along (x|y)=(\\d+)").unwrap();
}

fn parse(s: String) -> AocResult<(HashGrid, Vec<Fold>)> {
    let parts = s.split("\n\n").collect_vec();

    let mut points = Vec::new();
    for point in parts[0].lines() {
        match point.split(',').collect_vec().as_slice() {
            [x, y] => points.push(Point::new(x.parse()?, y.parse()?)),
            _ => bail!("Unknown input: {}", point),
        }
    }

    let mut folds = Vec::new();
    for fold in parts[1].lines() {
        if let Some(captures) = RE.captures(fold) {
            folds.push(match &captures[1] {
                "x" => Fold::X(captures[2].parse()?),
                "y" => Fold::Y(captures[2].parse()?),
                _ => bail!("Unrecognized fold: {}", fold),
            })
        }
    }

    Ok((HashGrid::new(points), folds))
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE_INPUT: &str = r#"6,10
0,14
9,10
0,3
10,4
4,11
6,0
6,12
4,1
0,13
10,12
3,4
3,0
8,4
1,10
2,14
8,10
9,0

fold along y=7
fold along x=5"#;

    #[test]
    fn part1_sample() {
        let input = parse(SAMPLE_INPUT.lines().map(|s| s.to_string()).collect());
        let (grid, folds) = parse(SAMPLE_INPUT.to_string()).unwrap();

        assert_eq!(17, part1(grid, &folds));
    }
}
