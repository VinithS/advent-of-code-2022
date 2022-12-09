pub fn puzzle(file_contents: &str) {
    let stragey_guide = file_contents;

    // first column what elf will play
    // second column what I _should_ play

    // paper beats rock     - B Y >> A X
    // rock beats scissors  - A X >> C Z
    // scissors beats paper - C Z >> B Y

    // rock -> 1 -> a, x
    // paper -> 2 -> b, y
    // scissor -> 3 -> c, z

    // win -> 6
    // draw -> 3
    // lose -> 0

    let total_points: i32 = stragey_guide
        .split("\n")
        .filter(|s| !s.is_empty())
        .map(|s| {
            return s.split(" ").map(|ec| elf_code_to_tool(ec)).collect();
        })
        .map(|item: Vec<RPSTool>| {
            let my_tool = item.get(1).unwrap();
            let elf_tool = item.get(0).unwrap();
            let points = my_tool.point + my_tool.against(elf_tool);

            // println!(
            //     "{:?} vs {:?} - {:?} {}",
            //     elf_tool.tool, my_tool.tool, my_tool.point, points
            // );

            return points;
        })
        .sum();

    println!("You would get a total of {total_points} points");

    let new_points: i32 = stragey_guide
        .split("\n")
        .filter(|s| !s.is_empty())
        .map(|s| {
            return s.split(" ").map(|ec| elf_code_to_tool(ec)).collect();
        })
        .map(|item: Vec<RPSTool>| {
            let my_tool = item.get(1).unwrap(); // ignore my tool
            let elf_tool = item.get(0).unwrap();

            let points = elf_tool.point_given_outcome(&my_tool.tool);

            return points;
        })
        .sum();

    println!("But with the new strategy you would get total {new_points} points")
}

fn elf_code_to_tool(elf_code: &str) -> RPSTool {
    match elf_code {
        "A" | "X" => {
            return RPSTool {
                tool: Tool::Rock,
                point: 1,
            };
        }
        "B" | "Y" => {
            return RPSTool {
                tool: Tool::Paper,
                point: 2,
            }
        }
        "C" | "Z" => {
            return RPSTool {
                tool: Tool::Scissor,
                point: 3,
            }
        }
        &_ => panic!("shouldn't get here"),
    }
}

#[derive(Debug, PartialEq, Eq)]
enum Tool {
    Rock,
    Paper,
    Scissor,
}
struct RPSTool {
    tool: Tool,
    point: i32,
}

trait Game<'a> {
    fn against(&self, tool: &RPSTool) -> i32;
    fn point_given_outcome(&self, outcome: &Tool) -> i32;
}

impl Game<'_> for RPSTool {
    fn against(&self, tool: &RPSTool) -> i32 {
        // take the full slice of the String to get a &'static str
        match tool.tool {
            Tool::Rock => match self.tool {
                Tool::Rock => 3,
                Tool::Paper => 6,
                Tool::Scissor => 0,
            },
            Tool::Paper => match self.tool {
                Tool::Rock => 0,
                Tool::Paper => 3,
                Tool::Scissor => 6,
            },
            Tool::Scissor => match self.tool {
                Tool::Rock => 6,
                Tool::Paper => 0,
                Tool::Scissor => 3,
            },
        }
    }

    fn point_given_outcome(&self, tool: &Tool) -> i32 {
        match tool {
            // win
            Tool::Scissor => {
                6 + match self.tool {
                    Tool::Rock => elf_code_to_tool("B").point,    // paper
                    Tool::Paper => elf_code_to_tool("C").point,   // scissors
                    Tool::Scissor => elf_code_to_tool("A").point, // rock
                }
            }
            // lose
            Tool::Rock => {
                0 + match self.tool {
                    Tool::Rock => elf_code_to_tool("C").point,    // scissor
                    Tool::Paper => elf_code_to_tool("A").point,   // rock
                    Tool::Scissor => elf_code_to_tool("B").point, // paper
                }
            }
            // draw
            Tool::Paper => {
                3 + match self.tool {
                    Tool::Rock => elf_code_to_tool("A").point,    // rock
                    Tool::Paper => elf_code_to_tool("B").point,   // paper
                    Tool::Scissor => elf_code_to_tool("C").point, // scissor
                }
            }
            _ => panic!("Shouldn't get here!"),
        }
    }
}
