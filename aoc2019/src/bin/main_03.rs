use failure::{bail, format_err, Error};
use itertools::Itertools;
use std::collections::hash_set::HashSet;
use std::collections::HashMap;
use std::str::FromStr;
use util::aoc::*;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let wires: Vec<Wire> = input::read(3)?
        .iter()
        .map(|definition| Wire::parse(definition).unwrap())
        .collect();

    result("Part 1", || part1(&wires[0], &wires[1]).unwrap());
    result("Part 2", || part2(&wires[0], &wires[1]).unwrap());

    Ok(())
}

fn part1(wire_one: &Wire, wire_two: &Wire) -> Result<i32, Error> {
    wire_one
        .intersections(wire_two)
        .into_iter()
        .map(|(point, _)| point)
        .filter(|point| point != &Point::zero())
        .map(|point| point.manhattan_distance(&Point::zero()))
        .min()
        .ok_or_else(|| format_err!("Failed to find intersection"))
}

fn part2(wire_one: &Wire, wire_two: &Wire) -> Result<i32, Error> {
    let mut grid_one = HashMap::new();
    let mut grid_two = HashMap::new();

    grid_walk(&mut grid_one, &wire_one);
    grid_walk(&mut grid_two, &wire_two);

    grid_one
        .iter()
        .filter_map(|(point, one_steps)| grid_two.get(point).map(|steps| steps + one_steps))
        .min()
        .ok_or_else(|| format_err!("No intersections"))
}

fn grid_walk(grid: &mut HashMap<Point, i32>, wire: &Wire) {
    let mut last_point = Point::zero();
    let mut steps = 0;
    for direction in &wire.directions {
        for point in direction.point_iter(last_point) {
            last_point = point;
            steps += 1;
            grid.entry(point).or_insert(steps);
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
struct Wire {
    directions: Vec<Direction>,
    segments: Vec<LineSegment>,
}

impl Wire {
    pub fn parse(line_definition: &str) -> Result<Wire, Error> {
        let mut start = Point::zero();
        let mut segments = Vec::new();
        let directions = Self::parse_directions(line_definition)?;
        for direction in &directions {
            let end = match direction {
                Direction::Up(y) => Point::new(start.x, start.y + y),
                Direction::Down(y) => Point::new(start.x, start.y - y),
                Direction::Left(x) => Point::new(start.x - x, start.y),
                Direction::Right(x) => Point::new(start.x + x, start.y),
            };
            segments.push(LineSegment::new(start, end));
            start = end;
        }

        Ok(Wire {
            directions,
            segments,
        })
    }

    pub fn intersections(&self, other: &Wire) -> Vec<(Point, i32)> {
        let mut intersections_with_steps = Vec::new();

        let mut first_steps = 0;
        let mut second_steps = 0;
        for first in &self.segments {
            first_steps += first.len();
            for second in &other.segments {
                if let Some(point) = first.intersection(second) {
                    let base_steps = first_steps + second_steps;
                    let first_partial = LineSegment::new(first.start, point).len();
                    let second_partial = LineSegment::new(second.start, point).len();
                    let steps = base_steps + first_partial + second_partial;

                    intersections_with_steps.push((point, steps));
                }

                second_steps += second.len();
            }
        }

        intersections_with_steps
    }

    fn parse_directions(line_definition: &str) -> Result<Vec<Direction>, Error> {
        line_definition
            .split(",")
            .filter(|dir| dir.len() > 1)
            .map(|dir| match dir.chars().next() {
                Some('U') => Ok(Direction::Up(dir[1..].parse::<i32>()?)),
                Some('D') => Ok(Direction::Down(dir[1..].parse::<i32>()?)),
                Some('L') => Ok(Direction::Left(dir[1..].parse::<i32>()?)),
                Some('R') => Ok(Direction::Right(dir[1..].parse::<i32>()?)),
                _ => Err(format_err!("Unknown direction: {}", dir)),
            })
            .collect::<Result<Vec<Direction>, Error>>()
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct LineSegment {
    start: Point,
    end: Point,
}

impl LineSegment {
    pub fn new(start: Point, end: Point) -> LineSegment {
        LineSegment { start, end }
    }

    pub fn len(&self) -> i32 {
        // As these segments are always horizontal or vertical, we can be very lazy about this.
        let x_len = (self.start.x - self.end.x).abs();
        let y_len = (self.start.y - self.start.y).abs();

        std::cmp::max(x_len, y_len)
    }

    pub fn intersection(&self, other_segment: &LineSegment) -> Option<Point> {
        match self.relate(other_segment) {
            LineRelation::DivergentIntersecting(point) => Some(point),
            _ => None,
        }
    }

    /// Borrowed from line_intersection crate, and adapted for this integer-only AoC world.
    pub fn relate(&self, other: &LineSegment) -> LineRelation {
        // see https://stackoverflow.com/a/565282
        let p = self.start;
        let q = other.start;
        let r = self.end - self.start;
        let s = other.end - other.start;

        let r_cross_s = Self::cross(&r, &s);
        let q_minus_p = q - p;
        let q_minus_p_cross_r = Self::cross(&q_minus_p, &r);

        // are the lines are parallel?
        if r_cross_s == 0f32 {
            // are the lines collinear?
            if q_minus_p_cross_r == 0f32 {
                // the lines are collinear
                LineRelation::Collinear
            } else {
                // the lines are parallel but not collinear
                LineRelation::Parallel
            }
        } else {
            // the lines are not parallel
            let t = Self::cross_div(&q_minus_p, &s, r_cross_s);
            let u = Self::cross_div(&q_minus_p, &r, r_cross_s);

            // are the intersection coordinates both in range?
            let t_in_range = 0f32 <= t && t <= 1f32;
            let u_in_range = 0f32 <= u && u <= 1f32;

            if t_in_range && u_in_range {
                // there is an intersection
                LineRelation::DivergentIntersecting(Self::t_coord_to_point(&p, &r, t))
            } else {
                // there is no intersection
                LineRelation::DivergentDisjoint
            }
        }
    }

    fn cross(a: &Point, b: &Point) -> f32 {
        (a.x * b.y - a.y * b.x) as f32
    }

    fn cross_div(a: &Point, b: &Point, r_cross_s: f32) -> f32 {
        let x = b.x as f32 / r_cross_s;
        let y = b.y as f32 / r_cross_s;

        a.x as f32 * y - a.y as f32 * x
    }

    fn t_coord_to_point(p: &Point, r: &Point, t: f32) -> Point {
        let t_x = t * r.x as f32;
        let t_y = t * r.y as f32;
        Point::new(p.x + t_x as i32, p.y + t_y as i32)
    }
}

/// The relationship between two line segments.
#[derive(Debug, PartialEq)]
pub enum LineRelation {
    /// The line intervals are not parallel (or anti-parallel), and "meet" each other at exactly
    /// one point.
    DivergentIntersecting(Point),
    /// The line intervals are not parallel (or anti-parallel), and do not intersect; they "miss"
    /// each other.
    DivergentDisjoint,
    /// The line intervals lie on the same line. They may or may not overlap, and this intersection
    /// is possibly infinite.
    Collinear,
    /// The line intervals are parallel or anti-parallel.
    Parallel,
}

#[derive(Clone, Debug, Eq, PartialEq)]
enum Direction {
    Up(i32),
    Down(i32),
    Left(i32),
    Right(i32),
}

impl Direction {
    pub fn point_iter(&self, start: Point) -> DirectionIterator {
        DirectionIterator {
            point: start,
            steps: self.steps(),
            direction: self.clone(),
        }
    }

    pub fn steps(&self) -> i32 {
        match *self {
            Direction::Up(steps) => steps,
            Direction::Down(steps) => steps,
            Direction::Left(steps) => steps,
            Direction::Right(steps) => steps,
        }
    }
}

struct DirectionIterator {
    direction: Direction,
    steps: i32,
    point: Point,
}

impl Iterator for DirectionIterator {
    type Item = Point;

    fn next(&mut self) -> Option<Self::Item> {
        if self.steps == 0 {
            return None;
        }

        self.steps -= 1;

        match self.direction {
            Direction::Up(steps) => self.point = self.point + Point::new(0, 1),
            Direction::Down(steps) => self.point = self.point + Point::new(0, -1),
            Direction::Left(steps) => self.point = self.point + Point::new(-1, 0),
            Direction::Right(steps) => self.point = self.point + Point::new(1, 0),
        }

        Some(self.point.clone())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_directions() {
        use super::Direction::*;

        assert_eq!(
            Wire::parse_directions("R75,D30,R83,U83,L12,D49").unwrap(),
            vec![Right(75), Down(30), Right(83), Up(83), Left(12), Down(49)]
        )
    }

    #[test]
    fn test_steps() {
        let segment = LineSegment {
            start: Point { x: 155, y: -1024 },
            end: Point { x: 238, y: -1024 },
        };

        assert_eq!(segment.len(), 83)
    }

    #[test]
    fn test_segment_intersection() {
        let segment_one = LineSegment::new(Point::new(0, 0), Point::new(0, 5));
        let segment_two = LineSegment::new(Point::new(-5, 2), Point::new(5, 2));

        assert_eq!(
            segment_one.intersection(&segment_two),
            Some(Point::new(0, 2))
        )
    }

    #[test]
    fn test_example_program() {
        let wire_one = dbg!(Wire::parse("R75,D30,R83,U83,L12,D49,R71,U7,L72").unwrap());
        let wire_two = dbg!(Wire::parse("U62,R66,U55,R34,D71,R55,D58,R83").unwrap());

        assert_eq!(part1(&wire_one, &wire_two).unwrap(), 159);
        assert_eq!(part2(&wire_one, &wire_two).unwrap(), 610);
    }

    #[test]
    fn test_example_program2() {
        let wire_one = dbg!(Wire::parse("R98,U47,R26,D63,R33,U87,L62,D20,R33,U53,R51").unwrap());
        let wire_two = dbg!(Wire::parse("U98,R91,D20,R16,D67,R40,U7,R15,U6,R7").unwrap());

        assert_eq!(part1(&wire_one, &wire_two).unwrap(), 135);
        assert_eq!(part1(&wire_one, &wire_two).unwrap(), 410);
    }
}
