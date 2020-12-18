


use std::collections::HashMap;


use util::aoc::*;
use itertools::Itertools;
use failure::_core::hash::Hash;

fn main() -> AocResult<()> {
    let input: HashMap<Point3D, bool> = Itertools::flatten(input::read(17)?
        .into_iter()
        .enumerate()
        .map(|(row, line)| {
            line.chars()
                .enumerate()
                .map(|(column, char)| {
                    let active = match char {
                        '.' => false,
                        '#' => true,
                        _ => panic!("unexpected character: {}", char),
                    };

                    (Point3D::new(row as i32, column as i32, 0), active)
                })
                .collect::<Vec<_>>()
        }))
        .collect();

    let input_p2: HashMap<Point4D, bool> = input
        .iter()
        .map(|(k, &v)| (Point4D::new(k.x, k.y, k.z, 0), v))
        .collect();
    

    result("Part 1", || part1(&input));
    result("Part 2", || part2(&input_p2));

    Ok(())
}

fn part1(state: &HashMap<Point3D, bool>) -> usize {
    solve(state)
}

fn part2(state: &HashMap<Point4D, bool>) -> usize {
    solve(state)
}

fn solve<T: Cube>(state: &HashMap<T, bool>) -> usize {
    let mut state = state.clone();
    for _ in 0..6 {
        println!("active cubes: {}", state.values().filter(|&&v| v).count());

        let old_state = state.clone();

        // find cubes to disable
        old_state
            .iter()
            .filter(|(_, &active)| active)
            .filter(|(point, _)| {
                let active_neighbors = point
                    .neighbors()
                    .iter()
                    .filter(|p| old_state.get(p).cloned().unwrap_or(false))
                    .count();

                !(active_neighbors == 2 || active_neighbors == 3)
            })
            .for_each(|(p, _)| {
                state.insert(p.clone(), false);
            });

        // find cubes to activate by looking at all the neighbors of all active points
        old_state
            .iter()
            .flat_map(|(p, _)| {
                let mut vec = p.neighbors();
                vec.push(p.clone());
                vec
            })
            .unique()
            .filter(|p| !old_state.get(p).cloned().unwrap_or(false))
            .filter(|p| {
                p.neighbors()
                    .iter()
                    .filter(|p| old_state.get(p).cloned().unwrap_or(false))
                    .count()
                    == 3
            })
            .for_each(|p| {
                state.insert(p.clone(), true);
            });
    }

    state.values().filter(|&&v| v).count()
}

trait Cube: Eq + Hash + Copy + Clone + PartialEq {
    fn neighbors(&self) -> Vec<Self> where Self: Sized;
}

#[derive(Copy, Clone, Debug, Hash, Eq, PartialEq)]
pub struct Point3D {
    pub x: i32,
    pub y: i32,
    pub z: i32,
}

impl Point3D {
    pub const fn new(x: i32, y: i32, z: i32) -> Point3D {
        Point3D { x, y, z }
    }
}

impl Cube for Point3D {
    fn neighbors(&self) -> Vec<Point3D> {
        (self.x - 1..=(self.x + 1))
            .flat_map(move |x| {
                (self.y - 1..=(self.y + 1))
                    .flat_map(move |y| (self.z - 1..=(self.z + 1)).map(move |z| Point3D::new(x, y, z)))
            })
            .filter(|p| p != self)
            .collect()
    }
}

#[derive(Copy, Clone, Debug, Hash, Eq, PartialEq)]
pub struct Point4D {
    pub x: i32,
    pub y: i32,
    pub z: i32,
    pub w: i32,
}

impl Point4D {
    pub const fn new(x: i32, y: i32, z: i32, w: i32) -> Point4D {
        Point4D { x, y, z, w }
    }
}

impl Cube for Point4D {
    fn neighbors(&self) -> Vec<Point4D> {
        (self.x - 1..=(self.x + 1))
            .flat_map(move |x| {
                (self.y - 1..=(self.y + 1))
                    .flat_map(move |y| {
                        (self.z - 1..=(self.z + 1)).flat_map(move |z| {
                            (self.w - 1..=(self.w + 1)).map(move |w| {
                                Point4D::new(x, y, z, w)
                            })
                        })
                    })
            })
            .filter(|p| p != self)
            .collect()
    }
}