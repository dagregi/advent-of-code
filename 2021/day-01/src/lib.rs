pub fn part1(input: &str) -> String {
    let depth_inc: Vec<u32> = input.lines().map(|v| v.parse::<u32>().unwrap()).collect();
    let mut sum = 0;
    depth_inc.into_iter().reduce(|acc, v| {
        if acc < v {
            sum += 1;
        }
        v
    });
    sum.to_string()
}

pub fn part2(input: &str) -> String {
    let depth_inc: Vec<u32> = input.lines().map(|v| v.parse::<u32>().unwrap()).collect();
    let mut sum = 0;

    let window: Vec<&[u32]> = depth_inc.windows(3).map(|v| v).collect();
    window.into_iter().reduce(|acc, v| {
        if acc.iter().sum::<u32>() < v.iter().sum::<u32>() {
            sum += 1;
        }
        v
    });
    sum.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;
    const INPUT: &str = include_str!("../input.txt");

    #[test]
    fn part1_works() {
        let result = part1(INPUT);
        assert_eq!(result, "7");
    }

    #[test]
    fn part2_works() {
        let result = part2(INPUT);
        assert_eq!(result, "5");
    }
}
