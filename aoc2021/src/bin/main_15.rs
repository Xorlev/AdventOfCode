use failure::bail;
use itertools::Itertools;
use util::aoc::astar::AStarResult;
use util::aoc::grid::Grid;
use util::aoc::*;

type RiskMap = Grid<i32>;

fn main() -> AocResult<()> {
    let input: Vec<Vec<i32>> = parse(input::read(15)?);

    let risk_map = Grid::new(input);

    result("Part 1", || part1(&risk_map).expect("Failed to find path"));
    result("Part 2", || part2(&risk_map).expect("Failed to find path"));

    Ok(())
}

fn parse(lines: Vec<String>) -> Vec<Vec<i32>> {
    lines
        .iter()
        .map(|s| {
            s.chars()
                .map(|c| c.to_digit(10).unwrap() as i32)
                .collect_vec()
        })
        .collect()
}

fn part1(risk_map: &RiskMap) -> AocResult<i32> {
    solve(risk_map)
}

fn part2(risk_map: &RiskMap) -> AocResult<i32> {
    let tiled_grid = tile_map(risk_map);
    solve(&tiled_grid)
}

fn solve(risk_map: &RiskMap) -> AocResult<i32> {
    let start = Point::zero();
    let destination = risk_map.point_iterator().map(|(p, _)| p).last().unwrap();

    // There is no global heuristic for this problem (one can't predict much about the risk levels
    // of points between here and the destination), but combined with the cost function below it
    // is consistent (monotone). This is equivalent to just running Dijkstra's shortest path, since
    // we want the best solution and not an approximate path.
    let result = astar::search(
        &start,
        |current| current.manhattan_distance(&destination) as u32,
        |current, _| {
            // We only count the risk level if we enter it. We'll never re-enter the starting point,
            // so its cost is set to zero.
            if *current == start {
                0
            } else {
                *risk_map.lookup(current).unwrap_or(&0) as u32
            }
        },
        |point| {
            point
                .neighbors4()
                .into_iter()
                .filter(|p| risk_map.lookup(p).is_some())
        },
    );

    match result {
        AStarResult::Success(_, cost) => Ok(cost as i32),
        AStarResult::Failed => bail!("Failed to find a path"),
    }
}

fn tile_map(risk_map: &RiskMap) -> RiskMap {
    let Point { x: max_x, y: max_y } = risk_map.point_iterator().map(|(p, _)| p).last().unwrap();
    let tile_size_x = max_x + 1;
    let tile_size_y = max_y + 1;
    let mut rows = Vec::new();
    for _ in 0..(tile_size_y * 5) {
        rows.push(vec![0; (tile_size_x * 5) as usize]);
    }

    let mut grid = Grid::new(rows);
    let all_points = grid.point_iterator().map(|(p, _)| p).collect_vec();
    for point in all_points {
        // Maps back to the original point in the original tile.
        let untiled_point = Point::new(point.x % tile_size_x, point.y % tile_size_y);

        // The number of tiles to the left+above this tile, to adjust the risk level.
        let tile = point.x / tile_size_x + point.y / tile_size_y as i32;

        // Shifts all values by 1 to conform to the 1-9 range.
        let new_risk_level = (risk_map.lookup(&untiled_point).unwrap() + tile - 1) % 9 + 1;

        grid.update(&point, new_risk_level);
    }

    grid
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE_INPUT: &str = r#"1163751742
1381373672
2136511328
3694931569
7463417111
1319128137
1359912421
3125421639
1293138521
2311944581"#;

    #[test]
    fn part1_sample() {
        let inputs = parse(SAMPLE_INPUT.lines().map(|l| l.to_string()).collect_vec());
        let risk_map = Grid::new(inputs);

        assert_eq!(40, part1(&risk_map).unwrap());
    }

    #[test]
    fn part2_sample() {
        let inputs = parse(SAMPLE_INPUT.lines().map(|l| l.to_string()).collect_vec());
        let risk_map = Grid::new(inputs);

        assert_eq!(315, part2(&risk_map).unwrap());
    }
}
