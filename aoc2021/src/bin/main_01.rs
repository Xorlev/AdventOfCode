use itertools::Itertools;
use util::aoc::*;

fn main() -> AocResult<()> {
    let lines: Vec<String> = input::read(1)?;
    let depths = lines.parse::<i32>()?;

    result("Part 1", || part1(&depths));
    result("Part 2", || part2(&depths));

    Ok(())
}

fn part1(depths: &[i32]) -> i32 {
    depths
        .iter()
        .tuple_windows()
        .filter(|&(a, b)| a < b)
        .count() as i32
}

fn part2(depths: &[i32]) -> i32 {
    depths
        .iter()
        .tuple_windows()
        .filter(|&(a, b, c, d)| (a + b + c) < (b + c + d))
        .count() as i32
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn find_increasing_depths_p1() {
        let depths = vec![199, 200, 208, 210, 200, 207, 240, 269, 260, 263];

        assert_eq!(7, part1(&depths));
    }

    #[test]
    fn find_increasing_depths_p2() {
        let depths = vec![607, 618, 618, 617, 647, 716, 769, 792];

        assert_eq!(5, part2(&depths));
    }
}
