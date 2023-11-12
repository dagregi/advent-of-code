use std::{collections::HashSet, fs};

use itertools::Itertools;
use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{self, newline},
    multi::separated_list1,
    sequence::separated_pair,
    IResult, Parser,
};

fn main() {
    let input = fs::read_to_string("./input.txt").unwrap();
    let (_, move_set) = moves(input.as_str()).unwrap();
    let mut head = (0, 0);
    let mut tail = (0, 0);
    let mut tail_pos = HashSet::from([tail]);


    for head_move in move_set.iter() {
        match head_move {
            Directions::Up => head.1 += 1,
            Directions::Down => head.1 -= 1,
            Directions::Right => head.0 += 1,
            Directions::Left => head.0 -= 1,
        }
        let y_range = (head.1 - 1)..=(head.1 + 1);
        let x_range = (head.0 - 1)..=(head.0 + 1);
        let tail_connected = x_range
            .cartesian_product(y_range)
            .any(|tuple| tuple == tail);
        if !tail_connected {
            let mut new_tail = head;
            match head_move {
                Directions::Up => new_tail.1 -= 1,
                Directions::Down => new_tail.1 += 1,
                Directions::Right => new_tail.0 -= 1,
                Directions::Left => new_tail.0 += 1,
            }
            tail = new_tail;

            tail_pos.insert(new_tail);
        }
    }
    let part1 = tail_pos.len().to_string();

    println!("Part 1: {}\nPart 2: todo", part1);
}

#[derive(Debug, Clone, Copy)]
enum Directions {
    Up,
    Down,
    Left,
    Right,
}

fn direction(input: &str) -> IResult<&str, Directions> {
    let (input, dir) = alt((
        complete::char('U').map(|_| Directions::Up),
        complete::char('D').map(|_| Directions::Down),
        complete::char('L').map(|_| Directions::Left),
        complete::char('R').map(|_| Directions::Right),
    ))(input)?;
    Ok((input, dir))
}

fn moves(input: &str) -> IResult<&str, Vec<Directions>> {
    let (input, vecs) =
        separated_list1(newline, separated_pair(direction, tag(" "), complete::u32))(input)?;
    let vecs = vecs
        .iter()
        .flat_map(|(dir, repaet)| vec![*dir; *repaet as usize])
        .collect();
    Ok((input, vecs))
}
