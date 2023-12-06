use crate::day13::Outcome::{Continue, OutOfOrder};
use itertools::{concat, EitherOrBoth, Itertools};
use nom::{
    branch::alt,
    character::complete::char,
    character::complete::digit1,
    character::streaming::space0,
    combinator::cut,
    combinator::{map, map_res},
    error::context,
    multi::separated_list0,
    sequence::preceded,
    sequence::terminated,
    IResult,
};
use std::cmp::Ordering;
use std::fmt::{Debug, Display, Formatter};
use Outcome::InOrder;

#[derive(Clone, Debug, Eq, PartialEq)]
enum PacketData {
    List(Vec<PacketData>),
    Int(i32),
}

impl Display for PacketData {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            PacketData::List(l) => write!(f, "[{}]", l.iter().join(",")),
            PacketData::Int(v) => write!(f, "{}", v),
        }
    }
}

#[derive(Clone, Debug)]
struct PacketPair {
    left: PacketData,
    right: PacketData,
}

#[derive(Copy, Clone, Debug)]
enum Outcome {
    InOrder,
    OutOfOrder,
    Continue,
}

impl Outcome {
    fn is_terminal(&self) -> bool {
        !matches!(self, Outcome::Continue)
    }

    fn is_ordered(&self) -> bool {
        matches!(self, Outcome::InOrder)
    }

    fn as_ordering(&self) -> Ordering {
        match self {
            InOrder => Ordering::Less,
            OutOfOrder => Ordering::Greater,
            Continue => Ordering::Equal,
        }
    }
}

#[aoc_generator(day13)]
fn parse(input: &str) -> Vec<PacketPair> {
    input.split("\n\n").map(parse_pair).collect()
}

fn parse_pair(input: &str) -> PacketPair {
    let vec = input
        .lines()
        .map(|i| {
            let (_, packet) = parse_packet(i).unwrap();
            packet
        })
        .collect_vec();
    PacketPair {
        left: vec[0].clone(),
        right: vec[1].clone(),
    }
}

fn parse_packet(i: &str) -> IResult<&str, PacketData> {
    let (i, v) = value(i)?;

    Ok((i, v))
}

fn value(i: &str) -> IResult<&str, PacketData> {
    context(
        "value",
        preceded(
            space0,
            alt((
                map(parse_i32, PacketData::Int),
                map(array, PacketData::List),
            )),
        ),
    )(i)
}

fn array(i: &str) -> IResult<&str, Vec<PacketData>> {
    context(
        "array",
        preceded(
            char('['),
            cut(terminated(
                separated_list0(preceded(space0, char(',')), value),
                preceded(space0, char(']')),
            )),
        ),
    )(i)
}

fn parse_i32(i: &str) -> IResult<&str, i32> {
    map_res(digit1, |s: &str| s.parse::<i32>())(i)
}

#[aoc(day13, part1)]
fn part1(input: &[PacketPair]) -> usize {
    input
        .iter()
        .enumerate()
        .filter_map(|(index, pair)| {
            in_order(&pair.left, &pair.right)
                .is_ordered()
                .then_some(index + 1)
        })
        .sum()
}

#[aoc(day13, part2)]
fn part2(input: &[PacketPair]) -> usize {
    let divider_packets = [
        PacketData::List(vec![PacketData::List(vec![PacketData::Int(2)])]),
        PacketData::List(vec![PacketData::List(vec![PacketData::Int(6)])]),
    ];
    let all_packets = input
        .iter()
        .flat_map(|pair| vec![pair.left.clone(), pair.right.clone()].into_iter())
        .chain(divider_packets.iter().cloned())
        .sorted_by(|l, r| in_order(l, r).as_ordering())
        .collect_vec();

    all_packets
        .iter()
        .enumerate()
        .filter_map(|(index, packet)| divider_packets.contains(packet).then_some(index + 1))
        .product()
}

fn in_order(left: &PacketData, right: &PacketData) -> Outcome {
    match (left, right) {
        o @ (PacketData::List(_), PacketData::Int(_)) => {
            in_order(o.0, &PacketData::List(vec![o.1.clone()]))
        }
        o @ (PacketData::Int(_), PacketData::List(_)) => {
            in_order(&PacketData::List(vec![o.0.clone()]), o.1)
        }
        (PacketData::List(l), PacketData::List(r)) => {
            // If both values are lists, compare the first value of each list, then the second value,
            // and so on. If the left list runs out of items first, the inputs are in the right
            // order. If the right list runs out of items first, the inputs are not in the right
            // order. If the lists are the same length and no comparison makes a decision about the
            // order, continue checking the next part of the input.
            for pair in l.iter().zip_longest(r.iter()) {
                let result = match pair {
                    EitherOrBoth::Both(l, r) => in_order(l, r),
                    EitherOrBoth::Left(_) => OutOfOrder,
                    EitherOrBoth::Right(_) => InOrder,
                };

                if result.is_terminal() {
                    return result;
                }
            }

            Continue
        }
        (PacketData::Int(l), PacketData::Int(r)) => match l.cmp(r) {
            Ordering::Less => InOrder,
            Ordering::Equal => Continue,
            Ordering::Greater => OutOfOrder,
        },
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    const INPUT: &str = "[1,1,3,1,1]
[1,1,5,1,1]

[[1],[2,3,4]]
[[1],4]

[9]
[[8,7,6]]

[[4,4],4,4]
[[4,4],4,4,4]

[7,7,7,7]
[7,7,7]

[]
[3]

[[[]]]
[[]]

[1,[2,[3,[4,[5,6,7]]]],8,9]
[1,[2,[3,[4,[5,6,0]]]],8,9]";

    #[test]
    fn p1() {
        assert_eq!(13, part1(&parse(INPUT)));
    }

    #[test]
    fn p2() {
        assert_eq!(140, part2(&parse(INPUT)));
    }
}
