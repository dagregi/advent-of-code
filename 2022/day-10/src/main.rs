use std::{collections::BTreeMap, fs};

fn main() {
    let input = fs::read_to_string("./input.txt").unwrap();

    println!(
        "Part 1: {}\nPart 2:\n{}",
        part1(input.clone()),
        part2(input.clone())
    );
}

#[derive(Debug)]
enum Instructions {
    Noop,
    Add(i32),
}
impl Instructions {
    fn cycles(&self) -> u32 {
        match self {
            Noop => 1,
            Add(_) => 2,
        }
    }
}
use Instructions::*;

fn parse_instructions(input: String) -> Vec<Instructions> {
    let vecs = input
        .lines()
        .map(|line| {
            if line.eq("noop") {
                Noop
            } else {
                let val = line.split(' ').collect::<Vec<_>>();
                Add(val.last().unwrap().parse::<i32>().unwrap())
            }
        })
        .collect::<Vec<Instructions>>();

    vecs
}

fn part1(input: String) -> String {
    let vecs = parse_instructions(input);
    let signals = [20, 60, 100, 140, 180, 220];
    let mut x: i32 = 1;
    let mut cycles: u32 = 1;
    let mut result: BTreeMap<u32, i32> = BTreeMap::new();

    for inst in vecs.iter() {
        match inst {
            Noop => cycles += 1,
            Add(val) => {
                cycles += 2;
                x += val;
            }
        }
        if signals.contains(&cycles) {
            result.insert(cycles, (cycles as i32) * x);
        }
        if signals.contains(&(cycles + 1)) {
            result.insert(cycles + 1, (cycles as i32 + 1) * x);
        }
    }
    result.values().sum::<i32>().to_string()
}

fn part2(input: String) -> String {
    let vecs = parse_instructions(input);
    let mut x: i32 = 1;
    let mut cycles: u32 = 0;
    let mut result: String = "".to_string();

    for inst in vecs.iter() {
        for cycle in 0..inst.cycles() {
            let pixel = (cycles as i32 + cycle as i32) % 40;
            if ((x - 1)..=(x + 1)).contains(&pixel) {
                result.push('#');
            } else {
                result.push('.');
            }
        }
        cycles += inst.cycles();
        match inst {
            Noop => {}
            Add(val) => {
                x += val;
            }
        }
    }
    result
}
