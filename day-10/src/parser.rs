use std::collections::VecDeque;

use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{i32, newline},
    multi::separated_list0,
    sequence::tuple,
    IResult,
};

use crate::instructions::{IName, Instruction};

pub fn input_parser(input: &str) -> IResult<&str, VecDeque<Instruction>> {
    let (input, inst) = separated_list0(newline, alt((add_p, noop_p)))(input)?;
    Ok((input, VecDeque::from(inst)))
}

fn add_p(input: &str) -> IResult<&str, Instruction> {
    let (input, val) = tuple((tag("addx "), i32))(input)?;
    Ok((
        input,
        Instruction {
            name: IName::Add,
            cost: 2,
            value: val.1,
        },
    ))
}

fn noop_p(input: &str) -> IResult<&str, Instruction> {
    let (input, _) = tag("noop")(input)?;
    Ok((
        input,
        Instruction {
            name: IName::NoOp,
            cost: 1,
            value: 0,
        },
    ))
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "addx 15
addx -11
addx 6
noop
addx 13
addx -19
noop";

    #[test]
    fn test_parsing() {
        let result = input_parser(INPUT);
        assert_eq!(
            result,
            Ok((
                "",
                VecDeque::from(vec![
                    Instruction {
                        name: IName::Add,
                        cost: 2,
                        value: 15,
                    },
                    Instruction {
                        name: IName::Add,
                        cost: 2,
                        value: -11,
                    },
                    Instruction {
                        name: IName::Add,
                        cost: 2,
                        value: 6,
                    },
                    Instruction {
                        name: IName::NoOp,
                        cost: 1,
                        value: 0,
                    },
                    Instruction {
                        name: IName::Add,
                        cost: 2,
                        value: 13,
                    },
                    Instruction {
                        name: IName::Add,
                        cost: 2,
                        value: -19,
                    },
                    Instruction {
                        name: IName::NoOp,
                        cost: 1,
                        value: 0,
                    },
                ])
            ))
        );
    }
}
