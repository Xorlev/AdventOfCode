use failure::format_err;
use itertools::Itertools;
use nom::bits::bits;
use nom::bits::complete::take;
use nom::error::Error;
use nom::sequence::tuple;
use nom::{Err, IResult};
use std::num::ParseIntError;
use util::aoc::*;

type BitResult<'a, T> = IResult<(&'a [u8], usize), T>;

fn main() -> AocResult<()> {
    let input: Packet = parse(input::read_all(16)?)?;

    result("Part 1", || part1(&input));
    result("Part 2", || part2(&input));

    Ok(())
}

fn part1(packet: &Packet) -> i32 {
    version_sum(packet)
}

fn version_sum(packet: &Packet) -> i32 {
    packet.version + packet.sub_packets.iter().map(version_sum).sum::<i32>()
}

fn part2(packet: &Packet) -> i64 {
    packet.evaluate_packet().literal_value().unwrap_or(0)
}

#[derive(Debug, Clone)]
pub struct Packet {
    version: i32,
    value: PacketValue,
    sub_packets: Vec<Packet>,
}

impl Packet {
    fn evaluate_packet(&self) -> Packet {
        let mut evaluated_sub_packets = self
            .sub_packets
            .iter()
            .map(|p| p.evaluate_packet())
            .map(|p| p.literal_value().unwrap());

        let value = match self.value {
            PacketValue::Sum => evaluated_sub_packets.sum(),
            PacketValue::Product => evaluated_sub_packets.product(),
            PacketValue::Minimum => evaluated_sub_packets.min().unwrap(),
            PacketValue::Literal(value) => value,
            PacketValue::Maximum => evaluated_sub_packets.max().unwrap(),
            PacketValue::GreaterThan => {
                let packets = evaluated_sub_packets.collect_vec();
                one_if_true(packets[0] > packets[1])
            }
            PacketValue::LessThan => {
                let packets = evaluated_sub_packets.collect_vec();
                one_if_true(packets[0] < packets[1])
            }
            PacketValue::EqualTo => one_if_true(evaluated_sub_packets.all_equal()),
        };

        Packet {
            version: self.version,
            value: PacketValue::Literal(value),
            sub_packets: vec![],
        }
    }

    fn literal_value(&self) -> Option<i64> {
        if let PacketValue::Literal(value) = self.value {
            Some(value)
        } else {
            None
        }
    }
}

/// Packets have a version number, and are either literal values (type 4) or operators.
/// Packets contain zero or more sub-packets
#[derive(Debug, Clone)]
pub enum PacketValue {
    Sum,
    Product,
    Minimum,
    Literal(i64),
    Maximum,
    GreaterThan,
    LessThan,
    EqualTo,
}

enum SubPacketFrame {
    Count(usize),
    Bits(usize),
}

fn one_if_true(b: bool) -> i64 {
    if b {
        1
    } else {
        0
    }
}

fn parse(encoded: String) -> AocResult<Packet> {
    let input = decode_hex(&encoded)?;

    // TODO: Fix error handling
    let x = bits(parse_packet)(input.as_slice())
        .map(|(_, packet)| packet)
        .map_err(|_: Err<Error<_>>| format_err!("Failed to parse"));

    x
}

fn parse_packet(i: (&[u8], usize)) -> BitResult<Packet> {
    let (i, (version, type_id)) = tuple((take(3usize), take(3usize)))(i)?;
    let (mut i, (value, sub_packet_frame)) = match type_id {
        4 => parse_literal(i)?,
        _ => parse_operator(i, type_id)?,
    };

    let mut sub_packets = Vec::new();
    match sub_packet_frame {
        SubPacketFrame::Count(packets) => {
            for _ in 0..packets {
                let (new_i, packet) = parse_packet(i)?;
                i = new_i;
                sub_packets.push(packet);
            }
        }
        SubPacketFrame::Bits(bits_to_parse) => {
            let start = bits_remaining(i);
            while start - bits_remaining(i) < bits_to_parse {
                let (new_i, packet) = parse_packet(i)?;
                i = new_i;
                sub_packets.push(packet);
            }
        }
    }

    let packet = Packet {
        version,
        value,
        sub_packets,
    };

    Ok((i, packet))
}

fn bits_remaining((bytes, offset): (&[u8], usize)) -> usize {
    bytes.len() * 8 - offset
}

fn parse_literal(mut i: (&[u8], usize)) -> BitResult<(PacketValue, SubPacketFrame)> {
    // Literal packets:
    // Literal value packets encode a single binary number. To do this, the binary number is padded
    // with leading zeroes until its length is a multiple of four bits, and then it is broken into
    // groups of four bits. Each group is prefixed by a 1 bit except the last group, which is
    // prefixed by a 0 bit.
    // 110 100 10111 11110 00101 000
    // VVV TTT AAAAA BBBBB CCCCC
    // Below each bit is a label indicating its purpose:
    //
    // - The three bits labeled V (110) are the packet version, 6.
    // - The three bits labeled T (100) are the packet type ID, 4, which means the packet is a
    //   literal value.
    // - The five bits labeled A (10111) start with a 1 (not the last group, keep reading) and
    //   contain the first four bits of the number, 0111.
    // - The five bits labeled B (11110) start with a 1 (not the last group, keep reading) and
    //   contain four more bits of the number, 1110.
    // - The five bits labeled C (00101) start with a 0 (last group, end of packet) and contain the
    //   last four bits of the number, 0101.
    // - The three unlabeled 0 bits at the end are extra due to the hexadecimal representation and
    //   should be ignored.

    let mut number = 0;
    loop {
        let (new_i, (keep_reading, chunk)): (_, (i64, i64)) =
            tuple((take(1usize), take(4usize)))(i)?;
        number = (number << 4) | chunk;
        i = new_i;

        if keep_reading != 1 {
            break;
        }
    }

    Ok((i, (PacketValue::Literal(number), SubPacketFrame::Count(0))))
}

fn parse_operator(i: (&[u8], usize), type_id: i32) -> BitResult<(PacketValue, SubPacketFrame)> {
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

    let (i, mode): (_, i32) = take(1usize)(i)?;
    let (i, frame) = match mode {
        0 => {
            let (i, bits) = take(15usize)(i)?;
            (i, SubPacketFrame::Bits(bits))
        }
        1 => {
            let (i, count) = take(11usize)(i)?;
            (i, SubPacketFrame::Count(count))
        }
        _ => panic!("Unexpected mode: {}", mode),
    };

    let packet_value = match type_id {
        0 => PacketValue::Sum,
        1 => PacketValue::Product,
        2 => PacketValue::Minimum,
        3 => PacketValue::Maximum,
        5 => PacketValue::GreaterThan,
        6 => PacketValue::LessThan,
        7 => PacketValue::EqualTo,
        _ => panic!("Unexpected packet value"),
    };

    Ok((i, (packet_value, frame)))
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
        let input = parse("A0016C880162017C3686B18A3D4780".to_string()).unwrap();

        assert_eq!(31, part1(&input));
    }

    #[test]
    fn part2_sample() {
        let input = parse("04005AC33890".to_string()).unwrap();

        assert_eq!(54, part2(&input));
    }

    #[test]
    fn part2_sample_2() {
        let input = parse("9C0141080250320F1802104A08".to_string()).unwrap();

        assert_eq!(1, part2(&input));
    }
}
