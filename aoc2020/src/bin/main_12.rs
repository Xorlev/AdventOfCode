use failure::bail;
use itertools::Itertools;
use std::collections::HashMap;
use std::iter::FromIterator;
use std::str::FromStr;
use util::aoc::*;

fn main() -> AocResult<()> {
    let actions: Vec<Action> = input::read(12)?.parse()?;

    result("Part 1", || part1(&actions));
    result("Part 2", || part2(&actions));

    Ok(())
}

fn part1(actions: &[Action]) -> i32 {
    let start = Point::zero();
    let mut ship = Ship::new();
    actions.iter().for_each(|action| ship.apply(*action));

    start.manhattan_distance(&ship.position)
}

fn part2(actions: &[Action]) -> i32 {
    let start = Point::zero();
    let mut ship = Ship::new();
    actions.iter().for_each(|action| ship.apply_p2(*action));

    start.manhattan_distance(&ship.position)
}

struct Ship {
    position: Point,
    direction: i32,
    waypoint_position: Point,
}

impl Ship {
    fn new() -> Ship {
        Ship {
            position: Point::zero(),
            direction: 90,
            waypoint_position: Point::new(10, 1),
        }
    }

    fn apply(&mut self, action: Action) {
        match action.translate_forward(self.direction) {
            Action::North(steps) => self.position += Point::new(0, 1) * steps,
            Action::South(steps) => self.position += Point::new(0, -1) * steps,
            Action::East(steps) => self.position += Point::new(1, 0) * steps,
            Action::West(steps) => self.position += Point::new(-1, 0) * steps,
            Action::Left(degrees) => {
                self.direction = (self.direction - degrees).checked_rem_euclid(360).unwrap()
            }
            Action::Right(degrees) => {
                self.direction = (self.direction + degrees).checked_rem_euclid(360).unwrap()
            }
            Action::Forward(steps) => panic!("untranslated forward action!"),
        };
        println!("A: pos: {:?}, dir: {:?}", self.position, self.direction);
    }

    fn apply_p2(&mut self, action: Action) {
        match action {
            Action::North(steps) => self.waypoint_position += Point::new(0, 1) * steps,
            Action::South(steps) => self.waypoint_position += Point::new(0, -1) * steps,
            Action::East(steps) => self.waypoint_position += Point::new(1, 0) * steps,
            Action::West(steps) => self.waypoint_position += Point::new(-1, 0) * steps,
            Action::Left(degrees) => {
                self.waypoint_position = self.waypoint_position.rotate(degrees)
            }
            Action::Right(degrees) => {
                self.waypoint_position = self.waypoint_position.rotate(-degrees)
            }
            Action::Forward(steps) => {
                for _ in 0..steps {
                    self.position += self.waypoint_position;
                }
            }
        };
    }
}

#[derive(Debug, Copy, Clone)]
enum Action {
    North(i32),
    South(i32),
    East(i32),
    West(i32),
    Left(i32),
    Right(i32),
    Forward(i32),
}

impl Action {
    fn translate_forward(self, direction: i32) -> Action {
        if let Action::Forward(steps) = self {
            match direction {
                0 => Action::North(steps),
                90 => Action::East(steps),
                180 => Action::South(steps),
                270 => Action::West(steps),
                _ => panic!("Unknown direction: {}", direction),
            }
        } else {
            self
        }
    }
}

impl FromStr for Action {
    type Err = failure::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let chars = s.chars().collect_vec();
        let value: i32 = chars[1..].iter().collect::<String>().parse()?;
        let action = match chars[0] {
            'N' => Action::North(value),
            'S' => Action::South(value),
            'E' => Action::East(value),
            'W' => Action::West(value),
            'L' => Action::Left(value),
            'R' => Action::Right(value),
            'F' => Action::Forward(value),
            _ => bail!("Bad value: {}", s),
        };

        Ok(action)
    }
}
