use nom::{
    character::complete::u32,
    character::complete::{alpha1, newline, space0},
    multi::separated_list1,
    sequence::separated_pair,
    IResult,
};
use rope::{Movement, Rope};

mod rope;

pub fn puzzle1(input: &str) -> usize {
    let (_, mvs) = cmd_parser(input).unwrap();

    // dbg!(&mvs);

    let mut rope: Rope = Rope::new(2);

    mvs.iter().for_each(|mv| {
        rope.move_head_n(mv.0, mv.1);
        // dbg!(&rope);
    });

    // dbg!(&rope.visited);
    return rope.vis.len();
}

// multi crate crane
pub fn puzzle2(input: &str) -> usize {
    let (_, mvs) = cmd_parser(input).unwrap();

    dbg!(&mvs);

    let mut rope: Rope = Rope::new(10); // 10 knots

    mvs.iter().for_each(|mv| {
        rope.move_head_n(mv.0, mv.1);
        // dbg!(&rope);
    });

    // dbg!(&rope.visited);
    return rope.vis.len();
}

fn cmd_parser(input: &str) -> IResult<&str, Vec<(Movement, u32)>> {
    let (input, cmd) = separated_list1(newline, separated_pair(alpha1, space0, u32))(input)?;
    let mv = cmd
        .iter()
        .map(|(s, times)| match (*s, *times) {
            ("U", n) => (Movement::Up, n),
            ("D", n) => (Movement::Down, n),
            ("L", n) => (Movement::Left, n),
            ("R", n) => (Movement::Right, n),
            (_, _) => panic!("parsing error. input has incorrect letter."),
        })
        .collect();

    Ok((input, mv))
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "R 4
U 4
L 3
D 1
R 4
D 1
L 5
R 2";

    const INPUT_2: &str = "R 5
U 8
L 8
D 3
R 17
D 10
L 25
U 20";

    #[test]
    fn test_puzzle1() {
        let result = puzzle1(INPUT);
        assert_eq!(result, 13);
    }

    #[test]
    fn test_puzzle2() {
        let result = puzzle2(INPUT_2);
        assert_eq!(result, 36);
    }
}
