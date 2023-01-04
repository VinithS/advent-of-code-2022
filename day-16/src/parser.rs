use std::collections::HashMap;

use nom::{
    bytes::complete::tag,
    character::complete::{alpha1, newline, u32},
    multi::separated_list1,
    sequence::{preceded, separated_pair},
    IResult,
};

#[derive(Debug, PartialEq, Eq)]
pub struct Valve {
    rate: u32,
    child: Vec<String>,
}

// Valve AA has flow rate=0; tunnels lead to valves DD, II, BB
fn flow_parser(input: &str) -> IResult<&str, HashMap<String, Valve>> {
    let (input, valves) = separated_list1(
        newline,
        separated_pair(
            separated_pair(
                preceded(tag("Valve "), alpha1),
                tag(" has "),
                preceded(tag("flow rate="), u32),
            ),
            tag("; "),
            preceded(
                tag("tunnels lead to valves "),
                separated_list1(tag(", "), alpha1),
            ),
        ),
    )(input)?;

    let map: HashMap<String, Valve> = valves
        .into_iter()
        .map(|((name, rate), children)| {
            (
                name.to_string(),
                Valve {
                    rate,
                    child: children.into_iter().map(String::from).collect(),
                },
            )
        })
        .collect();

    Ok((input, map))
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "Valve AA has flow rate=0; tunnels lead to valves DD, II, BB
Valve BB has flow rate=13; tunnels lead to valves CC, AA";

    #[test]
    fn test_basic_parsing() {
        assert_eq!(
            flow_parser(INPUT),
            Ok((
                "",
                HashMap::from([
                    (
                        "AA".to_owned(),
                        Valve {
                            rate: 0,
                            child: vec!["DD".to_owned(), "II".to_owned(), "BB".to_owned()]
                        }
                    ),
                    (
                        "BB".to_owned(),
                        Valve {
                            rate: 13,
                            child: vec!["CC".to_owned(), "AA".to_owned()]
                        }
                    ),
                ])
            ))
        );
    }
}
