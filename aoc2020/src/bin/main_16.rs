use failure::format_err;
use itertools::Itertools;
use lazy_static::lazy_static;
use regex::Regex;
use std::collections::HashMap;
use std::ops::RangeInclusive;
use std::str::FromStr;
use util::aoc::*;

lazy_static! {
    static ref RE: Regex = Regex::new("([^:]+): (\\d+)-(\\d+) or (\\d+)-(\\d+)").unwrap();
}

fn main() -> AocResult<()> {
    let input = input::read_all(16)?;
    let sections = input.split("\n\n").collect_vec();
    let properties = sections[0]
        .lines()
        .map(|l| l.parse::<Property>().map_err(|e| e.into()))
        .collect::<AocResult<Vec<Property>>>()?;
    let my_ticket: Vec<i64> = sections[1].lines().collect_vec()[1]
        .split(",")
        .collect_vec()
        .parse()?;
    let nearby_tickets: Vec<Vec<i64>> = sections[2].lines().collect_vec()[1..]
        .iter()
        .map(|ticket| {
            ticket
                .split(",")
                .map(|v| v.parse::<i64>().map_err(|e| e.into()))
                .collect::<AocResult<Vec<i64>>>()
        })
        .collect::<AocResult<Vec<Vec<i64>>>>()?;

    result("Part 1", || part1(&properties, &nearby_tickets));
    result("Part 2", || part2(&properties, &my_ticket, &nearby_tickets))?;

    Ok(())
}

fn part1(properties: &[Property], nearby_tickets: &[Vec<i64>]) -> i64 {
    nearby_tickets
        .iter()
        .flat_map(|ticket| ticket.iter())
        .filter(|&&v| !properties.iter().any(|property| property.contains(v)))
        .sum()
}

fn part2(
    properties: &[Property],
    my_ticket: &[i64],
    nearby_tickets: &[Vec<i64>],
) -> AocResult<i64> {
    // Step 1: discard invalid tickets
    let valid_tickets = nearby_tickets
        .iter()
        .filter(|ticket| {
            ticket.iter().all(|&value| {
                // Ensure at least one value matches _a_ property.
                properties.iter().any(|property| property.contains(value))
            })
        })
        .cloned()
        .collect_vec();

    // Step 2: transpose the values such that each row is now a column.
    let by_column: Vec<Vec<i64>> = transpose(valid_tickets);

    // Step 3: figure out which row's values match which properties:
    let mut column_to_property_candidates: HashMap<usize, Vec<&str>> = by_column
        .iter()
        .enumerate()
        .map(|(column, column_values)| {
            let property_candidates = properties
                .iter()
                .filter(|property| column_values.iter().all(|&v| property.contains(v)))
                .map(|property| property.name.as_str())
                .collect::<Vec<_>>();

            (column, property_candidates)
        })
        .collect();

    // Step 4: Process of elimination. Properties can apply to multiple columns, but some columns
    // will only have a single candidate. Continue this process until all properties are assigned.
    let mut column_to_property: HashMap<usize, &str> = HashMap::new();
    while !column_to_property_candidates.is_empty() {
        let result = *column_to_property_candidates
            .iter()
            .find(|(_, properties)| properties.len() == 1)
            .map(|v| v.0)
            .ok_or_else(|| format_err!("No remaining candidate columns"))?;

        let property = column_to_property_candidates.get(&result).unwrap()[0];
        column_to_property.insert(result, property);
        column_to_property_candidates.remove(&result);
        column_to_property_candidates
            .values_mut()
            .for_each(|properties| {
                properties.retain(|&p| p != property);
            })
    }

    // Step 5: Then multiply the remaining values.
    let result = column_to_property
        .iter()
        .filter(|(_, name)| name.starts_with("departure"))
        .map(|(&column, _)| my_ticket[column as usize])
        .product();

    Ok(result)
}

#[derive(Debug)]
struct Property {
    name: String,
    bounds: Vec<RangeInclusive<i64>>,
}

impl Property {
    fn contains(&self, value: i64) -> bool {
        self.bounds.iter().any(|b| b.contains(&value))
    }
}

impl FromStr for Property {
    type Err = failure::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let c = RE
            .captures(s)
            .ok_or_else(|| format_err!("didn't match: {}", s))?;

        Ok(Property {
            name: c[1].to_string(),
            bounds: vec![
                RangeInclusive::new(c[2].parse::<i64>()?, c[3].parse::<i64>()?),
                RangeInclusive::new(c[4].parse::<i64>()?, c[5].parse::<i64>()?),
            ],
        })
    }
}

fn transpose<T>(v: Vec<Vec<T>>) -> Vec<Vec<T>>
where
    T: Clone,
{
    assert!(!v.is_empty());
    (0..v[0].len())
        .map(|i| v.iter().map(|inner| inner[i].clone()).collect::<Vec<T>>())
        .collect()
}
