#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
enum Operation {
    UP, LEFT, RIGHT, DOWN
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
struct Position {
    x: i32,
    y: i32
}

static TABLE2: &'static [[char; 5]; 5] = &[
    [' ', ' ', '1', ' ', ' '],
    [' ', '2', '3', '4', ' '],
    ['5', '6', '7', '8', '9'],
    [' ', 'A', 'B', 'C', ' '],
    [' ', ' ', 'D', ' ', ' '],
];

fn main() {
    use std::io::{self, Read};

    let mut buffer = String::new();

    io::stdin().read_to_string(&mut buffer).unwrap();

    println!("{:?}", compute(buffer.trim()));
    println!("{:?}", compute2(buffer.trim()));
}

fn parse_input(input: &str) -> Vec<Vec<Operation>> {
    let parts: Vec<&str> = input.split("\n").map(|s| s.trim()).collect();

    parts.into_iter().map(process_step).collect()
}

fn compute(input: &str) -> String {
    let mut start = 5u32;
    let mut code = Vec::<String>::new();
    for line in parse_input(input) {
        for op in line {
            start = transition(start, op);
        }
        code.push(start.to_string())
    }

    code.join("")
}

fn compute2(input: &str) -> String {
    let mut start = Position{x: 0, y: 2};
    let mut code = Vec::<String>::new();
    for line in parse_input(input) {
        for op in line {
            start = transition_part2(start, op);
        }

        code.push(TABLE2[start.y as usize][start.x as usize].to_string());
    }

    code.join("")
}


fn process_step(input: &str) -> Vec<Operation> {
    input.chars().map(to_operation).collect()
}

fn to_operation(c: char) -> Operation {
    match c {
        'U' => Operation::UP,
        'D' => Operation::DOWN,
        'L' => Operation::LEFT,
        'R' => Operation::RIGHT,
        _ => panic!("Unknown char: {}", c)
    }
}

fn transition(number: u32, op: Operation) -> u32 {
    /*
    1 2 3
    4 5 6
    7 8 9

    UP removes 3      if n > 3
    DOWN adds 3       if n < 7
    RIGHT adds 1      if n % 3 == 0
    LEFT subtracts 1  if (n+2) % 3 == 0
    */

    match op {
        Operation::UP if number > 3 => number - 3,
        Operation::DOWN if number < 7 => number + 3,
        Operation::LEFT if (number+2) % 3 != 0 => number - 1,
        Operation::RIGHT if number % 3 != 0 => number + 1,
        _ => number
    }
}

fn transition_part2(number: Position, op: Operation) -> Position {
    /*
    1
  2 3 4
5 6 7 8 9
  A B C
    D

    if new_index >= 0 && new_index <= 5 && table[new index] != ' '
    */
    let x = number.x as usize;
    let y = number.y as usize;

    match op {
        Operation::UP if y > 0 && TABLE2[y - 1][x] != ' ' => Position{x: x as i32, y: (y - 1) as i32},
        Operation::DOWN if y < 4 && TABLE2[y + 1][x] != ' ' => Position{x: x as i32, y: (y + 1) as i32},
        Operation::LEFT if x > 0 && TABLE2[y][x - 1] != ' ' => Position{x: (x - 1) as i32, y: y as i32},
        Operation::RIGHT if x < 4 && TABLE2[y][x + 1] != ' ' => Position{x: (x + 1) as i32, y: y as i32},
        _ => number
    }
}

#[test]
fn test_small_input() {
        let input =
"ULL
RRDDD
LURDL
UUUUD";
    assert_eq!("1985", compute(input))
}

#[test]
fn test_small_input2() {
        let input =
"ULL
RRDDD
LURDL
UUUUD";
    assert_eq!("5DB3", compute2(input))
}
