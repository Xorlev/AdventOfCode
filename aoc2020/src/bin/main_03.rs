
use util::aoc::*;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let geology: Vec<Vec<Entity>> = input::read(3)?
        .into_iter()
        .map(|line| parse_line(line))
        .collect();

    println!("{:?}", geology);

    result("Part 1", || part1(&geology));
    result("Part 2", || part2(&geology));

    Ok(())
}

fn part1(geology: &[Vec<Entity>]) -> i32 {
    count_trees_on_slope(geology, 3, 1)
}

fn part2(geology: &[Vec<Entity>]) -> i64 {
    let slopes = vec![vec![1, 1], vec![3, 1], vec![5, 1], vec![7, 1], vec![1, 2]];

    slopes
        .iter()
        .map(|slope| count_trees_on_slope(geology, slope[0], slope[1]) as i64)
        .product()
}

fn count_trees_on_slope(geology: &[Vec<Entity>], right: usize, down: usize) -> i32 {
    let mut trees = 0;
    let mut i = down;
    while i < geology.len() {
        let horizontal_slice = &geology[i];
        match horizontal_slice[(i / down * right) % horizontal_slice.len()] {
            Entity::Open => {}
            Entity::Tree => trees += 1,
        };

        i += down;
    }
    trees
}

fn parse_line(line: String) -> Vec<Entity> {
    line.chars()
        .map(|c| match c {
            '.' => Entity::Open,
            '#' => Entity::Tree,
            _ => panic!("Unexpected character: {:?}", c),
        })
        .collect()
}

#[derive(Debug)]
enum Entity {
    Open,
    Tree,
}
