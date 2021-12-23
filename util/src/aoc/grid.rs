use crate::aoc::Point;

#[derive(Debug, Clone)]
pub struct Grid<T> {
    states: Vec<Vec<T>>,
}

impl<T> Grid<T> {
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
}

pub struct PointIterator<'a, T> {
    grid: &'a Grid<T>,
    x: usize,
    y: usize,
}

impl<'a, T> Iterator for PointIterator<'a, T> {
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
