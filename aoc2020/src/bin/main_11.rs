use failure::bail;
use itertools::Itertools;
use std::collections::HashMap;
use std::iter::FromIterator;
use std::str::FromStr;
use util::aoc::*;

fn main() -> AocResult<()> {
    let grid: Grid = input::read(11)?.into_iter().collect();

    result("Part 1", || part1(&grid));
    result("Part 2", || part2(&grid));

    Ok(())
}

fn part1(grid: &Grid) -> usize {
    let mut grid = grid.clone();
    while grid.update_step_p1() {
        // grid.print_grid();
    }

    grid.occupied_seats()
}

fn part2(grid: &Grid) -> usize {
    let mut grid = grid.clone();
    while grid.update_step_p2() {
        // grid.print_grid();
    }

    grid.occupied_seats()
}

#[derive(Debug, Clone)]
struct Grid {
    states: Vec<Vec<State>>,
}

impl Grid {
    pub(crate) fn occupied_seats(&self) -> usize {
        self.point_iterator()
            .filter(|(_, state)| state.is_occupied())
            .count()
    }
}

impl Grid {
    fn lookup_point(&self, point: &Point) -> Option<State> {
        if point.x >= 0 && point.y >= 0 {
            self.states
                .get(point.y as usize)
                .and_then(|row| row.get(point.x as usize).cloned())
        } else {
            None
        }
    }

    fn update_point(&mut self, point: &Point, state: State) {
        self.states[point.y as usize][point.x as usize] = state;
    }

    fn point_iterator(&self) -> PointIterator {
        PointIterator {
            grid: self,
            x: 0,
            y: 0,
        }
    }

    fn update_step_p1(&mut self) -> bool {
        let mut new_states: HashMap<Point, State> = HashMap::new();

        self.point_iterator().for_each(|(point, state)| {
            let occupied_seats = point
                .neighbors8()
                .iter()
                .filter_map(|p| self.lookup_point(p))
                .filter(|s| s.is_occupied())
                .count();

            match state {
                State::Empty => {
                    if occupied_seats == 0 {
                        new_states.insert(point, State::Occupied);
                    }
                }
                State::Floor => {}
                State::Occupied => {
                    if occupied_seats >= 4 {
                        new_states.insert(point, State::Empty);
                    }
                }
            }
        });

        new_states.iter().for_each(|(point, new_state)| {
            self.update_point(point, *new_state);
        });

        !new_states.is_empty()
    }

    fn update_step_p2(&mut self) -> bool {
        let mut new_states: HashMap<Point, State> = HashMap::new();

        self.point_iterator().for_each(|(point, state)| {
            let occupied_seats = Point::zero()
                .neighbors8()
                .iter()
                .filter(|&unit_direction| self.explore(&point, unit_direction))
                .count();

            match state {
                State::Empty => {
                    if occupied_seats == 0 {
                        new_states.insert(point, State::Occupied);
                    }
                }
                State::Floor => {}
                State::Occupied => {
                    if occupied_seats >= 5 {
                        new_states.insert(point, State::Empty);
                    }
                }
            }
        });

        new_states.iter().for_each(|(point, new_state)| {
            self.update_point(point, *new_state);
        });

        !new_states.is_empty()
    }

    fn explore(&self, reference_point: &Point, direction: &Point) -> bool {
        let mut point = *reference_point + *direction;
        while let Some(state) = self.lookup_point(&point) {
            match state {
                State::Empty => return false,
                State::Floor => point += *direction,
                State::Occupied => return true,
            }
        }

        false
    }

    fn print_grid(&self) {
        for row in &self.states {
            for column in row {
                let str = match column {
                    State::Empty => "L",
                    State::Floor => ".",
                    State::Occupied => "#",
                };

                print!("{}", str);
            }
            println!();
        }
    }
}

impl FromIterator<String> for Grid {
    fn from_iter<T: IntoIterator<Item = String>>(iter: T) -> Self {
        let states = iter
            .into_iter()
            .map(|line| {
                line.chars()
                    .map(|c| c.to_string().parse().unwrap())
                    .collect_vec()
            })
            .collect();

        Grid { states }
    }
}

struct PointIterator<'a> {
    grid: &'a Grid,
    x: usize,
    y: usize,
}

impl<'a> Iterator for PointIterator<'a> {
    type Item = (Point, State);

    fn next(&mut self) -> Option<Self::Item> {
        let point = Point::new(self.x as i32, self.y as i32);
        if let Some(state) = self.grid.lookup_point(&point) {
            if self.x == self.grid.states[0].len() - 1 {
                self.y += 1;
                self.x = 0;
            } else {
                self.x += 1;
            }
            Some((point, state))
        } else {
            None
        }
    }
}

#[derive(Debug, Copy, Clone)]
enum State {
    Empty,
    Floor,
    Occupied,
}

impl State {
    pub(crate) fn is_occupied(&self) -> bool {
        matches!(self, State::Occupied)
    }
}

impl FromStr for State {
    type Err = failure::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "." => Ok(Self::Floor),
            "#" => Ok(Self::Occupied),
            "L" => Ok(Self::Empty),
            _ => bail!("unexpected state: {}", s),
        }
    }
}
