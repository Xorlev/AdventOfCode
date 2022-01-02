use bit_set::BitSet;
use itertools::Itertools;
use std::collections::HashMap;
use util::aoc::*;

use crate::Cave::{Big, Small};

type CaveMap = HashMap<Cave, Vec<Cave>>;

#[derive(Clone, Debug, Eq, PartialEq, Hash)]
enum Cave {
    Start,
    End,
    Big(usize),
    Small(usize),
}

impl Cave {
    fn is_big(&self) -> bool {
        match self {
            Big(_) => true,
            _ => false,
        }
    }

    fn is_small(&self) -> bool {
        match self {
            Small(_) => true,
            _ => false,
        }
    }

    fn index(&self) -> usize {
        match self {
            Cave::Start => 0,
            Cave::End => 1,
            Big(index) => *index,
            Small(index) => *index,
        }
    }

    fn parse_node(
        s: &str,
        string_to_index: &mut HashMap<String, usize>,
        current_index: &mut usize,
    ) -> Cave {
        let index = *string_to_index.entry(s.to_string()).or_insert_with(|| {
            *current_index += 1;
            *current_index
        });

        match s {
            "start" => Cave::Start,
            "end" => Cave::End,
            s if s.chars().all(|c| c.is_uppercase()) => Cave::Big(index),
            _ => Cave::Small(index),
        }
    }
}

fn main() -> AocResult<()> {
    let input = parse(input::read(12)?);

    result("Part 1", || part1(&input));
    result("Part 2", || part2(&input));

    Ok(())
}

fn parse(input: Vec<String>) -> CaveMap {
    let mut current_index = 1usize;
    let mut string_to_index = HashMap::new();
    let mut cave_map: CaveMap = HashMap::new();

    input
        .into_iter()
        .flat_map(
            |line| match line.split('-').collect_tuple().expect("Unexpected line") {
                (start, end) => {
                    let start = Cave::parse_node(start, &mut string_to_index, &mut current_index);
                    let end = Cave::parse_node(end, &mut string_to_index, &mut current_index);
                    vec![(start.clone(), end.clone()), (end, start)]
                }
            },
        )
        .filter(|(start, end)| *end != Cave::Start && *start != Cave::End)
        .for_each(|(start, end)| {
            cave_map.entry(start).or_default().push(end);
        });

    cave_map
}

#[derive(Debug, Clone)]
struct CaveWithPath<'a> {
    path: Vec<&'a Cave>,
    visited: BitSet,
    has_visited_twice: bool,
    last_cave: &'a Cave,
}

impl<'a> CaveWithPath<'a> {
    pub fn new(cave: &'a Cave) -> CaveWithPath<'a> {
        let mut visited = BitSet::new();
        visited.insert(cave.index());

        CaveWithPath {
            path: vec![cave],
            visited,
            has_visited_twice: false,
            last_cave: cave,
        }
    }

    pub fn last_step(&self) -> Option<&Cave> {
        self.path.last().map(|&c| c)
    }

    pub fn is_visited(&self, cave: &Cave) -> bool {
        self.visited.contains(cave.index())
    }

    pub fn append(&self, cave: &'a Cave) -> CaveWithPath<'a> {
        let mut new_path = self.path.clone();
        new_path.push(cave);
        let mut new_visited = self.visited.clone();
        let is_newly_visited = new_visited.insert(cave.index());
        let has_visited_twice = if !self.has_visited_twice && cave.is_small() {
            !is_newly_visited
        } else {
            self.has_visited_twice
        };

        CaveWithPath {
            path: new_path,
            visited: new_visited,
            has_visited_twice,
            last_cave: cave,
        }
    }
}

fn part1(cave_map: &CaveMap) -> i64 {
    solve(cave_map, |path, next_cave| {
        // Only visit small caves once.
        next_cave.is_big() || !path.is_visited(&next_cave)
    })
}

fn part2(cave_map: &CaveMap) -> i64 {
    solve(cave_map, |path, next_cave| {
        // Allow visiting a single small cave at least once.
        next_cave.is_big() || !path.is_visited(&next_cave) || !path.has_visited_twice
    })
}

fn solve(cave_map: &CaveMap, can_visit: fn(&CaveWithPath, &Cave) -> bool) -> i64 {
    let mut frontier = vec![CaveWithPath::new(&Cave::Start)];
    let mut paths = 0;
    while let Some(path) = frontier.pop() {
        if path.last_cave == &Cave::End {
            paths += 1;
            continue;
        }

        for next_cave in cave_map.get(&path.last_cave).unwrap() {
            // Avoid cycles.
            if Some(next_cave) == path.last_step() {
                continue;
            }

            if can_visit(&path, &next_cave) {
                frontier.push(path.append(next_cave));
            }
        }
    }

    paths
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE_INPUT: &str = r#"start-A
start-b
A-c
A-b
b-d
A-end
b-end"#;

    #[test]
    fn part1_sample() {
        let input = parse(SAMPLE_INPUT.lines().map(|s| s.to_string()).collect());

        println!("{:?}", input);

        assert_eq!(10, part1(&input));
    }

    #[test]
    fn part2_sample() {
        let input = parse(SAMPLE_INPUT.lines().map(|s| s.to_string()).collect());

        println!("{:?}", input);

        assert_eq!(36, part2(&input));
    }
}
