use std::borrow::Borrow;
use std::collections::{HashMap, VecDeque};
use std::collections::hash_set::HashSet;
use std::f32;
use std::fmt::Write;
use std::str::FromStr;

use failure::{bail, Error, format_err};
use failure::_core::fmt::Formatter;
use itertools::{Itertools, MinMaxResult};
use util::aoc::*;

use aoc2019::intcode::*;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let lines: Vec<String> = input::read(13)?
        .first()
        .map(|f| f.split(",").map(|s| s.to_string()).collect())
        .unwrap_or(Vec::new());
    let memory = lines.parse::<i64>()?;

    result("Part 1", || part1(memory.clone()));
    result("Part 2", || part2(memory.clone()));

    Ok(())
}

fn part1(memory: Vec<i64>) -> Result<usize, Error> {
    let mut computer = Computer::init(memory);
    let mut outputs = Vec::new();
    loop {
        match computer.resume(None)? {
            IOResult::InputRequired => panic!("Input!?"),
            IOResult::Output(o) => outputs.push(o as i32),
            IOResult::Halt(_) => break,
        }
    }

    let outputs = outputs.chunks(3)
        .map(|o| (Point::new(o[0], o[1]), o[2]))
        .collect::<Vec<_>>();

    let blocks = outputs
        .iter()
        .filter(|&&(_, t)| t == 2)
        .map(|(pt, _)| pt)
        .unique()
        .count();

    Ok(blocks)
}

fn part2(mut memory: Vec<i64>) -> Result<i32, Error> {
    // Set memory address 0 to '2' to play for free.
    memory[0] = 2;

    let mut computer = Computer::init(memory);
    let mut output_buffer = Vec::new();
    let mut ball_position: Option<Point> = None;
    let mut paddle_position: Option<Point> = None;
    let mut score = -1;
    let mut joystick_pos = None;
    loop {
        let input = joystick_pos.take();
        match computer.resume(input)? {
            IOResult::InputRequired => {
                let new_pos = if let (Some(ball), Some(paddle)) = (ball_position, paddle_position) {
                    if ball.x < paddle.x {
                        -1
                    } else if ball.x > paddle.x {
                        1
                    } else {
                        0
                    }
                } else {
                    0
                };

                joystick_pos = Some(new_pos)
            }
            IOResult::Output(o) => {
                output_buffer.push(o as i32);

                if output_buffer.len() == 3 {
                    let tile = Tile::new(output_buffer[0], output_buffer[1], output_buffer[2]);

                    if tile.point.x == -1 && tile.point.y == 0 {
                        score = output_buffer[2];
                    } else {
                        match tile.tile_type {
                            TileType::Ball => ball_position = Some(tile.point.clone()),
                            TileType::Paddle => paddle_position = Some(tile.point.clone()),
                            _ => {}
                        }
                    }

                    output_buffer.clear();
                }
            }
            IOResult::Halt(_) => break,
        }
    }

    Ok(score)
}

#[derive(Copy, Clone, Debug)]
struct Tile {
    point: Point,
    tile_type: TileType,
}

impl Tile {
    fn new(x: i32, y: i32, tile_type: i32) -> Tile {
        Tile {
            tile_type: TileType::from_i32(tile_type),
            point: Point::new(x, y),
        }
    }
}

#[derive(Copy, Clone, Debug)]
enum TileType {
    Empty,
    Wall,
    Block,
    Paddle,
    Ball,
    Score(i32),
}

impl TileType {
    fn from_i32(input: i32) -> TileType {
        match input {
            0 => TileType::Empty,
            1 => TileType::Wall,
            2 => TileType::Block,
            3 => TileType::Paddle,
            4 => TileType::Ball,
            x => TileType::Score(x),
        }
    }
}