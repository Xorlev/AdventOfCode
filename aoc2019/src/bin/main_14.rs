use failure::Error;
use std::f32;
use util::aoc::*;

use std::collections::{HashMap, VecDeque};
use std::convert::{TryFrom, TryInto};
use std::ops::Mul;

const FUEL: Chemical = Chemical("FUEL");
const ORE: Chemical = Chemical("ORE");

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let vec = input::read(14)?;
    let mut reactions: Vec<Reaction> = vec
        .iter()
        .map(|f| f.as_str().try_into())
        .collect::<Result<Vec<_>, _>>()?;
    reactions.push(Reaction {
        inputs: Vec::new(),
        output: Reagent::new(ORE, 1),
    });

    result("Part 1", || part1(reactions.clone()));
    result("Part 2", || part2(reactions.clone()));

    Ok(())
}

fn part1(reactions: Vec<Reaction>) -> Result<i64, Error> {
    let reactant_to_reaction: HashMap<Chemical, Reaction> = reactions
        .iter()
        .map(|r| (r.output.chemical.clone(), r.clone()))
        .collect();

    compute_ore_for_fuel(&reactant_to_reaction, 1)
}

fn part2(reactions: Vec<Reaction>) -> Result<i64, Error> {
    let reactant_to_reaction: HashMap<Chemical, Reaction> = reactions
        .iter()
        .map(|r| (r.output.chemical.clone(), r.clone()))
        .collect();

    const TARGET: i64 = 1_000_000_000_000;

    // First compute the ore for 1 FUEL, we'll use this to estimate the bounds of our search.
    let ore_for_one_fuel = compute_ore_for_fuel(&reactant_to_reaction, 1)?;
    let mut lo = 0;
    let mut hi = TARGET / ore_for_one_fuel * 2;
    while lo <= hi {
        let mid = (hi + lo) / 2;
        let ore = compute_ore_for_fuel(&reactant_to_reaction, mid)?;

        if ore <= TARGET {
            lo = mid + 1;
        } else {
            hi = mid - 1;
        }
    }

    Ok(lo - 1)
}

fn compute_ore_for_fuel(
    reactant_to_reaction: &HashMap<Chemical, Reaction>,
    fuel: i64,
) -> Result<i64, Error> {
    let mut order_q = VecDeque::new();
    order_q.push_back(Reagent::new(FUEL, fuel));
    let mut storage = HashMap::new();
    let mut amounts_produced = HashMap::new();
    while let Some(order) = order_q.pop_front() {
        let reaction = reactant_to_reaction
            .get(&order.chemical)
            .expect(format!("Missing reaction to produce {:?}", order).as_str());
        let mut required_amt = order.amount;
        // Fill from storage.
        let chem_in_storage = storage.entry(order.chemical).or_insert(0);
        let amt_from_storage = std::cmp::min(required_amt, *chem_in_storage);
        *chem_in_storage -= amt_from_storage;
        required_amt -= amt_from_storage;

        if required_amt > 0 {
            // Run an integer number of batches, storing the excess.
            let batches = (required_amt as f32 / reaction.output.amount as f32).ceil() as i64;
            let amount_produced = batches * reaction.output.amount;

            // Issue orders for missing reagents.
            reaction
                .inputs
                .iter()
                .for_each(|input| order_q.push_back(input * batches));

            // Store excess.
            let extra = amount_produced - required_amt;
            *chem_in_storage += extra;

            // Log production.
            amounts_produced
                .entry(order.chemical)
                .and_modify(|e| *e += amount_produced)
                .or_insert(amount_produced);
        }
    }

    Ok(*amounts_produced.get(&ORE).unwrap_or(&0))
}

#[derive(Clone, Debug)]
struct Reaction<'a> {
    inputs: Vec<Reagent<'a>>,
    output: Reagent<'a>,
}

impl<'a> TryFrom<&'a str> for Reaction<'a> {
    type Error = Error;

    fn try_from(s: &'a str) -> Result<Self, Self::Error> {
        let parts: Vec<&str> = s.split(" => ").collect();
        let inputs: Vec<Reagent> = parts[0]
            .split(", ")
            .map(|s| s.try_into())
            .collect::<Result<Vec<_>, _>>()?;
        let output: Reagent = parts[1].try_into()?;

        Ok(Reaction { inputs, output })
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Hash)]
struct Reagent<'a> {
    chemical: Chemical<'a>,
    amount: i64,
}

impl Reagent<'_> {
    fn new(chemical: Chemical, amount: i64) -> Reagent {
        Reagent { chemical, amount }
    }

    fn with_amount(&self, amount: i64) -> Reagent {
        Reagent::new(self.chemical, amount)
    }
}

impl<'a> Mul<i64> for &'a Reagent<'a> {
    type Output = Reagent<'a>;

    fn mul(self, rhs: i64) -> Self::Output {
        self.with_amount(self.amount * rhs)
    }
}

impl<'a> TryFrom<&'a str> for Reagent<'a> {
    type Error = Error;

    fn try_from(s: &'a str) -> Result<Self, Self::Error> {
        let parts: Vec<&'a str> = s.split(" ").collect();

        Ok(Reagent {
            chemical: Chemical(parts[1]),
            amount: parts[0].parse()?,
        })
    }
}

#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
struct Chemical<'a>(&'a str);
