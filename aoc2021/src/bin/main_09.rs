use itertools::Itertools;
use std::collections::hash_map::Entry;
use std::collections::HashMap;
use util::aoc::grid::Grid;
use util::aoc::*;

type HeightMap = Grid<i32>;

fn main() -> AocResult<()> {
    let input: Vec<Vec<i32>> = input::read(9)?
        .iter()
        .map(|s| {
            s.chars()
                .map(|c| c.to_digit(10).unwrap() as i32)
                .collect_vec()
        })
        .collect();
    let height_map = Grid::new(input);

    result("Part 1", || part1(&height_map));
    result("Part 2", || part2(&height_map));

    Ok(())
}

fn part1(height_map: &HeightMap) -> i32 {
    height_map
        .point_iterator()
        .map(|(point, height)| {
            let is_low_point = point.neighbors4().iter().all(|neighbor| {
                if let Some(neighbor_height) = height_map.lookup(neighbor) {
                    neighbor_height > height
                } else {
                    true
                }
            });

            if is_low_point {
                height + 1
            } else {
                0
            }
        })
        .sum()
}

fn part2(height_map: &HeightMap) -> i32 {
    let mut point_to_basin_id: HashMap<Point, i32> = HashMap::new();

    // Connected components:
    //  - BFS from each point.
    //  - Check if that point has a component assigned.
    //  - If not, assign and explore its neighbors.
    let mut basin_id = 0;
    for (point, &height) in height_map.point_iterator() {
        if height > 8 || point_to_basin_id.contains_key(&point) {
            continue;
        }

        let mut frontier = vec![point];
        while let Some(frontier_point) = frontier.pop() {
            if let Entry::Vacant(e) = point_to_basin_id.entry(frontier_point) {
                e.insert(basin_id);
                for neighbor_point in frontier_point.neighbors4() {
                    if !point_to_basin_id.contains_key(&neighbor_point) {
                        match height_map.lookup(&neighbor_point) {
                            Some(&height) if height < 9 => frontier.push(neighbor_point),
                            _ => {}
                        }
                    }
                }
            }
        }

        basin_id += 1;
    }

    // Then group by basin_id
    let basins = point_to_basin_id.iter().into_group_map_by(|(_, &v)| v);

    // And multiply the top 3.
    basins
        .values()
        .map(|g| g.len() as i32)
        .sorted()
        .rev()
        .take(3)
        .product()
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE_INPUT: &str = r#"2199943210
3987894921
9856789892
8767896789
9899965678"#;

    #[test]
    fn part1_sample() {
        let inputs = SAMPLE_INPUT
            .lines()
            .map(|s| {
                s.chars()
                    .map(|c| c.to_digit(10).unwrap() as i32)
                    .collect_vec()
            })
            .collect_vec();
        let height_map = Grid::new(inputs);

        assert_eq!(15, part1(&height_map));
    }

    #[test]
    fn part2_sample() {
        let inputs = SAMPLE_INPUT
            .lines()
            .map(|s| {
                s.chars()
                    .map(|c| c.to_digit(10).unwrap() as i32)
                    .collect_vec()
            })
            .collect_vec();
        let height_map = Grid::new(inputs);

        assert_eq!(1134, part2(&height_map));
    }
}
