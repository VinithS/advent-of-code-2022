#[derive(Debug, PartialEq)]
pub enum IName {
    NoOp,
    Add,
}

#[derive(Debug, PartialEq)]
pub struct Instruction {
    pub name: IName,
    pub cost: i32,
    pub value: i32,
}

pub trait Executable {
    fn execute(&mut self) -> i32;
}

impl Executable for Instruction {
    fn execute(&mut self) -> i32 {
        self.cost -= 1;
        return self.value;
    }
}
