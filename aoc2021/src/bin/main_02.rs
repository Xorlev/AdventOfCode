use failure::bail;
use itertools::Itertools;
use std::str::FromStr;
use util::aoc::*;

fn main() -> AocResult<()> {
    let course: Vec<CourseStep> = input::read(2)?.parse()?;

    result("Part 1", || part1(&course));
    result("Part 2", || part2(&course));

    Ok(())
}

fn part1(course: &[CourseStep]) -> i32 {
    let mut x = 0;
    let mut z = 0;
    for step in course {
        match step {
            CourseStep::Forward(units) => x += units,
            CourseStep::Down(units) => z += units,
            CourseStep::Up(units) => z -= units,
        }
    }

    x * z
}

fn part2(course: &[CourseStep]) -> i32 {
    let mut aim = 0;
    let mut x = 0;
    let mut z = 0;
    for step in course {
        match step {
            CourseStep::Forward(units) => {
                x += units;
                z += aim * units;
            }
            CourseStep::Down(units) => aim += units,
            CourseStep::Up(units) => aim -= units,
        }
    }

    x * z
}

#[derive(Debug, Copy, Clone)]
enum CourseStep {
    Forward(i32),
    Down(i32),
    Up(i32),
}

impl CourseStep {}

impl FromStr for CourseStep {
    type Err = failure::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if let Some((action, units)) = s.split_once(" ") {
            let units = units.parse()?;
            Ok(match action {
                "forward" => CourseStep::Forward(units),
                "down" => CourseStep::Down(units),
                "up" => CourseStep::Up(units),
                _ => bail!("Bad value: {}", s),
            })
        } else {
            bail!("Bad value: {}", s)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn final_depth_and_position_p1() {
        let course = vec![
            "forward 5",
            "down 5",
            "forward 8",
            "up 3",
            "down 8",
            "forward 2",
        ];
        let course = course
            .iter()
            .map(|s| s.parse::<CourseStep>().unwrap())
            .collect_vec();

        assert_eq!(150, part1(&course));
    }

    #[test]
    fn final_depth_and_position_p2() {
        let course = vec![
            "forward 5",
            "down 5",
            "forward 8",
            "up 3",
            "down 8",
            "forward 2",
        ];
        let course = course
            .iter()
            .map(|s| s.parse::<CourseStep>().unwrap())
            .collect_vec();

        assert_eq!(900, part2(&course));
    }
}
