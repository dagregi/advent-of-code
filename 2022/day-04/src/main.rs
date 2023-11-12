use std::{fs, ops::RangeInclusive};

use nom::{
    bytes::complete::tag,
    character::complete::{self, newline},
    multi::separated_list1,
    sequence::separated_pair,
    *,
};

fn section(input: &str) -> IResult<&str, RangeInclusive<u32>> {
    let (input, start) = complete::u32(input)?;
    let (input, _) = tag("-")(input)?;
    let (input, end) = complete::u32(input)?;

    Ok((input, start..=end))
}

fn line_parser(input: &str) -> IResult<&str, (RangeInclusive<u32>, RangeInclusive<u32>)> {
    let (input, (start, end)) = separated_pair(section, tag(","), section)(input)?;

    Ok((input, (start, end)))
}

fn section_assignment(
    input: &str,
) -> IResult<&str, Vec<(RangeInclusive<u32>, RangeInclusive<u32>)>> {
    let (input, ranges) = separated_list1(newline, line_parser)(input)?;
    Ok((input, ranges))
}

fn main() {
    let input = fs::read_to_string("./input.txt").unwrap();
    let (_, assignments) = section_assignment(input.as_str()).unwrap();

    let results = assignments
        .iter()
        .filter(|(range_a, range_b)| {
            let a_contains_b = range_a.clone().any(|num| range_b.contains(&num));
            let b_contains_a = range_b.clone().any(|num| range_a.contains(&num));

            a_contains_b || b_contains_a
        })
        .count();
    println!("{:?}", results);
}
