use std::collections::hash_set::HashSet;
use std::str::FromStr;
use util::aoc::*;

fn main() -> Result<(), Box<std::error::Error>> {
    let lines: Vec<String> = input::read(1)?;
    let module_masses = lines.parse::<i32>()?;

    result("Part 1", || part1(&module_masses));
    result("Part 2", || part2(&module_masses));

    Ok(())
}

fn part1(module_masses: &[i32]) -> i32 {
    module_masses
        .iter()
        .map(|mass| calculate_fuel(*mass).unwrap_or(0))
        .sum()
}

fn part2(module_masses: &[i32]) -> i32 {
    module_masses
        .iter()
        .map(|mass| calculate_fuel_until_zero(*mass))
        .sum()
}

fn calculate_fuel(mass: i32) -> Option<i32> {
    let fuel_mass = mass / 3 - 2;
    if fuel_mass > 0 {
        Some(fuel_mass)
    } else {
        // Wish hard.
        None
    }
}

fn calculate_fuel_until_zero(mass: i32) -> i32 {
    let mut total_fuel_mass = 0;
    let mut last_fuel_mass = mass;
    while let Some(fuel_mass) = calculate_fuel(last_fuel_mass) {
        total_fuel_mass += fuel_mass;
        last_fuel_mass = fuel_mass;
    }
    total_fuel_mass
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_calculate_fuel() {
        assert_eq!(654, calculate_fuel(1969));
        assert_eq!(33583, calculate_fuel(100756));
    }
}
