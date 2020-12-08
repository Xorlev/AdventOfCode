use std::collections::{HashMap, HashSet, VecDeque};

use failure::bail;
use itertools::{Itertools, MinMaxResult};
use lazy_static::lazy_static;
use regex::Regex;
use util::aoc::*;

lazy_static! {
    static ref RE: Regex = Regex::new("(\\d+) ([a-z]+ [a-z]+) bag").unwrap();
}

type Answers = HashSet<char>;

fn main() -> AocResult<()> {
    let bag_graph: HashMap<String, Vec<Bag>> = input::read(7)?
        .iter()
        .map(|line| {
            let parts = line.split(" bags contain ").collect_vec();
            let contained_bags = RE
                .captures_iter(parts[1])
                .map(|c| Bag {
                    count: c[1].parse().unwrap(),
                    bag_type: c[2].to_string(),
                })
                .collect_vec();

            (parts[0].to_string(), contained_bags)
        })
        .collect();

    result("Part 1", || part1(&bag_graph));
    result("Part 2", || part2(&bag_graph));

    Ok(())
}

#[derive(Debug)]
struct Bag {
    count: i32,
    bag_type: String,
}

fn part1(bag_graph: &HashMap<String, Vec<Bag>>) -> usize {
    fn search<'a: 'b, 'b>(bag_graph: &'a HashMap<String, Vec<Bag>>, has_gold_bag: &'b mut HashMap<&'a str, bool>, bag: &str) -> bool {
        if let Some(contains_bag) = has_gold_bag.get(bag) {
            return *contains_bag;
        }

        if let Some(held_bags) = bag_graph.get(bag) {
            for bag in held_bags {
                if bag.bag_type == "shiny gold" || search(&bag_graph, has_gold_bag, &bag.bag_type) {
                    has_gold_bag.insert(&bag.bag_type, true);
                    return true;
                } else {
                    has_gold_bag.insert(&bag.bag_type, false);
                }
            }
        }

        false
    }

    let mut has_gold_bag: HashMap<&str, bool> = HashMap::new();
    bag_graph
        .keys()
        .filter(|&key| search(bag_graph, &mut has_gold_bag, key))
        .count()
}

fn part2(bag_graph: &HashMap<String, Vec<Bag>>) -> i32 {
    fn count_bags(bag_graph: &HashMap<String, Vec<Bag>>, key: &str) -> i32 {
        if let Some(bags) = bag_graph.get(key) {
            bags.iter()
                .map(|bag| bag.count + bag.count * count_bags(&bag_graph, &bag.bag_type))
                .sum()
        } else {
            0
        }
    }

    count_bags(&bag_graph, "shiny gold")
}
