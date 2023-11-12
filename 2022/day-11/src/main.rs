use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{self, multispace1},
    multi::separated_list1,
    sequence::{delimited, preceded},
    IResult, Parser,
};
use std::{collections::VecDeque, fs};

fn main() {
    let input = fs::read_to_string("./input.txt").unwrap();
    println!(
        "Part 1: {}\nPart 2: {}",
        part1(input.as_str()),
        part2(input.as_str())
    );
}
#[derive(Debug)]
enum Value {
    Old,
    Num(u64),
}
#[derive(Debug)]
enum Operation {
    Mul((Value, Value)),
    Add((Value, Value)),
}
#[derive(Debug)]
struct Test {
    divisible: u64,
    true_recp: u64,
    false_recp: u64,
}
#[derive(Debug)]
struct Monkey {
    items: VecDeque<u64>,
    operation: Operation,
    test: Test,
    touch_count: u64,
}
impl Monkey {
    fn inspect(&mut self, stress: bool) -> u64 {
        self.touch_count += 1;
        let item = self.items.pop_front().unwrap();
        let worry_level = match &self.operation {
            Operation::Mul((a, b)) => {
                let num_a = match a {
                    Value::Old => item,
                    Value::Num(num) => *num,
                };
                let num_b = match b {
                    Value::Old => item,
                    Value::Num(num) => *num,
                };
                num_a * num_b
            }
            Operation::Add((a, b)) => {
                let num_a = match a {
                    Value::Old => item,
                    Value::Num(num) => *num,
                };
                let num_b = match b {
                    Value::Old => item,
                    Value::Num(num) => *num,
                };
                num_a + num_b
            }
        };
        if stress {
            worry_level / 3
        } else {
            worry_level
        }
    }
    fn test(&self, item: u64) -> u64 {
        if item % self.test.divisible == 0 {
            self.test.true_recp
        } else {
            self.test.false_recp
        }
    }
}

fn monkey(input: &str) -> IResult<&str, Monkey> {
    let (input, _id) = delimited(tag("Monkey "), complete::u64, tag(":"))(input)?;
    let (input, _) = multispace1(input)?;
    let (input, items) = preceded(
        tag("Starting items: "),
        separated_list1(tag(", "), complete::u64),
    )(input)?;
    let (input, _) = multispace1(input)?;
    let (input, op) = operation(input)?;
    let (input, _) = multispace1(input)?;
    let (input, test) = test_fn(input)?;
    Ok((
        input,
        Monkey {
            items: VecDeque::from(items),
            operation: op,
            test,
            touch_count: 0,
        },
    ))
}

fn test_fn(input: &str) -> IResult<&str, Test> {
    let (input, divisible) = preceded(tag("Test: divisible by "), complete::u64)(input)?;
    let (input, _) = multispace1(input)?;
    let (input, true_recp) = preceded(tag("If true: throw to monkey "), complete::u64)(input)?;
    let (input, _) = multispace1(input)?;
    let (input, false_recp) = preceded(tag("If false: throw to monkey "), complete::u64)(input)?;

    Ok((
        input,
        Test {
            divisible,
            true_recp,
            false_recp,
        },
    ))
}

fn operation(input: &str) -> IResult<&str, Operation> {
    let (input, _) = tag("Operation: new = ")(input)?;
    let (input, val1) = value(input)?;
    let (input, operator) = delimited(multispace1, alt((tag("*"), tag("+"))), multispace1)(input)?;
    let (input, val2) = value(input)?;

    let result = match operator {
        "*" => Operation::Mul((val1, val2)),
        "+" => Operation::Add((val1, val2)),
        _ => panic!("Unkonwn operator!"),
    };
    Ok((input, result))
}
fn value(input: &str) -> IResult<&str, Value> {
    alt((
        tag("old").map(|_| Value::Old),
        complete::u64.map(Value::Num),
    ))(input)
}

fn part1(input: &str) -> String {
    let (_, mut monkeys) = separated_list1(tag("\n\n"), monkey)(input).unwrap();
    for _ in 0..20 {
        for idx in 0..monkeys.len() {
            for _ in 0..monkeys[idx].items.len() {
                let monkey = monkeys.get_mut(idx).unwrap();
                let item = monkey.inspect(true);
                let monkey_sends_to = monkey.test(item);
                monkeys
                    .get_mut(monkey_sends_to as usize)
                    .unwrap()
                    .items
                    .push_back(item);
            }
        }
    }
    monkeys.sort_by_key(|monkey| monkey.touch_count);
    monkeys
        .iter()
        .rev()
        .take(2)
        .map(|monkey| monkey.touch_count)
        .product::<u64>()
        .to_string()
}

fn part2(input: &str) -> String {
    let (_, mut monkeys) = separated_list1(tag("\n\n"), monkey)(input).unwrap();
    for _ in 0..20 {
        for idx in 0..monkeys.len() {
            for _ in 0..monkeys[idx].items.len() {
                let monkey = monkeys.get_mut(idx).unwrap();
                let item = monkey.inspect(false);
                let monkey_sends_to = monkey.test(item);
                monkeys
                    .get_mut(monkey_sends_to as usize)
                    .unwrap()
                    .items
                    .push_back(item);
            }
        }
    }
    monkeys.sort_by_key(|monkey| monkey.touch_count);
    monkeys
        .iter()
        .rev()
        .take(2)
        .map(|monkey| monkey.touch_count)
        .product::<u64>()
        .to_string()
}
