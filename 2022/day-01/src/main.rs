use std::fs;

fn main() {
    let contents = fs::read_to_string("src/input.txt").unwrap();
    let mut result = contents
        .split("\n\n")
        .map(|elf_cal| {
            elf_cal
                .lines()
                .map(|cal| cal.parse::<u32>().unwrap())
                .sum::<u32>()
        })
        .collect::<Vec<_>>();
    result.sort_by(|a, b| b.cmp(a));
    let sum: u32 = result.iter().take(3).sum();

    println!("Part 1: {}\nPart 2: {}", result[0], sum);
}
