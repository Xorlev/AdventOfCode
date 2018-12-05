use std::collections::HashSet;
use util::aoc::*;

fn main() -> Result<(), Box<std::error::Error>> {
    let lines: Vec<String> = input::read(5)?;
    let string = &lines[0];
//    let string = &"dacAcCaCBAcCcaDA".to_string();
    result("Part 1", || part1(string));
    result("Part 2", || part2(string));

    Ok(())
}

fn part1(polymer: &String) -> usize {
    let mut polymer: Vec<char> = polymer.chars().collect();

    let mut index = 0;
    loop {
        if index + 1 >= polymer.len() {
            break;
        }

        let c1 = polymer[index];
        let c2 = polymer[index + 1];

        // If it's not aa, but it is lower(a) == lower(A), edit the polymer.
        if c1 != c2 && c1.to_ascii_lowercase() == c2.to_ascii_lowercase() {
            polymer.remove(index);
            polymer.remove(index);

            if index > 0 {
                index = index - 1
            }
        } else {
            index += 1
        }
    }

    polymer.len()
}

fn part2(polymer: &String) -> usize {
    let unique_chars: HashSet<char> = polymer
        .chars()
        .map(|c| c.to_ascii_lowercase())
        .collect();

    let mut shortest_polymer = polymer.len();
    for char_to_remove in unique_chars {
        let edited_polymer: String = polymer
            .chars()
            .filter(|c| c.to_ascii_lowercase() != char_to_remove)
            .collect();

        shortest_polymer = std::cmp::min(shortest_polymer, part1(&edited_polymer));
    }

    shortest_polymer
}
