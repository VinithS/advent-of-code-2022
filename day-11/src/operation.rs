#[derive(Debug, PartialEq)]
pub enum OpVal {
    Itself,
    Val(u64),
}

#[derive(Debug, PartialEq)]
pub enum Operation {
    Add(OpVal),
    Mult(OpVal),
}

impl Operation {
    pub fn calculate_worry(&self, a: u64, dc: bool, mn: u64) -> u64 {
        let val = match self {
            Operation::Add(OpVal::Itself) => a + a,
            Operation::Add(OpVal::Val(n)) => a + n,
            Operation::Mult(OpVal::Itself) => a * a,
            Operation::Mult(OpVal::Val(n)) => a * n,
        };

        if dc {
            val / 3
        } else {
            val % mn
        }
    }
}
