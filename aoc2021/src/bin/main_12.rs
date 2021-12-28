use std::collections::hash_map::Entry;
use std::collections::{HashMap, HashSet};
use std::str::FromStr;

use failure::{bail, Fail};
use itertools::Itertools;
use util::aoc::frequency::FrequencyMap;
use util::aoc::grid::Grid;
use util::aoc::*;

use crate::Cave::{Big, Small};

type CaveMap = HashMap<Cave, Vec<Cave>>;

#[derive(Clone, Debug, Eq, PartialEq, Hash)]
enum Cave {
    Start,
    End,
    Big(String),
    Small(String),
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
}

impl FromStr for Cave {
    type Err = failure::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(match s {
            "start" => Cave::Start,
            "end" => Cave::End,
            s if s.chars().all(|c| c.is_uppercase()) => Cave::Big(s.to_string()),
            s => Cave::Small(s.to_string()),
        })
    }
}

fn main() -> AocResult<()> {
    let input = parse(input::read(12)?);

    result("Part 1", || part1(&input));
    result("Part 2", || part2(&input));

    Ok(())
}

fn parse(input: Vec<String>) -> CaveMap {
    let mut cave_map: CaveMap = HashMap::new();

    input
        .into_iter()
        .flat_map(
            |line| match line.split('-').collect_tuple().expect("Unexpected line") {
                (start, end) => vec![
                    (start.parse::<Cave>().unwrap(), end.parse::<Cave>().unwrap()),
                    (end.parse::<Cave>().unwrap(), start.parse::<Cave>().unwrap()),
                ],
                _ => panic!("Oh no"),
            },
        )
        .filter(|(start, end)| *end != Cave::Start && *start != Cave::End)
        .for_each(|(start, end)| {
            cave_map.entry(start).or_default().push(end);
        });

    cave_map
}

#[derive(Debug, Clone)]
struct CaveWithPath {
    path: Vec<Cave>,
    last_cave: Cave,
}

impl CaveWithPath {
    pub fn new(cave: Cave) -> CaveWithPath {
        CaveWithPath {
            path: vec![cave.clone()],
            last_cave: cave,
        }
    }

    pub fn last_step(&self) -> Option<Cave> {
        self.path.last().cloned()
    }

    pub fn is_visited(&self, cave: &Cave) -> bool {
        self.path.contains(cave)
    }

    pub fn max_small_cave_visits(&self) -> usize {
        let map: FrequencyMap<&Cave> = self.path.iter().filter(|&c| c.is_small()).collect();

        map.entries()
            .map(|(_, count)| *count as usize)
            .max()
            .unwrap_or(0)
    }

    pub fn append(&self, cave: Cave) -> CaveWithPath {
        let mut new_path = self.path.clone();
        new_path.push(cave.clone());

        CaveWithPath {
            path: new_path,
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
        next_cave.is_big() || !path.is_visited(&next_cave) || path.max_small_cave_visits() < 2
    })
}

fn solve(cave_map: &CaveMap, can_visit: fn(&CaveWithPath, &Cave) -> bool) -> i64 {
    // BFS, with partial results?
    let mut frontier = vec![CaveWithPath::new(Cave::Start)];
    let mut paths = 0;
    while let Some(path) = frontier.pop() {
        if path.last_cave == Cave::End {
            // println!("===> {:?}", path.path);
            paths += 1;
            continue;
        }

        for next_cave in cave_map.get(&path.last_cave).cloned().unwrap_or(vec![]) {
            // Avoid cycles.
            if Some(next_cave.clone()) == path.last_step() {
                continue;
            }

            if can_visit(&path, &next_cave) {
                // println!("{:?} -> {:?}", path.path, next_cave);
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
