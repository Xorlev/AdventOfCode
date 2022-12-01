use itertools::Itertools;
use util::aoc::*;
use util::aoc::top_k::TopK;

fn main() -> AocResult<()> {
    let lines: Vec<String> = input::read(1)?;
    let carried_calories: Vec<Vec<i32>> = lines.iter().batching(|line_iter| {
        let elf_calories: Vec<i32> = line_iter.take_while(|line| !line.is_empty()).map(|line| line.parse().unwrap()).collect();
        if elf_calories.len() > 0 {
            Some(elf_calories)
        } else {
            None
        }
    }).collect();

    result("Part 1", || part1(&carried_calories));
    result("Part 2", || part2(&carried_calories));

    Ok(())
}

fn part1(carried_calories: &[Vec<i32>]) -> i32 {
    carried_calories
        .iter()
        .map(|food| food.iter().sum())
        .max().unwrap_or(-1)
}

fn part2(carried_calories: &[Vec<i32>]) -> i32 {
    carried_calories
        .iter()
        .map(|food| food.iter().sum())
        .topk(3).iter().sum()
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn calorie_counting_p1() {
        let carried_calories = vec![
            vec![1000, 2000, 3000],
            vec![4000],
            vec![5000, 6000],
            vec![7000, 8000, 9000],
            vec![10_000],
        ];

        assert_eq!(24000, part1(&carried_calories));
    }

    #[test]
    fn calorie_counting_p2() {
        let carried_calories = vec![
            vec![1000, 2000, 3000],
            vec![4000],
            vec![5000, 6000],
            vec![7000, 8000, 9000],
            vec![10_000],
        ];

        assert_eq!(45000, part2(&carried_calories));
    }
}
