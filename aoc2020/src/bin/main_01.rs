use std::collections::hash_set::HashSet;
use std::str::FromStr;

use util::aoc::*;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let lines: Vec<String> = input::read(1)?;
    let expenses = lines.parse::<i32>()?;

    result("Part 1", || part1(&expenses));
    result("Part 2", || part2(&expenses));

    Ok(())
}

fn part1(expenses: &[i32]) -> i32 {
    find_expense_pair_summing_to(expenses, 2020)
        .map(|(a, b)| a * b)
        .unwrap()
}

fn part2(expenses: &[i32]) -> i32 {
    find_expense_triplet_summing_to(expenses, 2020)
        .map(|(a, b, c)| a * b * c)
        .unwrap()
}

fn find_expense_pair_summing_to(expenses: &[i32], target: i32) -> Option<(i32, i32)> {
    let available_expenses: HashSet<i32> = expenses.iter().cloned().collect();

    expenses
        .iter()
        .find(|&&expense| available_expenses.contains(&(target - expense)))
        .map(|&expense| (expense, target - expense))
}

fn find_expense_triplet_summing_to(expenses: &[i32], target: i32) -> Option<(i32, i32, i32)> {
    let available_expenses: HashSet<i32> = expenses.iter().cloned().collect();

    for a in expenses {
        for b in expenses {
            let target_expense = target - a - b;
            if available_expenses.contains(&target_expense) {
                return Some((*a, *b, target_expense));
            }
        }
    }
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn find_expense_pair_summing_to_value() {
        let expenses = vec![1721, 979, 366, 299, 675, 1456];

        assert_eq!(
            Some((1721, 299)),
            find_expense_pair_summing_to(&expenses, 2020)
        );
    }

    #[test]
    fn find_expense_triplet_summing_to_value() {
        let expenses = vec![1721, 979, 366, 299, 675, 1456];

        assert_eq!(
            Some((979, 366, 675)),
            find_expense_triplet_summing_to(&expenses, 2020)
        );
    }
}
