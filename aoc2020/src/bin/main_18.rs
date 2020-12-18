use pest::iterators::{Pair, Pairs};
use pest::prec_climber::{Assoc, Operator, PrecClimber};
use pest::Parser;
use pest_derive::Parser;
use util::aoc::*;

#[derive(Parser)]
#[grammar = "day18.pest"]
struct MathParser;

fn main() -> AocResult<()> {
    let lines = input::read(18)?;
    let input = lines
        .iter()
        .map(|expr| MathParser::parse(Rule::expr, expr).map_err(|e| e.into()))
        .collect::<AocResult<Vec<Pairs<Rule>>>>()?;

    result("Part 1", || part1(input.clone()));
    result("Part 2", || part2(input.clone()));

    Ok(())
}

fn part1(expressions: Vec<Pairs<Rule>>) -> i64 {
    let prec_climber: PrecClimber<Rule> = PrecClimber::new(vec![
            Operator::new(Rule::add, Assoc::Left) | Operator::new(Rule::multiply, Assoc::Left),
        ]);

    expressions
        .into_iter()
        .map(|e| eval(&prec_climber, e))
        .sum()
}

fn part2(expressions: Vec<Pairs<Rule>>) -> i64 {
    let prec_climber: PrecClimber<Rule> = PrecClimber::new(vec![
            Operator::new(Rule::multiply, Assoc::Left),
            Operator::new(Rule::add, Assoc::Left),
        ]);

    expressions
        .into_iter()
        .map(|e| eval(&prec_climber, e))
        .sum()
}

fn eval(prec_climber: &PrecClimber<Rule>, expression: Pairs<Rule>) -> i64 {
    prec_climber.climb(
        expression,
        |pair: Pair<Rule>| match pair.as_rule() {
            Rule::num => pair.as_str().parse::<i64>().unwrap(),
            Rule::expr => eval(prec_climber, pair.into_inner()),
            _ => unreachable!(),
        },
        |lhs: i64, op: Pair<Rule>, rhs: i64| match op.as_rule() {
            Rule::add => lhs + rhs,
            Rule::multiply => lhs * rhs,
            _ => unreachable!(),
        },
    )
}
