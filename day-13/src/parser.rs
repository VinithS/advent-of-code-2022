use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{newline, u32},
    multi::{separated_list0, separated_list1},
    sequence::{delimited, separated_pair},
    IResult, Parser,
};

use crate::packet::Packet;

/// packet pair parser
pub fn ppp(input: &str) -> IResult<&str, Vec<(Packet, Packet)>> {
    separated_list1(tag("\n\n"), separated_pair(lpp, newline, lpp))(input)
}

/// list packet parser
fn lpp(input: &str) -> IResult<&str, Packet> {
    alt((
        delimited(tag("["), separated_list0(tag(","), lpp), tag("]"))
            .map(|v: Vec<Packet>| Packet::List(v)),
        u32.map(|i: u32| Packet::Int(i)),
    ))(input)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic_parsing() {
        let result = ppp("[1,1,3,1,1]\n[1,1,5,1,1]");
        assert_eq!(
            result,
            Ok((
                "",
                vec![(
                    (Packet::List(vec![
                        Packet::Int(1),
                        Packet::Int(1),
                        Packet::Int(3),
                        Packet::Int(1),
                        Packet::Int(1)
                    ])),
                    Packet::List(vec![
                        Packet::Int(1),
                        Packet::Int(1),
                        Packet::Int(5),
                        Packet::Int(1),
                        Packet::Int(1)
                    ])
                )]
            ))
        );
    }

    #[test]
    fn test_parsing_list_in_list() {
        let result = ppp("[[1],[2,3,4]]\n[[1],4]");
        assert_eq!(
            result,
            Ok((
                "",
                vec![(
                    Packet::List(vec![
                        Packet::List(vec![Packet::Int(1)]),
                        Packet::List(vec![Packet::Int(2), Packet::Int(3), Packet::Int(4),])
                    ]),
                    Packet::List(vec![Packet::List(vec![Packet::Int(1)]), Packet::Int(4)]),
                )]
            ))
        );
    }

    #[test]
    fn test_parsing_empty_list() {
        let result = ppp("[[[]]]\n[[]]");

        assert_eq!(
            result,
            Ok((
                "",
                vec![(
                    Packet::List(vec![Packet::List(vec![Packet::List(vec![])])]), // 3 layers
                    Packet::List(vec![Packet::List(vec![])]),                     // 2 layers
                )]
            ))
        );
    }
}
