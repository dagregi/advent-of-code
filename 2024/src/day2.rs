pub fn first_part(input: &str) -> usize {
    0
}

pub fn second_part(input: &str) -> usize {
    0
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "7 6 4 2 1
1 2 7 8 9
9 7 6 2 1
1 3 2 4 5
8 6 4 4 1
1 3 6 7 9";

    #[test]
    fn first_part_test() {
        let result = first_part(INPUT);
        assert_eq!(result, 2);
    }

    #[test]
    fn second_part_test() {
        let result = second_part(INPUT);
        assert_eq!(result, 31);
    }
}
