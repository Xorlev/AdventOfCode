use std::ops::Add;
use std::collections::HashSet;

trait Steps {
    fn steps(self) -> i32;
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
enum Operation {
    Left(i32),
    Right(i32),
    Unknown
}

impl Steps for Operation {
    fn steps(self) -> i32 {
        match self {
            Operation::Left(s) => s,
            Operation::Right(s) => s,
            Operation::Unknown => 0
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
enum Direction {
    North, South, East, West
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
struct Position {
    x: i32,
    y: i32
}

impl Position {
    fn distance(self, other: Position) -> u32 {
        let x = self.x - other.x;
        let y = self.y - other.y;

        (x.abs() + y.abs()) as u32
    }

    fn steps(self, steps: u32, direction: Direction) -> Vec<Position> {
        let mut position = self.clone();
        let mut positions = Vec::<Position>::new();
        let v = unit_vector(direction);

        for _ in 0..steps {
            position = position + v;
            positions.push(position);
        }

        positions
    }
}

impl Add for Position {
    type Output = Position;

    fn add(self, other: Position) -> Position {
        Position {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}


fn main() {
    use std::io::{self, Read};

    let mut buffer = String::new();

    io::stdin().read_to_string(&mut buffer).unwrap();

    let parts: Vec<&str> = buffer.split(", ").map(|s| s.trim()).collect();

    let output: Vec<Operation> = parts.into_iter().map(process_step).collect();

    let mut pos = Position{x: 0, y: 0};
    let mut d = Direction::North;
    let mut point_history: HashSet<Position> = HashSet::new();
    let mut first_pos: Option<Position> = Option::None;

    for op in output {
        d = new_direction(&d, &op);
        let steps = &pos.steps(op.steps() as u32, d);

        if first_pos.is_none() {
            for step in steps {
                if first_pos.is_none() && !point_history.insert(*step) {
                    first_pos = Some(*step);
                }
            }
        }

        pos = *steps.last().unwrap();
        println!("{:?}", pos);
    }

    println!("Distance from HQ: {:?}", pos.distance(Position{x:0, y:0}));

    if let Some(p) = first_pos {
        println!("Distance from first repeated pair: {:?}", p.distance(Position{x:0, y:0}));
    }
}

fn new_direction(direction: &Direction, op: &Operation) -> Direction {
    match (direction, op) {
        (&Direction::North, &Operation::Left(_)) => Direction::West,
        (&Direction::North, &Operation::Right(_)) => Direction::East,
        (&Direction::East, &Operation::Left(_)) => Direction::North,
        (&Direction::East, &Operation::Right(_)) => Direction::South,
        (&Direction::South, &Operation::Left(_)) => Direction::East,
        (&Direction::South, &Operation::Right(_)) => Direction::West,
        (&Direction::West, &Operation::Left(_)) => Direction::South,
        (&Direction::West, &Operation::Right(_)) => Direction::North,
        (_, _) => panic!("Bad direction/op")
    }
}

fn unit_vector(direction: Direction) -> Position {
    match direction {
        Direction::North => Position{x: 0, y: 1},
        Direction::South => Position{x: 0, y: -1},
        Direction::East => Position{x: 1, y: 0},
        Direction::West => Position{x: -1, y: 0}
    }
}

fn process_step(s: &str) -> Operation {
    match s {
        x if x.starts_with("L") => Operation::Left(to_steps(x)),
        x if x.starts_with("R") => Operation::Right(to_steps(x)),
        _ => Operation::Unknown
    }
}

fn to_steps(s: &str) -> i32 {
    s.chars().skip(1).collect::<String>().parse::<i32>().unwrap()
}
