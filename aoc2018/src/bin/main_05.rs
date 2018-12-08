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
    let mut p1 = 0;
    let mut p2 = 1;
    loop {
        if p2 > polymer.len() {
            break;
        }

        while p1 < polymer.len() - 1 && polymer[p1] == '?' {
            p1 += 1
        }

        p2 = p1 + 1;

        while p2 < polymer.len() && polymer[p2] == '?' {
            p2 += 1
        }

        if p1 >= p2 {
            panic!("Foo");
        }

        if p2 >= polymer.len() {
            break;
        }

        let c1 = polymer[p1];
        let c2 = polymer[p2];

        // If it's not aa, but it is lower(a) == lower(A), edit the polymer.
        if c1 != c2 && c1.to_ascii_lowercase() == c2.to_ascii_lowercase() {
            polymer[p1] = '?';
            polymer[p2] = '?';

            if p1 > 0 {
                p1 -= 1;
            }
        } else {
            p1 += 1;
        }
    }

    polymer.into_iter().filter(|&c| c != '?').map(|_| 1).sum()
}

fn part2(polymer: &String) -> usize {
    let unique_chars: HashSet<char> = polymer.chars().map(|c| c.to_ascii_lowercase()).collect();

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
