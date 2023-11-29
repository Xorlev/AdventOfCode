use crate::day7::Node::{Directory, File};
use itertools::Itertools;
use std::collections::HashMap;

type NodeHandle = usize;

#[derive(PartialEq, Eq, Hash)]
enum Node {
    Directory {
        name: String,
        children: Vec<NodeHandle>,
        parent: Option<NodeHandle>,
    },
    File {
        name: String,
        size: u64,
        parent: NodeHandle,
    },
}

impl Node {
    fn name(&self) -> &str {
        match self {
            Directory { name, .. } => name,
            File { name, .. } => name,
        }
    }
    fn parent(&self) -> Option<NodeHandle> {
        match self {
            Directory { parent, .. } => *parent,
            File { parent, .. } => Some(*parent),
        }
    }
    fn is_directory(&self) -> bool {
        matches!(self, Directory { .. })
    }

    fn new_directory(parent: Option<NodeHandle>, name: String) -> Node {
        Directory {
            name,
            children: vec![],
            parent,
        }
    }
    fn new_file(parent: NodeHandle, name: String, size: u64) -> Node {
        File { parent, name, size }
    }
}

struct DirectoryTree {
    root: NodeHandle,
    cwd: NodeHandle,
    nodes: Vec<Node>,
}

impl DirectoryTree {
    pub fn new() -> Self {
        Self {
            root: 0,
            cwd: 0,
            nodes: vec![Node::new_directory(None, "".to_string())],
        }
    }

    fn add_directory(&mut self, name: String) {
        let index = self.nodes.len();
        self.nodes
            .push(Node::new_directory(Some(self.cwd), name.clone()));
        if let Directory { children, .. } = &mut self.nodes[self.cwd] {
            children.push(index);
        }
    }

    fn add_file(&mut self, name: String, size: u64) {
        let index = self.nodes.len();
        self.nodes
            .push(Node::new_file(self.cwd, name.clone(), size));
        if let Directory { children, .. } = &mut self.nodes[self.cwd] {
            children.push(index);
        }
    }

    fn node(&self, node: NodeHandle) -> &Node {
        &self.nodes[node]
    }

    fn nodes(&self) -> &Vec<Node> {
        &self.nodes
    }

    fn chdir(&mut self, name: &str) {
        if let Directory { children, .. } = &self.nodes[self.cwd] {
            for child in children {
                let node = &self.nodes[*child];
                if node.is_directory() && node.name() == name {
                    self.cwd = *child;
                }
            }
        }
    }

    fn chdir_root(&mut self) {
        self.cwd = self.root;
    }

    fn pop_dir(&mut self) -> Option<NodeHandle> {
        if let Directory {
            parent: Some(parent),
            ..
        } = &self.nodes[self.cwd]
        {
            self.cwd = *parent;
            return Some(*parent);
        }
        None
    }
}

#[aoc_generator(day7)]
fn parse(input: &str) -> DirectoryTree {
    let mut root = DirectoryTree::new();

    input.lines().for_each(|command| {
        let command_parts = command.split_ascii_whitespace().collect_vec();
        match command_parts.as_slice() {
            ["$", "ls"] => {}
            ["$", "cd", dir_name] => match *dir_name {
                "/" => root.chdir_root(),
                ".." => {
                    root.pop_dir();
                }
                _ => root.chdir(dir_name),
            },
            ["dir", dir_name] => {
                root.add_directory(dir_name.to_string());
            }
            [num, file_name] => {
                root.add_file(file_name.to_string(), num.parse().unwrap());
            }
            _ => {}
        }
    });

    root.chdir_root();
    root
}

#[aoc(day7, part1)]
fn part1(input: &DirectoryTree) -> u64 {
    calculate_directory_sizes(input)
        .values()
        .filter(|size| *size <= &100000u64)
        .sum()
}

#[aoc(day7, part2)]
fn part2(input: &DirectoryTree) -> u64 {
    let sizes = calculate_directory_sizes(input);
    let free_space = 70000000 - sizes.get(input.node(input.root)).unwrap();
    let required_space = 30000000 - free_space;

    *sizes
        .values()
        .sorted()
        .find(|size| **size >= required_space)
        .unwrap()
}

fn calculate_directory_sizes(input: &DirectoryTree) -> HashMap<&Node, u64> {
    let mut dir_sizes = HashMap::new();
    for node in input.nodes() {
        if let File { size, .. } = node {
            let mut p = node.parent();
            while let Some(parent) = p {
                let parent_node = input.node(parent);
                dir_sizes
                    .entry(parent_node)
                    .and_modify(|e| *e += size)
                    .or_insert(*size);
                p = parent_node.parent();
            }
        }
    }
    dir_sizes
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "$ cd /
$ ls
dir a
14848514 b.txt
8504156 c.dat
dir d
$ cd a
$ ls
dir e
29116 f
2557 g
62596 h.lst
$ cd e
$ ls
584 i
$ cd ..
$ cd ..
$ cd d
$ ls
4060174 j
8033020 d.log
5626152 d.ext
7214296 k";

    #[test]
    fn p1() {
        let node = parse(INPUT);
        assert_eq!(95437, part1(&node));
    }

    #[test]
    fn p2() {
        let node = parse(INPUT);
        assert_eq!(24933642, part2(&node));
    }
}
