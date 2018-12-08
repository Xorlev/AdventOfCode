use failure::*;
use lazy_static::*;
use regex::Regex;
use std::cmp::max;
use util::aoc::*;

lazy_static! {
    static ref RE: Regex = Regex::new("#(\\d+) @ (\\d+),(\\d+): (\\d+)x(\\d+)").unwrap();
}

#[derive(Debug)]
struct Claim {
    id: String,
    from_left: usize,
    from_top: usize,
    height: usize,
    width: usize,
}

impl Claim {
    fn parse(claim_str: &String) -> Result<Claim, Error> {
        if let Some(captures) = RE.captures(claim_str.as_str()) {
            return Ok(Claim {
                id: captures[1].to_owned(),
                from_left: captures[2].parse::<usize>()?,
                from_top: captures[3].parse::<usize>()?,
                width: captures[4].parse::<usize>()?,
                height: captures[5].parse::<usize>()?,
            });
        }

        Err(format_err!("Couldn't parse claim: {}", claim_str))
    }

    fn abs_height(&self) -> usize {
        self.from_top + self.height
    }

    fn abs_width(&self) -> usize {
        self.from_left + self.width
    }
}

fn main() -> Result<(), Box<std::error::Error>> {
    let lines: Vec<String> = input::read(3)?;
    let claims: Vec<Claim> = lines
        .iter()
        .map(Claim::parse)
        .collect::<Result<Vec<_>, _>>()?;
    let (size, grid) = time("Grid", || build_grid(&claims));

    result("Part 1", || part1(size, &grid));
    result("Part 2", || part2(&grid, &claims));

    Ok(())
}

// How many square inches of fabric are within two or more claims?
fn part1(size: (usize, usize), grid: &[Vec<u32>]) -> u32 {
    let mut overlapping_squares = 0;
    for y in 0..size.1 {
        for x in 0..size.0 {
            if grid[y][x] > 1 {
                overlapping_squares += 1
            }
        }
    }

    overlapping_squares
}

// How many square inches of fabric are within two or more claims?
fn part2(grid: &[Vec<u32>], claims: &[Claim]) -> Option<String> {
    for claim in claims {
        let mut overlapping = false;
        for y in claim.from_left..claim.abs_width() {
            for x in claim.from_top..claim.abs_height() {
                if grid[y][x] > 1 {
                    overlapping = true;
                    break;
                }
            }

            if overlapping {
                break;
            }
        }

        if !overlapping {
            return Some(claim.id.to_string());
        }
    }

    None
}

fn build_grid(claims: &[Claim]) -> ((usize, usize), Vec<Vec<u32>>) {
    // Find the size of the grid
    let size = claims
        .iter()
        .map(|c| (c.abs_height(), c.abs_width()))
        .fold((0, 0), |acc, c| (max(acc.0, c.0), max(acc.1, c.1)));

    // Create the grid
    let mut grid = vec![vec![0u32; size.0]; size.1];
    for claim in claims {
        for y in claim.from_left..claim.abs_width() {
            for x in claim.from_top..claim.abs_height() {
                grid[y][x] += 1
            }
        }
    }
    (size, grid)
}
