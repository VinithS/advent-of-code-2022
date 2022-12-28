use std::collections::VecDeque;

use cpu::CPU;
use instructions::Instruction;

use crate::{cpu::Controller, parser::input_parser};

mod cpu;
mod instructions;
mod parser;

pub fn puzzle1(input: &str) -> i32 {
    let instructions: VecDeque<Instruction> = input_parser(input).unwrap().1;

    let mut cpu: CPU = CPU::new(instructions);
    let mut val: i32 = 0;

    for cyc in 1..=240 {
        match cyc {
            20 | 60 | 100 | 140 | 180 | 220 => {
                val += cpu.register * cyc;
            }
            _ => {}
        }

        cpu.increment_clk();
    }

    return val;
}

pub fn puzzle2(input: &str) -> String {
    let instructions: VecDeque<Instruction> = input_parser(input).unwrap().1;
    let mut cpu: CPU = CPU::new(instructions);
    for _ in 1..=240 {
        cpu.increment_clk();
    }

    return cpu.render();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_puzzle1() {
        let result = puzzle1(INPUT);
        assert_eq!(result, 13140);
    }

    #[test]
    fn test_puzzle2() {
        let result = puzzle2(INPUT);
        assert_eq!(
            result,
            "##..##..##..##..##..##..##..##..##..##..
###...###...###...###...###...###...###.
####....####....####....####....####....
#####.....#####.....#####.....#####.....
######......######......######......####
#######.......#######.......#######.....
"
        )
    }

    const INPUT: &str = "addx 15
addx -11
addx 6
addx -3
addx 5
addx -1
addx -8
addx 13
addx 4
noop
addx -1
addx 5
addx -1
addx 5
addx -1
addx 5
addx -1
addx 5
addx -1
addx -35
addx 1
addx 24
addx -19
addx 1
addx 16
addx -11
noop
noop
addx 21
addx -15
noop
noop
addx -3
addx 9
addx 1
addx -3
addx 8
addx 1
addx 5
noop
noop
noop
noop
noop
addx -36
noop
addx 1
addx 7
noop
noop
noop
addx 2
addx 6
noop
noop
noop
noop
noop
addx 1
noop
noop
addx 7
addx 1
noop
addx -13
addx 13
addx 7
noop
addx 1
addx -33
noop
noop
noop
addx 2
noop
noop
noop
addx 8
noop
addx -1
addx 2
addx 1
noop
addx 17
addx -9
addx 1
addx 1
addx -3
addx 11
noop
noop
addx 1
noop
addx 1
noop
noop
addx -13
addx -19
addx 1
addx 3
addx 26
addx -30
addx 12
addx -1
addx 3
addx 1
noop
noop
noop
addx -9
addx 18
addx 1
addx 2
noop
noop
addx 9
noop
noop
noop
addx -1
addx 2
addx -37
addx 1
addx 3
noop
addx 15
addx -21
addx 22
addx -6
addx 1
noop
addx 2
addx 1
noop
addx -10
noop
noop
addx 20
addx 1
addx 2
addx 2
addx -6
addx -11
noop
noop
noop";
}
