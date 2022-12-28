use std::ops::Add;

use either::{self, Either};
use nom::{
    branch::alt,
    bytes::complete::{tag, take_while},
    character::{
        complete::{alphanumeric1, space0},
        is_alphanumeric,
    },
    multi::separated_list0,
    number::complete::i32,
    sequence::tuple,
    IResult,
};

use crate::monkey::{Monkey, Test};

#[derive(Debug, PartialEq)]
enum OpVal {
    Itself,
    Val(i32),
}

#[derive(Debug, PartialEq)]
pub enum Operation {
    Add(OpVal),
    Mult(OpVal),
}

impl Operation {
    pub fn calculate_worry(&self, a: i32) -> i32 {
        let raw_worry = match self {
            Operation::Add(OpVal::Itself) => a + a,
            Operation::Add(OpVal::Val(n)) => a + n,
            Operation::Mult(OpVal::Itself) => a * a,
            Operation::Mult(OpVal::Val(n)) => a * n,
        };

        return raw_worry / 3; // potential source of bugs
    }
}

fn input_parser(input: &str) -> IResult<&str, &str> {
    let (input, x) = separated_list0(tag("\n\n"), tag("Monkey: "))(input)?;

    dbg!(input, x);

    Ok(("", ""))
}

fn monkey_parser(input: &str) -> IResult<&str, Vec<Monkey>> {
    todo!()
}

fn item_parser(input: &str) -> IResult<&str, Vec<i32>> {
    let (input, _) = tag("Starting items: ")(input)?;
    // let (input, vals) = separated_list0(tag(","), i32);

    // Ok((input, vals))
    todo!()
}

fn test_parser(input: &str) -> IResult<&str, Test> {
    let (input, _) = tag("Test: divisible by ")(input)?;
    todo!()
}

fn op_parser(input: &str) -> IResult<&str, Operation> {
    let (input, _) = tag("Operation: new = old ")(input)?;

    // Operation: new = old <OP> <Num/Old>
    // in the format of x = y op z

    let (input, op) = tuple((alt((tag("+"), tag("*"))), space0, alphanumeric1))(input)?;

    let opval = match i32::from_str_radix(op.1, 10) {
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

    dbg!(i32::from_str_radix(op.1, 10).unwrap(), input, op, &m_op);

    Ok((input, m_op))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_op_parser() {
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
    #[test]
    fn test_parser() {
        let result = input_parser(INPUT);
        // assert_eq!(result, "");
    }

    const INPUT: &str = "Monkey 0:
Starting items: 79, 98
Operation: new = old * 19
Test: divisible by 23
  If true: throw to monkey 2
  If false: throw to monkey 3

Monkey 1:
Starting items: 54, 65, 75, 74
Operation: new = old + 6
Test: divisible by 19
  If true: throw to monkey 2
  If false: throw to monkey 0

Monkey 2:
Starting items: 79, 60, 97
Operation: new = old * old
Test: divisible by 13
  If true: throw to monkey 1
  If false: throw to monkey 3

Monkey 3:
Starting items: 74
Operation: new = old + 3
Test: divisible by 17
  If true: throw to monkey 0
  If false: throw to monkey 1";
}
