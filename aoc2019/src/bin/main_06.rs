use aoc2019::intcode::*;
use failure::{bail, format_err, Error};
use itertools::Itertools;
use std::borrow::Borrow;
use std::collections::hash_set::HashSet;
use std::collections::HashMap;
use std::str::FromStr;
use util::aoc::*;

type Object = String;
type Edge = (Object, Object);

const SAN: &str = "SAN";
const YOU: &str = "YOU";

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let lines: Vec<String> = input::read(6)?
        .iter()
        .map(|f| f.split(",").map(|s| s.to_string()).collect())
        .collect();
    let edges: Vec<Edge> = parse(lines);

    result("Part 1", || part1(edges.clone()));
    result("Part 2", || part2(edges.clone()));

    Ok(())
}

fn part1(edges: Vec<Edge>) -> usize {
    let (objects, parent_graph) = build_graph(edges);

    // Count all paths.
    return objects
        .iter()
        .flat_map(|object| parent_graph.traverse(object))
        .count();
}

fn part2(edges: Vec<Edge>) -> usize {
    let (objects, parent_graph) = build_graph(edges);

    // Find the common parent of SAN and YOU.
    let mut san_chain: Vec<Object> = parent_graph.traverse(&SAN).collect();
    return parent_graph
        .traverse(&YOU)
        .enumerate()
        .find_map(|(you_length, object)| {
            san_chain
                .iter()
                .position(|o| *o == object)
                .map(|san_length| you_length + san_length)
        })
        .unwrap_or(0);
}

struct ParentGraph {
    object_to_parent: HashMap<Object, Object>,
}

impl ParentGraph {
    fn new(object_to_parent: HashMap<Object, Object>) -> ParentGraph {
        ParentGraph { object_to_parent }
    }
    fn traverse(&self, start_at: &str) -> ParentGraphIterator {
        ParentGraphIterator {
            current_object: start_at.to_string(),
            parent_graph: self,
        }
    }
}

struct ParentGraphIterator<'a> {
    current_object: Object,
    parent_graph: &'a ParentGraph,
}

impl<'a> Iterator for ParentGraphIterator<'a> {
    type Item = Object;

    fn next(&mut self) -> Option<Self::Item> {
        match self.parent_graph.object_to_parent.get(&self.current_object) {
            Some(parent) => {
                self.current_object = parent.clone();
                Some(self.current_object.clone())
            }
            None => None,
        }
    }
}

fn parse(lines: Vec<String>) -> Vec<Edge> {
    lines
        .iter()
        .map(|l| l.split(")").collect::<Vec<_>>())
        .map(|s| (s[0].to_string(), s[1].to_string()))
        .collect()
}

fn build_graph(edges: Vec<Edge>) -> (HashSet<Object>, ParentGraph) {
    let mut objects = HashSet::new();
    let mut object_to_parent = HashMap::new();
    edges.into_iter().for_each(|(parent_object, orbit_object)| {
        objects.insert(orbit_object.clone());
        objects.insert(parent_object.clone());
        object_to_parent.insert(orbit_object, parent_object);
    });

    (objects, ParentGraph::new(object_to_parent))
}
