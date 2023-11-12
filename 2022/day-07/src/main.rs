use nom::{
    branch::alt,
    bytes::complete::{is_a, tag},
    character::complete::{self, alpha1, newline},
    multi::separated_list1,
    sequence::separated_pair,
    IResult,
};
use std::{collections::BTreeMap, fs};

fn main() {
    let input = fs::read_to_string("./input.txt").unwrap();
    let cmds = commands(input.as_str()).unwrap();

    let (_, sizes) = cmds.1.iter().fold((vec![], BTreeMap::new()), calculate_size);

    let part1 = sizes
        .iter()
        .filter(|(_, &size)| size < 100_000)
        .map(|(_, size)| size)
        .sum::<u64>()
        .to_string();

    let total_size = 70_000_000;
    let needed_size = 30_000_000;
    let used_space = sizes.get(&vec![""]).unwrap();
    let free_space = total_size- needed_size;
    let freeable_space = free_space - needed_size;

    let mut valid_dirs = sizes
        .iter()
        .filter(|(_, &size)| size > freeable_space)
        .map(|(_, size)| size)
        .collect::<Vec<&u64>>();
    valid_dirs.sort();
    let part2 = valid_dirs.first().unwrap().to_string();

    println!("Part 1: {}\nPart 2: {}", part1, part2);
}

enum Operation<'a> {
    Cd(Cd<'a>),
    Ls(Vec<Files<'a>>),
}
enum Cd<'a> {
    Root,
    Up,
    Down(&'a str),
}
enum Files<'a> {
    File { size: u64, name: &'a str },
    Dir(&'a str),
}

fn file(input: &str) -> IResult<&str, Files> {
    let (input, (size, name)) =
        separated_pair(complete::u64, tag(" "), is_a("pyfgcrlaoeuidhtnsjkxbmwvqz."))(input)?;

    Ok((input, Files::File { size, name }))
}

fn directory(input: &str) -> IResult<&str, Files> {
    let (input, _) = tag("dir ")(input)?;
    let (input, name) = alpha1(input)?;

    Ok((input, Files::Dir(name)))
}

fn ls(input: &str) -> IResult<&str, Operation> {
    let (input, _) = tag("$ ls")(input)?;
    let (input, _) = newline(input)?;
    let (input, files) = separated_list1(newline, alt((file, directory)))(input)?;

    Ok((input, Operation::Ls(files)))
}

fn cd(input: &str) -> IResult<&str, Operation> {
    let (input, _) = tag("$ cd ")(input)?;
    let (input, dir) = alt((tag(".."), alpha1, tag("/")))(input)?;
    let op = match dir {
        "/" => Operation::Cd(Cd::Root),
        ".." => Operation::Cd(Cd::Up),
        name => Operation::Cd(Cd::Down(name)),
    };

    Ok((input, op))
}

fn commands(input: &str) -> IResult<&str, Vec<Operation>> {
    let (input, cmd) = separated_list1(newline, alt((ls, cd)))(input)?;
    Ok((input, cmd))
}

fn calculate_size<'a>(
    (mut context, mut sizes): (Vec<&'a str>, BTreeMap<Vec<&'a str>, u64>),
    command: &'a Operation,
) -> (Vec<&'a str>, BTreeMap<Vec<&'a str>, u64>) {
    match command {
        Operation::Cd(Cd::Root) => {
            context.push("");
        }
        Operation::Cd(Cd::Up) => {
            context.pop();
        }
        Operation::Cd(Cd::Down(name)) => {
            context.push(name);
        }
        Operation::Ls(files) => {
            let sum = files
                .iter()
                .filter_map(|file| {
                    if let Files::File { size, .. } = file {
                        Some(size)
                    } else {
                        None
                    }
                })
                .sum::<u64>();

            for i in 0..context.len() {
                sizes
                    .entry(context[0..=i].to_vec())
                    .and_modify(|v| *v += sum)
                    .or_insert(sum);
            }
        }
    };
    (context, sizes)
}
