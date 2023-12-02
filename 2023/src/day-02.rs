use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{self, newline},
    multi::separated_list1,
    sequence::separated_pair,
    IResult,
};

fn main() {
    // let input = include_str!("../inputs/2.txt");
    let input = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green";

    let (_, games) = game_parser(input).unwrap();

    println!("part 1: {}\npart 2: {:?}", part1(games), "part2");
}

fn part1(games: Vec<Game>) -> u32 {
    let mut possible_ids = Vec::new();

    for game in games.iter() {
        let mut possible = Vec::new();
        for set in game.sets.iter() {
            match set {
                Set::Red(v) => {
                    if v < &13 {
                        possible.push(true);
                    } else {
                        possible.push(false);
                    }
                }
                Set::Green(v) => {
                    if v < &14 {
                        possible.push(true);
                    } else {
                        possible.push(false);
                    }
                }
                Set::Blue(v) => {
                    if v < &15 {
                        possible.push(true);
                    } else {
                        possible.push(false);
                    }
                }
            }
        }
        if !possible.contains(&false) {
            possible_ids.push(game.id);
        }
    }

    let part1: u32 = possible_ids.iter().sum();
    part1
}

#[derive(Debug)]
enum Set {
    Red(u32),
    Green(u32),
    Blue(u32),
}
#[derive(Debug)]
struct Game {
    id: u32,
    sets: Vec<Set>,
}

fn game_parser(input: &str) -> IResult<&str, Vec<Game>> {
    let (input, games) = separated_list1(newline, line_parser)(input)?;

    Ok((input, games))
}

fn line_parser(input: &str) -> IResult<&str, Game> {
    let (input, _) = tag("Game ")(input)?;
    let (input, id) = complete::u32(input)?;
    let (input, _) = tag(": ")(input)?;
    let (input, sets) = separated_list1(alt((tag("; "), tag(", "))), cube_parser)(input)?;

    Ok((input, Game { id, sets }))
}

fn cube_parser(input: &str) -> IResult<&str, Set> {
    let (input, (number, color)) = separated_pair(
        complete::u32,
        tag(" "),
        alt((tag("red"), tag("green"), tag("blue"))),
    )(input)?;
    let set = match color {
        "red" => Set::Red(number),
        "green" => Set::Green(number),
        "blue" => Set::Blue(number),
        _ => panic!("unkonwn color"),
    };

    Ok((input, set))
}
