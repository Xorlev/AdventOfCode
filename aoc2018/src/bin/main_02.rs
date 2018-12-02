use util::aoc::time;
use util::aoc::input;
use std::collections::hash_map::HashMap;

fn main() ->  Result<(), Box<std::error::Error>> {
    let lines: Vec<String> = input::read(2)?;

    println!("Part 1: {}", time(|| part1(&lines)));
    println!("Part 2: {:?}", time(|| part2(&lines)));

    Ok(())
}

fn part1(lines: &Vec<String>) -> u32 {
    let counts = lines
        .iter()
        .map(checksum)
        .fold((0, 0), |acc, c| (acc.0+c.0, acc.1+c.1));

    counts.0 * counts.1
}

fn checksum(box_id: &String) -> (u32, u32) {
    let mut frequency_map: HashMap<char, u32> = HashMap::new();

    for ch in box_id.chars() {
        let counter = frequency_map.entry(ch).or_insert(0);
        *counter += 1;
    };

    let mut occurs_twice = 0;
    let mut occurs_thrice = 0;
    for entry in frequency_map.iter() {
        if *entry.1 == 2 {
            occurs_twice = 1
        }
        if *entry.1 == 3 {
            occurs_thrice = 1
        }
    }

    (occurs_twice, occurs_thrice)
}

fn part2(lines: &Vec<String>) -> Option<String> {
    for l1 in lines {
        for l2 in lines {
            if l1 != l2 {
                let edit_str = edited_string(l1, l2);

                if edit_str.is_some() {
                    return edit_str
                }
            }
        }
    }

    None
}

fn edited_string(str1: &String, str2: &String) -> Option<String> {
    let mut s1_chars = str1.chars();
    let mut s2_chars = str2.chars();

    let mut dist = 0;
    let mut new_str = String::new();
    loop {
        match (s1_chars.next(), s2_chars.next()) {
            (Some(c1), Some(c2)) => {
                if c1 != c2 {
                    dist += 1
                } else {
                    new_str.push(c1)
                }
            }
            (Some(_), None) => return None,
            (None, Some(_)) => return None,
            (None, None) => break
        }
    }

    if dist == 1 {
        Some(new_str)
    } else {
        None
    }
}