use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{self, multispace1, newline},
    multi::separated_list1,
    sequence::separated_pair,
    IResult,
};

fn main() {
    let input = include_str!("../inputs/4.txt");

    let (stt, cards) = input_parser(input).unwrap();
    dbg!(stt, cards);
}

#[derive(Debug)]
struct Card {
    card_no: u32,
    winning_set: Vec<u32>,
    my_set: Vec<u32>,
}

fn input_parser(input: &str) -> IResult<&str, Vec<Card>> {
    let (input, cards) = separated_list1(newline, card_parser)(input)?;

    Ok((input, cards))
}

fn card_parser(input: &str) -> IResult<&str, Card> {
    let (input, _) = tag("Card ")(input)?;
    let (input, card_no) = complete::u32(input)?;
    let (input, _) = tag(": ")(input)?;
    let (input, (winning_set, my_set)) = separated_pair(
        separated_list1(multispace1, complete::u32),
        tag(" | "),
        separated_list1(multispace1, complete::u32),
    )(input)?;

    Ok((
        input,
        Card {
            card_no,
            winning_set,
            my_set,
        },
    ))
}
