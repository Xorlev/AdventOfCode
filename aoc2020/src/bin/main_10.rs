use failure::bail;
use itertools::{Itertools, MinMaxResult};
use std::collections::HashMap;
use util::aoc::*;

fn main() -> AocResult<()> {
    let adapters: Vec<i64> = input::read(10)?
        .iter()
        .map(|line| line.parse::<i64>().map_err(|e| e.into()))
        .collect::<AocResult<Vec<_>>>()?;

    result("Part 1", || part1(&adapters))?;
    result("Part 2", || part2(&adapters));

    Ok(())
}

fn part1(adapters: &[i64]) -> AocResult<i64> {
    let mut sorted_adapters = adapters.iter().cloned().sorted();
    sorted_adapters.push(adapters.iter().max().unwrap() + 3);
    let mut previous_jolts = 0;
    let mut one_jolt_diffs = 0;
    let mut three_jolt_diffs = 0;
    for adapter_jolts in sorted_adapters {
        if adapter_jolts - previous_jolts == 1 {
            one_jolt_diffs += 1;
        } else if adapter_jolts - previous_jolts == 3 {
            three_jolt_diffs += 1;
        } else {
            bail!(
                "Unexpected difference: {} - {}",
                adapter_jolts,
                previous_jolts
            );
        }

        previous_jolts = adapter_jolts;
    }

    Ok(one_jolt_diffs * three_jolt_diffs)
}

fn part2(adapters: &[i64]) -> i64 {
    fn recurse(
        subpath_counts: &mut HashMap<i64, i64>,
        adapters_remaining: &[i64],
        previous_adapter: i64,
    ) -> i64 {
        if adapters_remaining.is_empty() {
            return 1;
        }

        let mut paths = 0;
        for (idx, &adapter_jolts) in adapters_remaining.iter().enumerate() {
            if adapter_jolts - previous_adapter > 3 {
                break;
            } else {
                if let Some(subpath_count) = subpath_counts.get(&adapter_jolts) {
                    paths += subpath_count;
                } else {
                    let subpath_count = recurse(
                        subpath_counts,
                        &adapters_remaining[idx + 1..],
                        adapter_jolts,
                    );
                    subpath_counts.insert(adapter_jolts, subpath_count);
                    paths += subpath_count;
                }
            }
        }

        paths
    }

    let mut sorted_adapters = adapters.iter().cloned().sorted();
    sorted_adapters.push(adapters.iter().max().unwrap() + 3);

    // This could be the full DP solution, but memoization is so trivial here. :)
    let mut subpath_counts = HashMap::new();
    recurse(&mut subpath_counts, &sorted_adapters, 0)
}
