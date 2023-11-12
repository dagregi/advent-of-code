#[derive(Debug)]
enum Direction {
    Forward(u32),
    Down(u32),
    Up(u32),
}
use Direction::*;

fn parse_moves(input: &str) -> Vec<Direction> {
    input.lines().map(directions).collect()
}

fn directions(ln: &str) -> Direction {
    let valve: Vec<&str> = ln.split(' ').collect();
    let dir = valve.first().unwrap();
    let val = valve.last().unwrap().parse::<u32>().unwrap();

    match *dir {
        "forward" => Forward(val),
        "down" => Down(val),
        "up" => Up(val),
        &_ => todo!(),
    }
}

pub fn part1(input: &str) -> String {
    let moves = parse_moves(input);
    let mut x_coor = 0;
    let mut y_coor = 0;

    for val in moves.iter() {
        match val {
            Forward(v) => x_coor += v,
            Down(v) => y_coor += v,
            Up(v) => y_coor -= v,
        }
    }
    dbg!(x_coor, y_coor);
    (x_coor * y_coor).to_string()
}

pub fn part2(input: &str) -> String {
    let moves = parse_moves(input);
    let mut x_coor = 0;
    let mut depth = 0;
    let mut aim = 0;

    for val in moves.iter() {
        match val {
            Forward(v) => {
                x_coor += v;
                depth += v * aim;
            }
            Down(v) => aim += v,
            Up(v) => aim -= v,
        }
    }

    (x_coor * depth).to_string()
}

#[cfg(test)]
mod tests {
    use super::*;
    // -- Real input
    // const INPUT: &str = include_str!("../input.txt");
    //
    // -- Test input
    const INPUT: &str = "forward 5
down 5
forward 8
up 3
down 8
forward 2";

    #[test]
    fn part1_works() {
        let result = part1(INPUT);
        assert_eq!(result, "150");
    }

    #[test]
    fn part2_works() {
        let result = part2(INPUT);
        assert_eq!(result, "900");
    }
}
