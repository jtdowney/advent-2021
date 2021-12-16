use eyre::eyre;
use nom::{
    bits,
    bits::complete::{tag, take},
    branch::alt,
    bytes::complete::take_while_m_n,
    combinator::{map, map_res, verify},
    multi::{length_count, many1, many_till},
    sequence::preceded,
    Finish, IResult,
};

type BitSlice<'a> = (&'a [u8], usize);

#[derive(Debug, Clone)]
enum Packet {
    Literal {
        version: u8,
        value: u64,
    },
    Operator {
        version: u8,
        operation: Operation,
        packets: Vec<Packet>,
    },
}

#[derive(Debug, Clone, Copy)]
enum Operation {
    Sum,
    Product,
    Minimum,
    Maximum,
    GreaterThan,
    LessThan,
    EqualTo,
}

fn hex_digit(input: &str) -> IResult<&str, u8> {
    map_res(take_while_m_n(2, 2, |c: char| c.is_digit(16)), |s: &str| {
        u8::from_str_radix(s, 16)
    })(input)
}

fn hex_string(input: &str) -> IResult<&str, Vec<u8>> {
    many1(hex_digit)(input)
}

struct PacketHeader {
    version: u8,
    packet_type: u8,
}

fn header(input: BitSlice) -> IResult<BitSlice, PacketHeader> {
    let (input, version) = take(3usize)(input)?;
    let (input, packet_type) = take(3usize)(input)?;

    Ok((
        input,
        PacketHeader {
            version,
            packet_type,
        },
    ))
}

fn variable_length_value(input: BitSlice) -> IResult<BitSlice, u64> {
    let nibble = |input| take::<_, u8, _, _>(4usize)(input);
    let (input, nibbles) = map(
        many_till(
            preceded(tag(1, 1usize), nibble),
            preceded(tag(0, 1usize), nibble),
        ),
        |(mut parts, part)| {
            parts.push(part);
            parts
        },
    )(input)?;

    let value = nibbles
        .iter()
        .rev()
        .enumerate()
        .fold(0, |acc, (offset, &nibble)| {
            acc | (nibble as u64) << (offset * 4)
        });

    Ok((input, value))
}

fn literal_packet(input: BitSlice) -> IResult<BitSlice, Packet> {
    let (input, header) = verify(header, |header| header.packet_type == 4)(input)?;
    let (input, value) = variable_length_value(input)?;

    let version = header.version;
    Ok((input, Packet::Literal { version, value }))
}

fn consumed_length(a: BitSlice, b: BitSlice) -> usize {
    let (input_a, index_a) = a;
    let (input_b, index_b) = b;
    (input_a.len() * 8 - index_a) - (input_b.len() * 8 - index_b)
}

fn operator_subpackets(input: BitSlice) -> IResult<BitSlice, Vec<Packet>> {
    let subpackets_by_length = preceded(tag(0, 1usize), |input| {
        let (input, length) = take(15usize)(input)?;

        let mut packets = vec![];
        let mut remaining_input = input;
        let mut consumed = 0;
        while consumed < length {
            let (next_input, packet) = packet(remaining_input)?;

            consumed += consumed_length(remaining_input, next_input);
            remaining_input = next_input;

            packets.push(packet);
        }

        Ok((remaining_input, packets))
    });

    let subpackets_by_count = preceded(
        tag(1, 1usize),
        length_count(take::<_, u16, _, _>(11usize), packet),
    );

    alt((subpackets_by_length, subpackets_by_count))(input)
}

fn operator_packet(input: BitSlice) -> IResult<BitSlice, Packet> {
    let (input, header) = verify(header, |header| header.packet_type != 4)(input)?;
    let (input, packets) = operator_subpackets(input)?;

    let version = header.version;
    let operation = match header.packet_type {
        0 => Operation::Sum,
        1 => Operation::Product,
        2 => Operation::Minimum,
        3 => Operation::Maximum,
        5 => Operation::GreaterThan,
        6 => Operation::LessThan,
        7 => Operation::EqualTo,
        _ => unreachable!(),
    };

    Ok((
        input,
        Packet::Operator {
            version,
            operation,
            packets,
        },
    ))
}

fn packet(input: BitSlice) -> IResult<BitSlice, Packet> {
    alt((literal_packet, operator_packet))(input)
}

fn parse_packet(input: &[u8]) -> IResult<&[u8], Packet> {
    bits(packet)(input)
}

#[aoc_generator(day16)]
fn generator(input: &str) -> eyre::Result<Packet> {
    let (_, data) = hex_string(input)
        .finish()
        .map_err(|_| eyre!("unable to parse hex"))?;

    let (_, packet) = parse_packet(&data)
        .finish()
        .map_err(|_| eyre!("unable to parse packets"))?;

    Ok(packet)
}

#[aoc(day16, part1)]
fn part1(input: &Packet) -> usize {
    let mut search = vec![input];
    let mut count = 0;

    while let Some(packet) = search.pop() {
        match packet {
            Packet::Literal { version, .. } => count += *version as usize,
            Packet::Operator {
                version, packets, ..
            } => {
                count += *version as usize;
                search.extend(packets);
            }
        }
    }

    count
}

fn evaluate(packet: &Packet) -> u64 {
    match packet {
        Packet::Literal { value, .. } => *value,
        Packet::Operator {
            operation, packets, ..
        } => match operation {
            Operation::Sum => packets.iter().map(evaluate).sum(),
            Operation::Product => packets.iter().map(evaluate).product(),
            Operation::Minimum => packets.iter().map(evaluate).min().unwrap_or_default(),
            Operation::Maximum => packets.iter().map(evaluate).max().unwrap_or_default(),
            Operation::GreaterThan => {
                let left = evaluate(&packets[0]);
                let right = evaluate(&packets[1]);
                if left > right {
                    1
                } else {
                    0
                }
            }
            Operation::LessThan => {
                let left = evaluate(&packets[0]);
                let right = evaluate(&packets[1]);
                if left < right {
                    1
                } else {
                    0
                }
            }
            Operation::EqualTo => {
                let left = evaluate(&packets[0]);
                let right = evaluate(&packets[1]);
                if left == right {
                    1
                } else {
                    0
                }
            }
        },
    }
}

#[aoc(day16, part2)]
fn part2(input: &Packet) -> u64 {
    evaluate(input)
}
