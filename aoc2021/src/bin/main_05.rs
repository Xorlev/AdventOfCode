use failure::bail;
use lazy_static::lazy_static;
use regex::Regex;
use util::aoc::frequency::FrequencyMap;
use util::aoc::lines::LineSegment;
use util::aoc::*;

lazy_static! {
    static ref RE: Regex = Regex::new("(\\d+),(\\d+) -> (\\d+),(\\d+)").unwrap();
}

fn main() -> AocResult<()> {
    let input: Vec<String> = input::read(5)?;
    let vents = parse(input)?;

    result("Part 1", || part1(&vents));
    result("Part 2", || part2(&vents));

    Ok(())
}

fn parse(input: Vec<String>) -> AocResult<Vec<LineSegment>> {
    input
        .iter()
        .map(|line| {
            if let Some(captures) = RE.captures(line.as_str()) {
                Ok(LineSegment::new(
                    Point::new(captures[1].trim().parse()?, captures[2].trim().parse()?),
                    Point::new(captures[3].trim().parse()?, captures[4].trim().parse()?),
                ))
            } else {
                bail!("Didn't match input: {}", line);
            }
        })
        .collect::<AocResult<_>>()
}

fn part1(vents: &[LineSegment]) -> i32 {
    solve(
        vents
            .iter()
            .filter(|vent| vent.start.x == vent.end.x || vent.start.y == vent.end.y),
    )
}

fn part2(vents: &[LineSegment]) -> i32 {
    solve(vents.iter())
}

fn solve<'a>(vents: impl Iterator<Item = &'a LineSegment>) -> i32 {
    let mut frequency = FrequencyMap::new();
    vents
        .flat_map(|vent| vent.point_iterator())
        .for_each(|point| frequency.add(point));

    frequency
        .entries()
        .filter(|(_, &overlaps)| overlaps > 1)
        .count() as i32
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE_INPUT: &str = r#"0,9 -> 5,9
8,0 -> 0,8
9,4 -> 3,4
2,2 -> 2,1
7,0 -> 7,4
6,4 -> 2,0
0,9 -> 2,9
3,4 -> 1,4
0,0 -> 8,8
5,5 -> 8,2"#;

    #[test]
    fn part1_sample() {
        let vents = parse(SAMPLE_INPUT.lines().map(|l| l.to_string()).collect()).unwrap();

        assert_eq!(5, part1(&vents));
    }

    #[test]
    fn part1_sample() {
        let vents = parse(SAMPLE_INPUT.lines().map(|l| l.to_string()).collect()).unwrap();

        assert_eq!(12, part2(&vents));
    }
}
