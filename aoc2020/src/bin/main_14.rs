use failure::{bail, format_err};
use itertools::Itertools;
use lazy_static::lazy_static;
use regex::Regex;
use std::collections::HashMap;
use std::str::FromStr;
use util::aoc::*;

type MemoryWrite = (usize, i64);

lazy_static! {
    static ref RE: Regex = Regex::new("mem\\[(\\d+)\\] = (\\d+)").unwrap();
}

const MASK_BITS: usize = 36;

fn main() -> AocResult<()> {
    let input = input::read_all(14)?
        .split("mask = ")
        .filter(|s| !s.is_empty())
        .map(|s| s.parse())
        .collect::<AocResult<Vec<Stanza>>>()?;

    result("Part 1", || part1(&input))?;
    result("Part 2", || part2(&input))?;

    Ok(())
}

fn part1(stanzas: &[Stanza]) -> AocResult<i64> {
    let mut memory: HashMap<usize, i64> = HashMap::new();

    stanzas.iter().for_each(|s| {
        s.writes.iter().for_each(|(address, value)| {
            let value_masked = mask_value(&s.mask, value);
            memory.insert(*address, value_masked);
        });
    });

    Ok(memory.values().filter(|&&v| v > 0).copied().sum())
}

fn part2(stanzas: &[Stanza]) -> AocResult<i64> {
    let mut memory: HashMap<i64, i64> = HashMap::new();

    stanzas.iter().for_each(|s| {
        s.writes.iter().for_each(|&(address, value)| {
            translate_address(&s.mask, address)
                .iter()
                .for_each(|&translated_address| {
                    if value == 0 {
                        memory.remove(&translated_address);
                    } else {
                        memory.insert(translated_address, value);
                    }
                });
        });
    });

    Ok(memory.values().filter(|&&v| v > 0).copied().sum())
}

#[inline]
fn mask_value(mask: &[Option<u8>], value: &i64) -> i64 {
    let mut x = *value;
    for (idx, mask_bit) in mask.iter().enumerate().take(36) {
        match mask_bit {
            Some(0u8) => x &= !(1 << idx),
            Some(1u8) => x |= 1 << idx,
            _ => {}
        }
    }

    x
}

#[inline]
pub fn translate_address(mask: &[Option<u8>], address: usize) -> Vec<i64> {
    let mut addresses = Vec::new();

    let mut base_address = address as i64;
    for (idx, mask_bit) in mask.iter().enumerate().take(MASK_BITS) {
        if let Some(1u8) = mask_bit {
            base_address |= 1 << idx
        }
    }
    addresses.push(base_address);
    for (idx, mask_bit) in mask.iter().enumerate().take(MASK_BITS) {
        if mask_bit.is_none() {
            for i in 0..addresses.len() {
                addresses.push(addresses[i] & !(1 << idx));
                addresses.push(addresses[i] | 1 << idx);
            }
        }
    }

    addresses
}

#[derive(Debug)]
struct Stanza {
    mask: Vec<Option<u8>>,
    writes: Vec<MemoryWrite>,
}

impl FromStr for Stanza {
    type Err = failure::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let lines = s.lines().collect_vec();

        let mask = lines[0]
            .chars()
            .map(|c| match c {
                '0' => Some(0u8),
                '1' => Some(1u8),
                _ => None,
            })
            .rev()
            .collect_vec();

        if mask.len() != MASK_BITS {
            bail!("bad mask (len: {}): {}", mask.len(), lines[0]);
        }

        let writes = lines[1..]
            .iter()
            .map(|&line| {
                let c = RE
                    .captures(line)
                    .ok_or_else(|| format_err!("failed to match: {}", line))?;

                Ok((c[1].parse()?, c[2].parse()?))
            })
            .collect::<AocResult<Vec<_>>>()?;

        Ok(Stanza { mask, writes })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn translate_address() {
        let stanza: Stanza = "\
                000000000000000000000000000000X1001X\n\
                mem[42] = 100"
            .parse()
            .unwrap();

        let translated_addresses = super::translate_address(&stanza.mask, 42);

        assert_eq!(translated_addresses, vec![58, 59, 26, 27])
    }

    #[test]
    fn translate_address2() {
        let stanza: Stanza = "\
                00000000000000000000000000000000X0XX\n\
                mem[42] = 100"
            .parse()
            .unwrap();

        let translated_addresses = super::translate_address(&stanza.mask, 26);

        assert_eq!(translated_addresses, vec![26, 27, 24, 25, 18, 19, 16, 17])
    }
}
