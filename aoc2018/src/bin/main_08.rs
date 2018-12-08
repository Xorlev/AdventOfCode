use failure::*;
use util::aoc::*;

type MetadataEntry = u32;

#[derive(Debug, Default)]
struct Node {
    child_nodes: Vec<Node>,
    metadata_entry: Vec<MetadataEntry>,
}

fn main() -> Result<(), Box<std::error::Error>> {
    let lines: Vec<String> = input::read(8)?;
    let header = parse(&lines[0])?;
    println!("{:?}", header);

    result("Part 1", || part1(&header));
    result("Part 2", || part2(&header));

    Ok(())
}

fn part1(header: &Node) -> u32 {
    header.metadata_entry.iter().sum::<u32>()
        + header
            .child_nodes
            .iter()
            .map(part1)
            .sum::<u32>()
}

fn part2(header: &Node) -> u32 {
    if header.child_nodes.len() > 0 {
        header
            .metadata_entry
            .iter()
            .map(|entry| {
                header
                    .child_nodes
                    .get(*entry as usize - 1)
                    .map(part2)
                    .unwrap_or(0)
            })
            .sum()
    } else {
        header.metadata_entry.iter().sum::<u32>()
    }
}

/*
A header, which is always exactly two numbers:
 - The quantity of child nodes.
 - The quantity of metadata entries.
Zero or more child nodes (as specified in the header).
One or more metadata entries (as specified in the header).


2 3 0 3 10 11 12 1 1 0 1 99 2 1 1 2
A----------------------------------
    B----------- C-----------
                     D-----
*/
fn parse(line: &String) -> Result<Node, Error> {
    let pieces: Vec<u32> = line
        .split(" ")
        .map(|p| p.parse::<u32>())
        .collect::<Result<Vec<_>, std::num::ParseIntError>>()?;

    Ok(parse_header(&pieces)?.0)
}

fn parse_header(chunk: &[u32]) -> Result<(Node, usize), Error> {
    let num_child_nodes = *chunk.get(0).ok_or(format_err!("Bad chunk"))?;
    let num_metadata_nodes = *chunk.get(1).ok_or(format_err!("Bad chunk"))?;

    let mut node = Node::default();
    let mut position = 2;
    for _ in 0..num_child_nodes {
        let (child_node, consumed_entries) = parse_header(&chunk[position..])?;
        position += consumed_entries;
        node.child_nodes.push(child_node);
    }
    for _ in 0..num_metadata_nodes {
        node.metadata_entry.push(chunk[position]);
        position += 1;
    }

    Ok((node, position))
}
