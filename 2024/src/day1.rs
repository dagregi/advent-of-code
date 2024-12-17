pub fn first_part(input: &str) -> i64 {
    let ll = input
        .lines()
        .map(|l| {
            l.split_whitespace()
                .map(|c| c.parse::<i64>().unwrap())
                .collect::<Vec<i64>>()
        })
        .collect::<Vec<_>>();
    let mut first = Vec::new();
    let mut last = Vec::new();

    ll.into_iter().for_each(|arr| {
        first.push(arr[0]);
        last.push(arr[1]);
    });

    first.sort();
    last.sort();

    first
        .iter()
        .enumerate()
        .map(|(idx, v)| {
            let sub = v - last[idx];
            if sub < 0 {
                -sub
            } else {
                sub
            }
        })
        .sum()
}

pub fn second_part(input: &str) -> i64 {
    let ll = input
        .lines()
        .map(|l| {
            l.split_whitespace()
                .map(|c| c.parse::<i64>().unwrap())
                .collect::<Vec<i64>>()
        })
        .collect::<Vec<_>>();
    let mut first = Vec::new();
    let mut last = Vec::new();

    ll.into_iter().for_each(|arr| {
        first.push(arr[0]);
        last.push(arr[1]);
    });

    first
        .iter()
        .map(|v| v * last.iter().filter(|&lv| lv == v).count() as i64)
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "3   4
4   3
2   5
1   3
3   9
3   3";

    #[test]
    fn first_part_test() {
        let result = first_part(INPUT);
        assert_eq!(result, 11);
    }

    #[test]
    fn second_part_test() {
        let result = second_part(INPUT);
        assert_eq!(result, 31);
    }
}
