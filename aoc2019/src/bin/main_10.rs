use std::borrow::Borrow;
use std::collections::hash_set::HashSet;
use std::collections::{HashMap, VecDeque};
use std::fmt::Write;
use std::str::FromStr;

use failure::_core::fmt::Formatter;
use failure::{bail, format_err, Error};
use itertools::Itertools;
use std::f32;
use util::aoc::*;

use aoc2019::intcode::*;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let lines: Vec<String> = input::read(10)?;
    let grid = Grid::build_grid(lines);

    println!("{}", grid);

    let (best, _) = result("Part 1", || part1(&grid));
    result("Part 2", || part2(&grid, best));

    Ok(())
}

fn part1(grid: &Grid) -> (Point, usize) {
    let (best, all_visible) = grid
        .asteroids()
        .into_iter()
        .map(|point| (point, visible_asteroids(grid, point)))
        .max_by_key(|(point, all_visible)| all_visible.len())
        .unwrap();

    (best, all_visible.len())
}

fn part2(grid: &Grid, best: Point) -> (Option<Point>, Option<i32>) {
    let bet_point = vaporization_order(best, &grid.asteroids());
    let x_y = bet_point.map(|p| p.x * 100 + p.y);

    (bet_point, x_y)
}

fn visible_asteroids(grid: &Grid, start: Point) -> HashSet<Point> {
    let mut points = HashSet::new();
    for edge_point in grid.asteroids().iter() {
        let mut aligned = Vec::new();
        for asteroid in grid.asteroids() {
            if asteroid == start {
                continue;
            }

            // Determine if c falls along the line drawn from a to b.
            let a = start;
            let b = edge_point;
            let c = asteroid;

            let crossproduct =
                (c.y - a.y) as f32 * (b.x - a.x) as f32 - (c.x - a.x) as f32 * (b.y - a.y) as f32;
            if crossproduct.abs() > 1e-6 {
                continue;
            }

            let dotproduct =
                (c.x - a.x) as f32 * (b.x - a.x) as f32 + (c.y - a.y) as f32 * (b.y - a.y) as f32;
            if dotproduct < 0.0 {
                continue;
            }

            let squaredlengthba =
                (b.x - a.x) as f32 * (b.x - a.x) as f32 + (b.y - a.y) as f32 * (b.y - a.y) as f32;
            if dotproduct > squaredlengthba {
                continue;
            }

            aligned.push(asteroid);
        }

        if let Some(closest) = aligned
            .iter()
            .min_by_key(|asteroid| start.distance(asteroid))
        {
            points.insert(*closest);
        }
    }

    points
}

fn vaporization_order(station: Point, all_asteroids: &[Point]) -> Option<Point> {
    let mut angle_to_asteroids: HashMap<i32, Vec<(Point, i32)>> = HashMap::new();
    let mut total = 0;
    let mut start = 0;

    // Sort points by angle and then distance.
    all_asteroids
        .into_iter()
        .filter(|point| **point != station)
        .for_each(|point| {
            let angle = ((180.0 / f32::consts::PI
                * ((point.x - station.x) as f32).atan2(((point.y - station.y) as f32)))
                * 1000.0) as i32;
            if 180000 - angle < 180000 - start {
                start = angle
            }

            let distance = point.distance(&station);
            println!("{:?}, angle={}, distance={}", point, angle, distance);
            let points = angle_to_asteroids
                .entry(angle)
                .or_insert_with(|| Vec::new());
            points.push((*point, distance));
            total += 1;
        });

    angle_to_asteroids.values_mut().for_each(|asteroids| {
        asteroids.sort_by_key(|(point, distance)| *distance);
        // Make it efficient to pop by reversing the list.
        asteroids.reverse();
    });

    let sorted_angles = angle_to_asteroids.keys().cloned().sorted_by_key(|a| -1 * a);
    let mut index = sorted_angles
        .iter()
        .position(|angle| *angle == start)
        .unwrap();
    let mut order = Vec::new();
    loop {
        let angle = sorted_angles[index];
        println!("Aiming: {}", angle as f32 / 1000.0);
        let asteroids = angle_to_asteroids.get_mut(&angle).unwrap();
        println!("  Asteroids: {:?}", asteroids);

        if let Some(asteroid) = asteroids.pop() {
            println!("  Vaporizing: {:?}", asteroid);
            order.push(asteroid.0);
            total -= 1;
        }

        if order.len() == 200 {
            return Some(order[199]);
        }

        if total == 0 {
            break
        }

        index = (index + 1) % sorted_angles.len();
    }

    None
}

struct Grid {
    grid: HashMap<Point, bool>,
    points: Vec<Point>,
    x_width: usize,
    y_width: usize,
}

impl Grid {
    fn build_grid(map: Vec<String>) -> Grid {
        let y_width = map.len();
        let x_width = map[0].len();

        let mut grid = HashMap::new();
        let mut all_points = Vec::new();
        for y in 0..y_width {
            for x in 0..x_width {
                let asteroid = map[y].chars().nth(x).unwrap() == '#';
                let point = Point::new(x as i32, y as i32);
                grid.insert(point.clone(), asteroid);
                all_points.push(point)
            }
        }

        Grid {
            grid,
            points: all_points,
            x_width,
            y_width,
        }
    }

    fn get(&self, point: &Point) -> bool {
        *self.grid.get(point).unwrap()
    }

    fn asteroids(&self) -> Vec<Point> {
        self.points
            .iter()
            .cloned()
            .filter(|point| *self.grid.get(point).unwrap())
            .collect()
    }

    fn edge_points(&self) -> Vec<Point> {
        let mut points = Vec::new();
        for x in 0..self.x_width {
            points.push(Point::new(x as i32, 0));
            points.push(Point::new(x as i32, self.y_width as i32));
        }
        for y in 0..self.y_width {
            points.push(Point::new(0, y as i32));
            points.push(Point::new(self.x_width as i32, y as i32));
        }

        points
    }

    fn print_visible(&self, start: Point, visible: &HashSet<Point>) {
        for y in 0..self.y_width {
            for x in 0..self.x_width {
                let point = Point::new(x as i32, y as i32);
                let mut output = if *self.grid.get(&point).unwrap() {
                    '#'
                } else {
                    '.'
                };

                if visible.contains(&point) {
                    output = 'X'
                }

                if start == point {
                    output = 'O'
                }

                print!("{}", output);
            }

            println!()
        }
    }
}

impl std::fmt::Display for Grid {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), std::fmt::Error> {
        for y in 0..self.y_width {
            for x in 0..self.x_width {
                let point = Point::new(x as i32, y as i32);
                let output = if *self.grid.get(&point).unwrap() {
                    '#'
                } else {
                    '.'
                };

                f.write_char(output);
            }

            f.write_char('\n');
        }

        Ok(())
    }
}
