use failure::bail;
use itertools::Itertools;
use nom::bits::bits;
use nom::bits::streaming::take;
use nom::error::Error;
use nom::sequence::tuple;
use nom::IResult;
use std::num::ParseIntError;
use util::aoc::astar::AStarResult;
use util::aoc::grid::Grid;
use util::aoc::*;

#[derive(Debug, Clone)]
pub struct Packet {
    version: i32,
    value: PacketValue,
    sub_packets: Vec<Packet>,
}

/// Packets have a version number, and are either literal values (type 4) or operators.
/// Packets contain zero or more sub-packets
#[derive(Debug, Clone)]
pub enum PacketValue {
    Literal(i32),
    Operator, // TODO
}

fn main() -> AocResult<()> {
    let input: Packet = parse(input::read_all(16)?)?;

    result("Part 1", || part1(&input));

    Ok(())
}

fn part1(packet: &Packet) -> i32 {
    version_sum(packet)
}

fn version_sum(packet: &Packet) -> i32 {
    packet.version
        + packet
            .sub_packets
            .iter()
            .map(|sub_packet| version_sum(sub_packet))
            .sum::<i32>()
}

fn parse(encoded: String) -> AocResult<Packet> {
    let input = decode_hex(&encoded)?;
    parse_packet(&input)
}

fn parse_packet(input: &[u8]) -> AocResult<Packet> {
    let take_version_and_type = tuple((take(3usize), take(3usize)));

    let (remaining, (version, type_id)) =
        bits::<_, _, Error<(&[u8], usize)>, _, _>(take_version_and_type)(input)?;

    // Literal packets:
    // Literal value packets encode a single binary number. To do this, the binary number is padded
    // with leading zeroes until its length is a multiple of four bits, and then it is broken into
    // groups of four bits. Each group is prefixed by a 1 bit except the last group, which is
    // prefixed by a 0 bit.

    // Operator packets:
    // An operator packet contains one or more packets. To indicate which subsequent binary data
    // represents its sub-packets, an operator packet can use one of two modes indicated by the bit
    // immediately after the packet header; this is called the length type ID:
    //
    //  - If the length type ID is 0, then the next 15 bits are a number that represents the total
    //    length in bits of the sub-packets contained by this packet.
    //  - If the length type ID is 1, then the next 11 bits are a number that represents the number
    //    of sub-packets immediately contained by this packet.
    // Finally, after the length type ID bit and the 15-bit or 11-bit field, the sub-packets appear.

    let value = match type_id {
        4 => PacketValue::Literal(1),
        _ => PacketValue::Operator,
    };

    Ok(Packet {
        version,
        value,
        sub_packets: vec![],
    })
}

fn decode_hex(s: &str) -> Result<Vec<u8>, ParseIntError> {
    (0..s.len())
        .step_by(2)
        .map(|i| u8::from_str_radix(&s[i..i + 2], 16))
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_sample() {
        let input = parse("8A004A801A8002F478".to_string()).unwrap();
        println!("{:?}", input);

        assert_eq!(16, part1(&input));
    }
}
