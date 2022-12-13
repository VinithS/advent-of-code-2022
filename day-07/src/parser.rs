use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{alpha1, newline},
    multi::separated_list1,
    sequence::separated_pair,
    IResult,
};

use crate::dir::{File, Resources};

pub fn shell_parser(input: &str) -> IResult<&str, Vec<Cmd>> {
    // we always start with `cd root`

    let (input, cmd) = separated_list1(newline, alt((ls_p, cd_p)))(input)?;

    Ok((input, cmd))
}

#[derive(Debug)]
pub enum Cmd<'a> {
    Cd(Cd<'a>),
    Ls(Vec<&'a str>),
}

#[derive(Debug)]
pub enum Cd<'a> {
    Up,
    Down(&'a str),
    Root,
}

fn cd_p(input: &str) -> IResult<&str, Cmd> {
    let (input, _) = tag("$ cd ")(input)?;
    let (input, dir) = alt((tag(".."), alpha1))(input)?;

    let cmd = match dir {
        "/" => Cmd::Cd(Cd::Root),
        ".." => Cmd::Cd(Cd::Up),
        name => Cmd::Cd(Cd::Down(name)),
    };

    Ok((input, cmd))
}

fn ls_p(input: &str) -> IResult<&str, Cmd> {
    let (input, _) = tag("$ ls")(input)?;
    let (input, files) = separated_list1(newline, alt((file_p, dir_p)))(input)?;

    todo!()
}

fn file_p(input: &str) -> IResult<&str, Resources> {
    let (input, (size, name)) = separated_pair(
        nom::character::complete::u32,
        tag(" "),
        // many1(one_of("qwertyuiopasdfghjklzxcvbnm.")),
        alpha1,
    )(input)?;

    Ok((input, Resources::File(File(name, size))))
}

fn dir_p(input: &str) -> IResult<&str, Resources> {
    let (input, _) = tag("dir ")(input)?;
    let (ipnut, dirname) = alpha1(input)?;

    Ok((input, Resources::Dir(dirname)))
}

// #[cfg(test)]
// mod tests {
//     use super::*;

//     const INPUT: &str = "$ cd /
// $ ls
// dir a
// 14848514 b.txt
// 8504156 c.dat
// dir d
// $ cd a
// $ ls
// dir e
// 29116 f
// 2557 g
// 62596 h.lst
// $ cd e
// $ ls
// 584 i
// $ cd ..
// $ cd ..
// $ cd d
// $ ls
// 4060174 j
// 8033020 d.log
// 5626152 d.ext
// 7214296 k";

//     #[test]
//     fn test_puzzle1() {
//         assert_eq!(dir_p("dir d"), Ok(("", Resources::Dir("d"))));
//     }
// }
