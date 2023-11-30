use crate::aoc::Point;
use std::fmt::Debug;

#[derive(Debug, Clone)]
pub struct Grid<T> {
    states: Vec<Vec<T>>,
}

impl<T: Debug> Grid<T> {
    pub fn new(states: Vec<Vec<T>>) -> Grid<T> {
        Grid { states }
    }

    pub fn lookup(&self, point: &Point) -> Option<&T> {
        if point.x >= 0 && point.y >= 0 {
            self.states
                .get(point.y as usize)
                .and_then(|row| row.get(point.x as usize))
        } else {
            None
        }
    }

    pub fn update(&mut self, point: &Point, state: T) {
        self.states[point.y as usize][point.x as usize] = state;
    }

    pub fn update_fn(&mut self, point: &Point, state_fn: fn(&T) -> T) -> &T {
        let new_state = state_fn(&self.states[point.y as usize][point.x as usize]);
        self.states[point.y as usize][point.x as usize] = new_state;
        &self.states[point.y as usize][point.x as usize]
    }

    pub fn point_iterator(&self) -> PointIterator<T> {
        PointIterator {
            grid: self,
            x: 0,
            y: 0,
        }
    }

    pub fn perimeter_iterator(&self) -> PerimeterIterator<T> {
        PerimeterIterator {
            grid: self,
            x: 0,
            y: 0,
            direction: Direction::Right,
        }
    }

    pub fn print_grid(&self) {
        for y in 0..self.y_len() {
            for x in 0..self.x_len() {
                print!("{:?}", self.states[y][x]);
            }
            println!();
        }
    }

    pub fn x_len(&self) -> usize {
        self.states[0].len()
    }

    pub fn y_len(&self) -> usize {
        self.states.len()
    }
}

pub struct PointIterator<'a, T> {
    grid: &'a Grid<T>,
    x: usize,
    y: usize,
}

impl<'a, T: Debug> Iterator for PointIterator<'a, T> {
    type Item = (Point, &'a T);

    fn next(&mut self) -> Option<Self::Item> {
        let point = Point::new(self.x as i32, self.y as i32);
        if let Some(state) = self.grid.lookup(&point) {
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

#[derive(Debug)]
pub enum Direction {
    Right,
    Down,
    Left,
    Up,
}

impl Direction {
    pub fn delta(&self) -> Point {
        match self {
            Direction::Right => Point::new(1, 0),
            Direction::Down => Point::new(0, 1),
            Direction::Left => Point::new(-1, 0),
            Direction::Up => Point::new(0, -1),
        }
    }
    pub fn apply(&self, point: Point) -> Point {
        point + self.delta()
    }
}

pub struct PerimeterIterator<'a, T> {
    grid: &'a Grid<T>,
    x: usize,
    y: usize,
    direction: Direction,
}

impl<'a, T: Debug> Iterator for PerimeterIterator<'a, T> {
    type Item = (Point, &'a T);

    fn next(&mut self) -> Option<Self::Item> {
        let point = Point::new(self.x as i32, self.y as i32);

        // (0,0) -> (X,0)
        // (X,0) -> (X,Y)
        // (X,Y) -> (0,Y)
        // (0,Y) -> (0,1) (to avoid a repeat)

        if self.x == self.grid.x_len() - 1 && self.y == 0 {
            self.direction = Direction::Down;
        }
        if self.x == self.grid.x_len() - 1 && self.y == self.grid.y_len() - 1 {
            self.direction = Direction::Left;
        }
        if self.x == 0 && self.y == self.grid.y_len() - 1 {
            self.direction = Direction::Up;
        }
        if matches!(self.direction, Direction::Up) && self.y == 0 {
            return None;
        }

        let next_point = self.direction.apply(point);
        self.x = next_point.x as usize;
        self.y = next_point.y as usize;

        self.grid.lookup(&point).map(|value| (point, value))
    }
}
