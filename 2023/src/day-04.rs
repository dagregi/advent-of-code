use nom::{
    bytes::complete::tag,
    character::complete::{self, newline, space1},
    multi::separated_list1,
    sequence::{separated_pair, tuple},
    IResult,
};

fn main() {
    let input = include_str!("../inputs/4.txt");

    let (_, cards) = input_parser(input).unwrap();

    println!("part 1: {}\npart 2: {:?}", part1(cards), "part2");
}

#[derive(Debug)]
struct Card {
    winning_set: Vec<u32>,
    my_set: Vec<u32>,
}

impl Card {
    fn score(&self) -> u32 {
        let mut pow: u32 = 0;
        for val in self.winning_set.iter() {
            self.my_set.iter().for_each(|mine| {
                if mine == val {
                    pow += 1;
                }
            });
        }
        if pow == 0 {
            0
        } else {
            2u32.pow(pow - 1)
        }
    }
}

fn part1(cards: Vec<Card>) -> u32 {
    cards.iter().map(|vv| vv.score()).sum::<u32>()
}

fn input_parser(input: &str) -> IResult<&str, Vec<Card>> {
    separated_list1(newline, card_parser)(input)
}

fn card_parser(input: &str) -> IResult<&str, Card> {
    let (input, _) = tuple((tag("Card"), space1))(input)?;
    let (input, _) = complete::u32(input)?;
    let (input, _) = tuple((tag(":"), space1))(input)?;
    let (input, (winning_set, my_set)) = separated_pair(
        separated_list1(space1, complete::u32),
        tuple((tag(" |"), space1)),
        separated_list1(space1, complete::u32),
    )(input)?;

    Ok((
        input,
        Card {
            winning_set,
            my_set,
        },
    ))
}
