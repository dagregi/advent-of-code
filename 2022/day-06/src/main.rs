use std::{collections::BTreeSet, fs};

fn main() {
    let input = fs::read_to_string("./input.txt").unwrap();
    let size = 14;
    let char = input.chars().collect::<Vec<char>>();
    let sequence = char
        .windows(size)
        .enumerate()
        .find(|(_i, slice)| {
            let set = slice.iter().collect::<BTreeSet<&char>>();
            slice.len() == set.len()
        })
        .unwrap();
    println!("{}", sequence.0 + size);
}
