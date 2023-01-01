use nom::{
    bytes::complete::tag,
    character::complete::{i32, newline},
    multi::separated_list1,
    sequence::{preceded, separated_pair},
    IResult,
};

fn parser(input: &str) -> IResult<&str, Vec<((i32, i32), (i32, i32))>> {
    separated_list1(newline, line_parser)(input)
}

fn line_parser(input: &str) -> IResult<&str, ((i32, i32), (i32, i32))> {
    let (input, _) = tag("Sensor at ")(input)?;
    let (input, sensor) = xy_parser(input)?;
    let (input, _) = tag(": closest beacon is at ")(input)?;
    let (input, beacon) = xy_parser(input)?;

    Ok((input, (sensor, beacon)))
}

fn xy_parser(input: &str) -> IResult<&str, (i32, i32)> {
    separated_pair(
        preceded(tag("x="), i32),
        tag(", "),
        preceded(tag("y="), i32),
    )(input)
}

#[cfg(test)]
mod tests {
    use std::vec;

    use super::*;

    const INPUT: &str = "Sensor at x=2, y=18: closest beacon is at x=-2, y=15
Sensor at x=9, y=16: closest beacon is at x=10, y=16";

    #[test]
    fn test_basic_parsing() {
        assert_eq!(
            parser(INPUT),
            Ok(("", vec![((2, 18), (-2, 15)), ((9, 16), (10, 16))]))
        );
    }
}
