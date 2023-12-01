fn main() {
    // let input = include_str!("../inputs/1.txt");
    let input = "1abc2
pqr3stu8vwx
a1b2c3d4e5f
treb7uchet";

    let nums: Vec<_> = input
        .lines()
        .map(|ln| {
            ln.chars()
                .filter_map(|val| val.to_digit(10))
                .collect::<Vec<_>>()
        })
        .collect();
    // You are the dumbest person!!!
    let fsum: u32 = nums.iter().map(|v| v.first().unwrap()).sum();
    let lsum: u32 = nums.iter().map(|v| v.last().unwrap()).sum();
    println!("{} {}", fsum, lsum);
}
