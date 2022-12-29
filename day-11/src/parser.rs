use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{alphanumeric1, multispace1, u64},
    multi::separated_list1,
    sequence::{delimited, preceded, tuple},
    IResult,
};

use crate::{
    monkey::{Colony, Monkey, Test},
    operation::{OpVal, Operation},
};

pub fn colony_parser(input: &str) -> IResult<&str, Colony> {
    let (input, monkies) = separated_list1(tag("\n\n"), monkey_parser)(input)?;

    Ok((input, Colony { monkies }))
}

fn monkey_parser(input: &str) -> IResult<&str, Monkey> {
    let (input, _id) = delimited(tag("Monkey "), u64, tag(":"))(input)?;
    let (input, _) = multispace1(input)?;

    let (input, items) = item_parser(input)?;
    let (input, _) = multispace1(input)?;

    let (input, op) = op_parser(input)?;
    let (input, _) = multispace1(input)?;

    let (input, test) = test_parser(input)?;

    Ok((
        input,
        Monkey {
            items,
            op,
            test,
            inspected: 0,
        },
    ))
}

fn item_parser(input: &str) -> IResult<&str, Vec<u64>> {
    preceded(tag("Starting items: "), separated_list1(tag(", "), u64))(input)
}

fn test_parser(input: &str) -> IResult<&str, Test> {
    let (input, div) = preceded(tag("Test: divisible by "), u64)(input)?;
    let (input, _) = multispace1(input)?;

    let (input, pass_condition) = preceded(tag("If true: throw to monkey "), u64)(input)?;
    let (input, _) = multispace1(input)?;
    let (input, fail_condition) = preceded(tag("If false: throw to monkey "), u64)(input)?;

    Ok((
        input,
        Test {
            // div: div / 10,
            div,
            pass_condition: pass_condition as usize,
            fail_condition: fail_condition as usize,
        },
    ))
}

fn op_parser(input: &str) -> IResult<&str, Operation> {
    let (input, _) = tag("Operation: new = old ")(input)?;
    let (input, op) = tuple((alt((tag("+"), tag("*"))), multispace1, alphanumeric1))(input)?;

    let opval = match u64::from_str_radix(op.2, 10) {
        Ok(v) => OpVal::Val(v),
        Err(_) => OpVal::Itself,
    };

    let m_op = match op.0 {
        "+" => Operation::Add(opval),
        "*" => Operation::Mult(opval),
        _ => {
            panic!("Unexpected operations!");
        }
    };

    Ok((input, m_op))
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::vec;

    const INPUT: &str = include_str!("../test.txt");

    // Monkey { items, op, test })
    #[test]
    fn test_full_parsing() {
        assert_eq!(
            colony_parser(INPUT),
            Ok((
                "",
                Colony {
                    monkies: vec![
                        Monkey {
                            items: vec![79, 98],
                            op: Operation::Mult(OpVal::Val(19)),
                            test: Test {
                                div: 23,
                                pass_condition: 2,
                                fail_condition: 3,
                            },
                            inspected: 0,
                        },
                        Monkey {
                            items: vec![54, 65, 75, 74],
                            op: Operation::Add(OpVal::Val(6)),
                            test: Test {
                                div: 19,
                                pass_condition: 2,
                                fail_condition: 0,
                            },
                            inspected: 0,
                        },
                        Monkey {
                            items: vec![79, 60, 97],
                            op: Operation::Mult(OpVal::Itself),
                            test: Test {
                                div: 13,
                                pass_condition: 1,
                                fail_condition: 3,
                            },
                            inspected: 0,
                        },
                        Monkey {
                            items: vec![74],
                            op: Operation::Add(OpVal::Val(3)),
                            test: Test {
                                div: 17,
                                pass_condition: 0,
                                fail_condition: 1,
                            },
                            inspected: 0,
                        },
                    ]
                }
            ))
        );
    }

    #[test]
    fn test_op_parser() {
        assert_eq!(
            op_parser("Operation: new = old + 06"),
            Ok(("", Operation::Add(OpVal::Val(6))))
        );
        assert_eq!(
            op_parser("Operation: new = old + 6"),
            Ok(("", Operation::Add(OpVal::Val(6))))
        );
        assert_eq!(
            op_parser("Operation: new = old + old"),
            Ok(("", Operation::Add(OpVal::Itself)))
        );
        assert_eq!(
            op_parser("Operation: new = old * 19"),
            Ok(("", Operation::Mult(OpVal::Val(19))))
        );
        assert_eq!(
            op_parser("Operation: new = old * old"),
            Ok(("", Operation::Mult(OpVal::Itself)))
        );
    }
}
