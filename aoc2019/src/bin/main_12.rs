use failure::{format_err, Error};
use itertools::{zip, Itertools};
use lazy_static::*;
use num::integer::lcm;
use regex::Regex;
use util::aoc::*;

type Vector = [i32; 3];

lazy_static! {
    static ref RE: Regex =
        Regex::new("<x=([\\s\\-0-9]+), y=([\\s\\-0-9]+), z=([\\s\\-0-9]+)>").unwrap();
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let positions: Vec<Vector> = input::read(12)?
        .iter()
        .map(|f| parse_vector(f))
        .collect::<Result<Vec<Vector>, _>>()?;
    let moons = positions
        .into_iter()
        .map(|t| Moon::new(t))
        .collect::<Vec<Moon>>();

    result("Part 1", || part1(moons.clone()));
    result("Part 2", || part2(moons.clone()));

    Ok(())
}

fn part1(mut moons: Vec<Moon>) -> Result<i32, Error> {
    println!("Step 0: =========================");
    moons.iter().for_each(|m| println!("{:?}", m));

    let combinations = (0..moons.len())
        .combinations(2)
        .collect::<Vec<Vec<usize>>>();
    for _ in 1..=1000 {
        // Calculate velocities.
        combinations.iter().for_each(|m| {
            let (m1, m2) = moons.get_two_mut(m[0], m[1]);
            m1.apply_gravity(m2)
        });

        // Apply new velocities.
        moons.iter_mut().for_each(Moon::step);
    }

    Ok(moons.iter().map(Moon::total_energy).sum())
}

fn part2(moons: Vec<Moon>) -> Result<usize, Error> {
    let initial_state = moons.clone();
    println!("Step 0: =========================");
    moons.iter().for_each(|m| println!("{:?}", m));

    let combinations = (0..moons.len())
        .combinations(2)
        .collect::<Vec<Vec<usize>>>();
    let mut stops = [0; 3];
    for axis in 0..3 {
        let mut moons = moons.clone();
        let mut steps = 0;
        loop {
            combinations.iter().for_each(|m| {
                let (m1, m2) = moons.get_two_mut(m[0], m[1]);
                m1.apply_axis(m2, axis);
            });

            // Apply new velocities.
            moons.iter_mut().for_each(Moon::step);
            steps += 1;

            // For each moon, check if we've returned to the initial state.
            if zip(&initial_state, &moons).all(|(m1, m2)| {
                m1.position[axis] == m2.position[axis] && m1.velocity[axis] == m2.velocity[axis]
            }) {
                stops[axis];
                break;
            }
        }
    }

    Ok(lcm(lcm(stops[0], stops[1]), lcm(stops[1], stops[2])))
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct Moon {
    position: Vector,
    velocity: Vector,
}

impl Moon {
    fn new(position: Vector) -> Moon {
        Moon {
            position,
            velocity: [0; 3],
        }
    }

    fn potential_energy(&self) -> i32 {
        (0..3).map(|axis| self.position[axis].abs()).sum()
    }

    fn kinetic_energy(&self) -> i32 {
        (0..3).map(|axis| self.velocity[axis].abs()).sum()
    }

    fn total_energy(&self) -> i32 {
        self.potential_energy() * self.kinetic_energy()
    }

    fn step(&mut self) {
        (0..3).for_each(|axis| self.position[axis] = self.position[axis] + self.velocity[axis]);
    }

    fn apply_gravity(&mut self, other: &mut Moon) {
        // For each axis, add/subtract one unit of velocity toward the other moon.
        self.apply_axis(other, 0);
        self.apply_axis(other, 1);
        self.apply_axis(other, 2);
    }

    fn apply_axis(&mut self, other: &mut Moon, axis: usize) {
        if self.position[axis] > other.position[axis] {
            self.velocity[axis] -= 1;
            other.velocity[axis] += 1;
        } else if self.position[axis] < other.position[axis] {
            self.velocity[axis] += 1;
            other.velocity[axis] -= 1;
        }
    }
}

fn parse_vector(s: &str) -> Result<Vector, Error> {
    if let Some(captures) = RE.captures(s) {
        Ok([
            captures[1].trim().parse()?,
            captures[2].trim().parse()?,
            captures[3].trim().parse()?,
        ])
    } else {
        Err(format_err!("Didn't match input: {}", s))
    }
}
