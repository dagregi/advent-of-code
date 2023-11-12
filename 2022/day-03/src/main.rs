#![feature(iter_array_chunks)]

use std::{collections::HashMap, fs};

fn main() {
    let input = fs::read_to_string("./input.txt").unwrap();
    let scores = ('a'..='z')
        .chain('A'..='Z')
        .enumerate()
        .map(|(idx, c)| (c, idx + 1))
        .collect::<HashMap<char, usize>>();

    let sol_1 = part1(input.as_str(), scores);
    let sol_2 = part1(input.as_str(), scores);

    println!("Part 1: {}\nPart 2: {}", sol_1, sol_2);
}

fn part1(input: &str, scores: HashMap<char, usize>) -> String {
    input
        .lines()
        .map(|line| {
            let comp_size = line.len() / 2;
            let comp_a = &line[0..comp_size];
            let comp_b = &line[comp_size..(comp_size * 2)];
            let common = comp_a.chars().find(|c| comp_b.contains(*c)).unwrap();
            scores.get(&common).unwrap()
        })
        .sum::<usize>()
        .to_string()
}

fn part2(input: &str, scores: HashMap<char, usize>) -> String {
    input
        .lines()
        .array_chunks::<3>()
        .map(|[a, b, c]| {
            let common = a
                .chars()
                .find(|ch| b.contains(*ch) && c.contains(*ch))
                .unwrap();
            scores.get(&common).unwrap()
        })
        .sum::<usize>()
        .to_string()
}
