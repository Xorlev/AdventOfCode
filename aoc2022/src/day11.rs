use itertools::Itertools;
use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::char,
    character::complete::{digit0, digit1, multispace0, space1},
    character::streaming::space0,
    combinator::{map, map_res},
    multi::separated_list1,
    sequence::preceded,
    sequence::{delimited, tuple},
    IResult,
};
use num::Integer;

#[derive(Clone)]
enum Value {
    Literal(i64),
    Old,
}

impl Value {
    fn apply(&self, old_value: i64) -> i64 {
        match self {
            Value::Literal(value) => *value,
            Value::Old => old_value,
        }
    }
}

#[derive(Clone)]
enum Operation {
    Add(Value),
    Mul(Value),
}

impl Operation {
    /// Applies the given operation mod c, e.g. a + b (mod c).
    fn mod_apply(&self, a: i64, c: i64) -> i64 {
        match self {
            Operation::Add(source) => {
                let b = source.apply(a);
                ((a % c) + (b % c)) % c
            }
            Operation::Mul(source) => {
                let b = source.apply(a);
                ((a % c) * (b % c)) % c
            }
        }
    }
}

#[derive(Clone)]
struct Monkey {
    items: Vec<i64>,
    operation: Operation,
    test_divisor: i64,
    true_monkey: usize,
    false_monkey: usize,
}

#[aoc_generator(day11)]
fn parse(input: &str) -> Vec<Monkey> {
    let monkey_inputs = input.split("\n\n").collect_vec();
    monkey_inputs
        .iter()
        .map(|&input| parse_monkey(input).unwrap().1)
        .collect()
}

fn parse_monkey(i: &str) -> IResult<&str, Monkey> {
    let (i, _) = tuple((tag("Monkey"), space0, digit0, char(':'), multispace0))(i)?;
    let (i, items) = preceded(
        tuple((multispace0, tag("Starting items: "))),
        separated_list1(delimited(multispace0, char(','), multispace0), parse_i64),
    )(i)?;
    let (i, operation) = preceded(
        tuple((multispace0, tag("Operation: new = old"), space0)),
        alt((
            map(tuple((char('+'), parse_source)), |(_, val)| {
                Operation::Add(val)
            }),
            map(tuple((char('*'), parse_source)), |(_, val)| {
                Operation::Mul(val)
            }),
        )),
    )(i)?;
    let (i, test_divisor) = preceded(
        tuple((multispace0, tag("Test: divisible by"), space0)),
        parse_i64,
    )(i)?;
    let (i, true_monkey) = preceded(
        tuple((multispace0, tag("If true: throw to monkey"), space0)),
        parse_usize,
    )(i)?;
    let (i, false_monkey) = preceded(
        tuple((multispace0, tag("If false: throw to monkey"), space0)),
        parse_usize,
    )(i)?;

    Ok((
        i,
        Monkey {
            items,
            operation,
            test_divisor,
            true_monkey,
            false_monkey,
        },
    ))
}

fn parse_source(i: &str) -> IResult<&str, Value> {
    preceded(
        space1,
        alt((
            map(parse_i64, Value::Literal),
            map(tag("old"), |_| Value::Old),
        )),
    )(i)
}

fn parse_i64(i: &str) -> IResult<&str, i64> {
    map_res(digit1, |s: &str| s.parse::<i64>())(i)
}

fn parse_usize(i: &str) -> IResult<&str, usize> {
    map_res(digit1, |s: &str| s.parse::<usize>())(i)
}

#[aoc(day11, part1)]
fn part1(input: &Vec<Monkey>) -> i64 {
    perform_monkey_business(|item| item / 3, 20, &mut input.clone())
        .iter()
        .sorted()
        .rev()
        .take(2)
        .product()
}

#[aoc(day11, part2)]
fn part2(input: &Vec<Monkey>) -> i64 {
    perform_monkey_business(|item| item, 10000, &mut input.clone())
        .iter()
        .sorted()
        .rev()
        .take(2)
        .product()
}

fn perform_monkey_business(
    item_fn: fn(i64) -> i64,
    rounds: usize,
    input: &mut Vec<Monkey>,
) -> Vec<i64> {
    let lcm = input
        .iter()
        .map(|monkey| monkey.test_divisor)
        .reduce(|acc, div| div.lcm(&acc))
        .unwrap();

    let mut inspect_count: Vec<i64> = vec![0; input.len()];
    for _ in 0..rounds {
        for i in 0..input.len() {
            let thrown_items: Vec<(usize, i64)> = {
                let monkey = &mut input[i];
                monkey
                    .items
                    .drain(..)
                    .map(|item| {
                        inspect_count[i] += 1;
                        let item = item_fn(monkey.operation.mod_apply(item, lcm));
                        if item % monkey.test_divisor == 0 {
                            (monkey.true_monkey, item)
                        } else {
                            (monkey.false_monkey, item)
                        }
                    })
                    .collect()
            };

            thrown_items
                .iter()
                .for_each(|(monkey, item)| input[*monkey].items.push(*item));
        }
    }

    inspect_count
}

#[cfg(test)]
mod tests {
    use super::*;
    const INPUT: &str = "Monkey 0:
  Starting items: 79, 98
  Operation: new = old * 19
  Test: divisible by 23
    If true: throw to monkey 2
    If false: throw to monkey 3

Monkey 1:
  Starting items: 54, 65, 75, 74
  Operation: new = old + 6
  Test: divisible by 19
    If true: throw to monkey 2
    If false: throw to monkey 0

Monkey 2:
  Starting items: 79, 60, 97
  Operation: new = old * old
  Test: divisible by 13
    If true: throw to monkey 1
    If false: throw to monkey 3

Monkey 3:
  Starting items: 74
  Operation: new = old + 3
  Test: divisible by 17
    If true: throw to monkey 0
    If false: throw to monkey 1";

    #[test]
    fn p1() {
        assert_eq!(10605, part1(&parse(INPUT)));
    }

    #[test]
    fn p2() {
        assert_eq!(2713310158, part2(&parse(INPUT)));
    }
}
