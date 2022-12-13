use nom::{
    branch::alt,
    bytes::complete::{is_a, tag},
    character::complete::{alpha1, newline},
    multi::separated_list1,
    sequence::separated_pair,
    IResult,
};

use crate::dir::Resources;

#[derive(Debug, PartialEq, Eq)]
pub enum Cmd<'a> {
    Cd(Cd<'a>),
    Ls(Vec<Resources<'a>>),
}

#[derive(Debug, PartialEq, Eq)]
pub enum Cd<'a> {
    Up,
    Down(&'a str),
    Root,
}

pub fn shell_parser(input: &str) -> IResult<&str, Vec<Cmd>> {
    // we always start with `cd root`
    let (input, cmd) = separated_list1(newline, alt((ls_p, cd_p)))(input)?;

    Ok((input, cmd))
}

// --- individual parsers below
fn cd_p(input: &str) -> IResult<&str, Cmd> {
    let (input, _) = tag("$ cd ")(input)?;
    let (input, dir) = alt((tag(".."), tag("/"), alpha1))(input)?;

    let cmd = match dir {
        "/" => Cmd::Cd(Cd::Root),
        ".." => Cmd::Cd(Cd::Up),
        name => Cmd::Cd(Cd::Down(name)),
    };

    Ok((input, cmd))
}

fn ls_p(input: &str) -> IResult<&str, Cmd> {
    let (input, _) = tag("$ ls")(input)?;
    let (input, _) = newline(input)?;
    let (input, files) = separated_list1(newline, alt((file_p, dir_p)))(input)?;

    Ok((input, Cmd::Ls(files)))
}

fn file_p(input: &str) -> IResult<&str, Resources> {
    let (input, (size, name)) = separated_pair(
        nom::character::complete::u32,
        tag(" "),
        is_a("qwertyuiopasdfghjklzxcvbnm."),
    )(input)?;

    Ok((input, Resources::File(name, size)))
}

fn dir_p(input: &str) -> IResult<&str, Resources> {
    let (input, _) = tag("dir ")(input)?;
    let (input, dirname) = alpha1(input)?;

    Ok((input, Resources::Dir(dirname)))
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "$ cd /
$ ls
dir a
14848514 b.txt
8504156 c.dat
dir d
$ cd a
$ ls
dir e
29116 f
2557 g
62596 h.lst
$ cd e
$ ls
584 i
$ cd ..
$ cd ..
$ cd d
$ ls
4060174 j
8033020 d.log
5626152 d.ext
7214296 k";

    #[test]
    fn test_input_parsing() {
        let (remainder, parsed_cmds) = shell_parser(INPUT).unwrap();

        let expected_cmds = vec![
            Cmd::Cd(Cd::Root),
            Cmd::Ls(vec![
                Resources::Dir("a"),
                Resources::File("b.txt", 14848514),
                Resources::File("c.dat", 8504156),
                Resources::Dir("d"),
            ]),
            Cmd::Cd(Cd::Down("a")),
            Cmd::Ls(vec![
                Resources::Dir("e"),
                Resources::File("f", 29116),
                Resources::File("g", 2557),
                Resources::File("h.lst", 62596),
            ]),
            Cmd::Cd(Cd::Down("e")),
            Cmd::Ls(vec![Resources::File("i", 584)]),
            Cmd::Cd(Cd::Up),
            Cmd::Cd(Cd::Up),
            Cmd::Cd(Cd::Down("d")),
            Cmd::Ls(vec![
                Resources::File("j", 4060174),
                Resources::File("d.log", 8033020),
                Resources::File("d.ext", 5626152),
                Resources::File("k", 7214296),
            ]),
        ];

        // all should be parsed
        assert_eq!(remainder, "");

        assert_eq!(expected_cmds, parsed_cmds);
    }
}
