use crate::aoc::Point;
use std::fmt::Debug;
use std::marker::PhantomData;

pub trait Grid<T: Debug> {
    fn lookup(&self, point: &Point) -> Option<&T>;
    fn update(&mut self, point: &Point, state: T);
    fn update_fn(&mut self, point: &Point, state_fn: fn(&T) -> T) -> &T;
    fn point_iterator(&self) -> PointIterator<T>;
    fn print_grid(&self);
    fn x_max(&self) -> usize;
    fn y_max(&self) -> usize;
    fn x_len(&self, y: i32) -> usize;
}

#[derive(Debug, Clone)]
pub struct TranslationGrid<T: Debug, G: Grid<T> + Debug> {
    grid: G,
    translate: Point,
    marker: PhantomData<T>,
}

impl<T: Debug, G: Grid<T> + Debug + Clone> TranslationGrid<T, G> {
    pub fn new(grid: &G, translate: Point) -> Self {
        TranslationGrid {
            grid: grid.clone(),
            translate,
            marker: Default::default(),
        }
    }
}

impl<T: Debug, G: Grid<T> + Debug> Grid<T> for TranslationGrid<T, G> {
    fn lookup(&self, point: &Point) -> Option<&T> {
        self.grid.lookup(&(*point + self.translate))
    }

    fn update(&mut self, point: &Point, state: T) {
        let point = &(*point + self.translate);
        self.grid.update(point, state);
    }

    fn update_fn(&mut self, point: &Point, state_fn: fn(&T) -> T) -> &T {
        let point = &(*point + self.translate);
        self.grid.update_fn(point, state_fn)
    }

    fn point_iterator(&self) -> PointIterator<T> {
        // self.grid
        //     .point_iterator()
        //     .map(|(point, v)| (point - self.translate, v))
        todo!()
    }

    fn print_grid(&self) {
        self.grid.print_grid()
    }

    fn x_max(&self) -> usize {
        self.grid.x_max()
    }

    fn y_max(&self) -> usize {
        self.grid.y_max()
    }

    fn x_len(&self, y: i32) -> usize {
        self.grid.x_len(y + self.translate.y)
    }
}

#[derive(Debug, Clone)]
pub struct ExpandingArrayGrid<T> {
    states: Vec<Vec<T>>,
}

impl<T: Debug + Clone + Default> ExpandingArrayGrid<T> {
    pub fn new() -> Self {
        ExpandingArrayGrid { states: vec![] }
    }

    fn resize_for_point(&mut self, point: &Point) {
        if self.states.len() <= point.y as usize {
            self.states.resize(point.y as usize + 1, vec![]);
        }
        if self.states[point.y as usize].len() <= point.x as usize {
            self.states[point.y as usize].resize(point.x as usize + 1, T::default());
        }
    }
}

impl<T: Debug + Clone + Default> Grid<T> for ExpandingArrayGrid<T> {
    fn lookup(&self, point: &Point) -> Option<&T> {
        if point.x >= 0 && point.y >= 0 {
            self.states
                .get(point.y as usize)
                .and_then(|row| row.get(point.x as usize))
        } else {
            None
        }
    }

    fn update(&mut self, point: &Point, state: T) {
        self.resize_for_point(point);
        self.states[point.y as usize][point.x as usize] = state;
    }

    fn update_fn(&mut self, point: &Point, state_fn: fn(&T) -> T) -> &T {
        self.resize_for_point(point);
        let new_state = state_fn(&self.states[point.y as usize][point.x as usize]);
        self.states[point.y as usize][point.x as usize] = new_state;
        &self.states[point.y as usize][point.x as usize]
    }

    fn point_iterator(&self) -> PointIterator<T> {
        PointIterator {
            grid: self,
            x: 0,
            y: 0,
        }
    }

    fn print_grid(&self) {
        let x_max = self.x_max();
        for y in 0..self.y_max() {
            for x in 0..x_max {
                if x >= self.x_len(y as i32) {
                    println!(".")
                } else {
                    print!("{:?}", self.states[y][x]);
                }
            }
            println!();
        }
    }

    fn x_max(&self) -> usize {
        self.states
            .iter()
            .map(|row| row.len())
            .max()
            .unwrap_or_default()
    }

    fn y_max(&self) -> usize {
        self.states.len()
    }

    fn x_len(&self, y: i32) -> usize {
        if y >= self.y_max() as i32 {
            0
        } else {
            self.states[y as usize].len()
        }
    }
}

impl<T: Debug + Clone + Default> ExpandingArrayGrid<T> {}

pub struct PointIterator<'a, T> {
    grid: &'a dyn Grid<T>,
    x: usize,
    y: usize,
}

impl<'a, T: Debug> Iterator for PointIterator<'a, T> {
    type Item = (Point, &'a T);

    fn next(&mut self) -> Option<Self::Item> {
        let point = Point::new(self.x as i32, self.y as i32);
        if let Some(state) = self.grid.lookup(&point) {
            if self.x == self.grid.x_len(self.y as i32) - 1 {
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
