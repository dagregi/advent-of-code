use nom::{
    bytes::complete::tag,
    character::complete::{self, multispace1, newline},
    multi::separated_list1,
    IResult,
};

fn main() {
    // val = 0..=time
    // rec = ( time - val ) * val
    // rec > distance -> +1
    let input = include_str!("../inputs/6.txt");
    let (_, (time_col, distance_col)) = input_parser(input).unwrap();

    let mut margin = Vec::new();
    for (idx, &time) in time_col.iter().enumerate() {
        let mut count = 0;
        for val in 0..=time {
            let rec = (time - val) * val;
            if rec > distance_col[idx] {
                count += 1;
            }
        }
        margin.push(count);
    }

    println!("{}", margin.iter().product::<u32>());
}

fn input_parser(input: &str) -> IResult<&str, (Vec<u32>, Vec<u32>)> {
    let (input, _) = tag("Time:")(input)?;
    let (input, _) = multispace1(input)?;
    let (input, time) = separated_list1(multispace1, complete::u32)(input)?;
    let (input, _) = newline(input)?;

    let (input, _) = tag("Distance:")(input)?;
    let (input, _) = multispace1(input)?;
    let (input, distance) = separated_list1(multispace1, complete::u32)(input)?;

    Ok((input, (time, distance)))
}
