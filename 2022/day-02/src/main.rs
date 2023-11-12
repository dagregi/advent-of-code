// A == rock == 1pt
// B == paper == 2pts
// C == scissors == 3pts
//
// 0pt for lose
// 3pts for draw
// 6pts for win

use std::fs;

fn main() {
    let input = fs::read_to_string("./input.txt").unwrap();
    println!(
        "Part 1: {}\nPart 2: {}",
        part1(input.as_str()),
        part2(input.as_str())
    );
}

fn part1(input: &str) -> String {
    let outcome: Vec<&str> = input.lines().collect();
    let mut pts = Vec::new();

    for el in &outcome {
        match *el {
            "B X" => pts.push(1),
            "C Y" => pts.push(2),
            "A Z" => pts.push(3),
            "A X" => pts.push(4),
            "B Y" => pts.push(5),
            "C Z" => pts.push(6),
            "C X" => pts.push(7),
            "A Y" => pts.push(8),
            "B Z" => pts.push(9),
            &_ => println!("None"),
        }
    }
    pts.iter().sum::<u32>().to_string()
}

fn part2(input: &str) -> String {
    let outcome: Vec<&str> = input.lines().collect();
    let mut pts = Vec::new();

    for el in &outcome {
        match *el {
            "B X" => pts.push(1),
            "C Y" => pts.push(6),
            "A Z" => pts.push(8),
            "A X" => pts.push(2),
            "B Y" => pts.push(5),
            "C Z" => pts.push(7),
            "C X" => pts.push(3),
            "A Y" => pts.push(4),
            "B Z" => pts.push(9),
            &_ => println!("None"),
        }
    }
    pts.iter().sum::<u32>().to_string()
}
