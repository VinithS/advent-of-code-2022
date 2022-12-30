use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{newline, u32},
    multi::separated_list0,
    sequence::tuple,
    IResult, Parser,
};

use crate::packet::Packet;

/// packet pair parser
pub fn ppp(input: &str) -> IResult<&str, (Packet, Packet)> {
    // println!("Starting ppp on {:?}", input);
    let (input, v) = tuple((lpp, newline, lpp))(input)?;
    Ok((input, (v.0, v.2)))
}

/// list packet parser
fn lpp(input: &str) -> IResult<&str, Packet> {
    // println!("[ Starting lpp on {:?} ", input);

    let (input, _) = tag("[")(input)?;

    let (input, sublist) = separated_list0(
        tag(","),
        alt((lpp, alt((u32.map(|v| Packet::Int(v)), lpp)))),
    )(input)?;

    // println!("  Finished lpp {:?}", &sublist);

    // must find closing bracket of itself here
    let (input, _) = tag("]")(input)?;

    // println!(
    //     "Closing list ] with {:?}",
    //     Packet::List(sublist.clone()))
    // );

    Ok((input, Packet::List(sublist)))
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
                (
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
                )
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
                (
                    Packet::List(vec![
                        Packet::List(vec![Packet::Int(1)]),
                        Packet::List(vec![Packet::Int(2), Packet::Int(3), Packet::Int(4),])
                    ]),
                    Packet::List(vec![Packet::List(vec![Packet::Int(1)]), Packet::Int(4)]),
                )
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
                (
                    Packet::List(vec![Packet::List(vec![Packet::List(vec![])])]), // 3 layers
                    Packet::List(vec![Packet::List(vec![])]),                     // 2 layers
                )
            ))
        );
    }
}
