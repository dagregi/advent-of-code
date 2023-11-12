use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{self, newline},
    multi::{separated_list0, separated_list1},
    sequence::{delimited, separated_pair},
    IResult, Parser,
};

fn main() {
    let input = include_str!("../input.txt");
    println!("Part 1: {}\nPart 2: {}", part1(input), part2(input));
}

#[derive(Debug)]
struct Pair {
    left: Packet,
    right: Packet,
}

#[derive(Debug, Eq, PartialEq)]
enum Packet {
    List(Vec<Packet>),
    Number(u32),
}

impl Ord for Packet {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        match (self, other) {
            (Packet::List(a), Packet::List(b)) => a.cmp(b),
            (Packet::List(a), Packet::Number(b)) => a.cmp(&vec![Packet::Number(*b)]),
            (Packet::Number(a), Packet::List(b)) => vec![Packet::Number(*a)].cmp(b),
            (Packet::Number(a), Packet::Number(b)) => a.cmp(b),
        }
    }
}

impl PartialOrd for Packet {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

fn packet_parser(input: &str) -> IResult<&str, Packet> {
    alt((
        delimited(tag("["), separated_list0(tag(","), packet_parser), tag("]")).map(Packet::List),
        complete::u32.map(Packet::Number),
    ))(input)
}
fn pairs(input: &str) -> IResult<&str, Vec<Pair>> {
    separated_list1(
        tag("\n\n"),
        separated_pair(packet_parser, newline, packet_parser)
            .map(|(left, right)| Pair { left, right }),
    )(input)
}

fn part1(input: &str) -> String {
    let (_, pairs) = pairs(input).unwrap();
    pairs
        .iter()
        .enumerate()
        .filter_map(
            |(i, Pair { left, right })| {
                if left.lt(right) {
                    Some(i)
                } else {
                    None
                }
            },
        )
        .map(|v| v + 1)
        .sum::<usize>()
        .to_string()
}

fn part2(input: &str) -> String {
    let (_, pairs) = pairs(input).unwrap();
    let p2 = Packet::List(vec![Packet::List(vec![Packet::Number(2)])]);
    let p6 = Packet::List(vec![Packet::List(vec![Packet::Number(6)])]);
    let mut packets: Vec<&Packet> = pairs
        .iter()
        .flat_map(|Pair { left, right }| [left, right])
        .chain([&p2, &p6])
        .collect();
    packets.sort();

    let idx_2 = packets
        .iter()
        .enumerate()
        .find(|(_i, packet)| packet == &&&p2)
        .unwrap();
    let idx_6 = packets
        .iter()
        .enumerate()
        .find(|(_i, packet)| packet == &&&p6)
        .unwrap();

    ((idx_2.0 + 1) * (idx_6.0 + 1)).to_string()
}
