pub mod day1;
pub mod day2;

use std::fs;

fn main() {
    let day = 1;
    let input = fs::read_to_string(format!("inputs/{}.txt", day)).unwrap();

    let first = day2::first_part(&input);
    let second = day2::second_part(&input);

    println!("Part 1: {}\nPart 2: {}", first, second);
}
