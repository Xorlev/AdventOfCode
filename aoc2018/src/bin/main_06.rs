use failure::*;
use itertools::Itertools;
use std::cmp::Ordering;
use std::collections::HashMap;
use util::aoc::top_k::TopK;
use util::aoc::*;

fn main() -> Result<(), Box<std::error::Error>> {
    let lines: Vec<String> = input::read(6)?;
    let points = parse(&lines)?;
    result("Part 1", || part1(&points));
    result("Part 2", || part2(&points));

    Ok(())
}

fn parse(lines: &Vec<String>) -> Result<Vec<Point>, Error> {
    lines
        .iter()
        .map(|line| {
            let pieces: Vec<&str> = line.split(", ").collect();
            let x = pieces
                .get(0)
                .ok_or(format_err!("Invalid coordinate."))?
                .parse::<i32>()?;
            let y = pieces
                .get(1)
                .ok_or(format_err!("Invalid coordinate."))?
                .parse::<i32>()?;

            Ok(Point::new(x, y))
        })
        .collect()
}

fn part1(reference_points: &Vec<Point>) -> u32 {
    // Find the bounds
    let (x_min, x_max) = reference_points
        .iter()
        .map(|p| p.x)
        .minmax()
        .into_option()
        .unwrap();
    let (y_min, y_max) = reference_points
        .iter()
        .map(|p| p.y)
        .minmax()
        .into_option()
        .unwrap();

    let mut area: HashMap<Point, u32> = HashMap::new();
    for x in x_min..x_max {
        for y in y_min..y_max {
            // Find closest point
            let point = Point::new(x, y);
            if let Some(closest_point) = closest_to(&point, reference_points) {
                let count = area.entry(closest_point.clone()).or_default();
                *count += 1;
            }
        }
    }

    // Find points on the edge. These are 'infinite'.
    for x in x_min..x_max {
        for y in [y_min, y_max].iter() {
            // Find closest point
            let point = Point::new(x, *y);
            if let Some(closest_point) = closest_to(&point, reference_points) {
                area.remove(&closest_point);
            }
        }
    }
    for x in [x_min, x_max].iter() {
        for y in y_min..y_max {
            // Find closest point
            let point = Point::new(*x, y);
            if let Some(closest_point) = closest_to(&point, reference_points) {
                area.remove(&closest_point);
            }
        }
    }

    *area.iter().max_by_key(|(_, &area)| area).unwrap().1
}

fn part2(reference_points: &Vec<Point>) -> u32 {
    // Find the bounds
    let (x_min, x_max) = reference_points
        .iter()
        .map(|p| p.x)
        .minmax()
        .into_option()
        .unwrap();
    let (y_min, y_max) = reference_points
        .iter()
        .map(|p| p.y)
        .minmax()
        .into_option()
        .unwrap();

    let mut points_in_region = 0;
    for x in x_min..x_max {
        for y in y_min..y_max {
            let point = Point::new(x, y);

            // Add up the distances.
            let summed_distance: i32 = reference_points
                .iter()
                .map(|p| {
                    ComparablePoint {
                        point: &point,
                        reference: p,
                    }
                    .manhattan_distance()
                })
                .sum();

            if summed_distance < 10_000 {
                points_in_region += 1
            }
        }
    }

    points_in_region
}

fn closest_to<'a, 'b>(point: &'a Point, reference_points: &'b Vec<Point>) -> Option<&'b Point> {
    let top = reference_points
        .iter()
        .map(|p| ComparablePoint {
            point,
            reference: p,
        })
        .topk(2);

    match top.as_slice() {
        [first, second] if first.manhattan_distance() < second.manhattan_distance() => {
            Some(first.reference)
        }
        _ => None,
    }
}

#[derive(Debug, Eq, PartialEq)]
struct ComparablePoint<'a, 'b> {
    point: &'b Point,
    reference: &'a Point,
}

impl<'a, 'b> ComparablePoint<'a, 'b> {
    fn manhattan_distance(&self) -> i32 {
        self.point.manhattan_distance(self.reference)
    }
}

impl<'a, 'b> Ord for ComparablePoint<'a, 'b> {
    fn cmp(&self, other: &ComparablePoint) -> Ordering {
        let lhs_dist = self.manhattan_distance();
        let rhs_dist = other.manhattan_distance();

        lhs_dist.cmp(&rhs_dist)
    }
}

impl<'a, 'b> PartialOrd for ComparablePoint<'a, 'b> {
    fn partial_cmp(&self, other: &ComparablePoint) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}
