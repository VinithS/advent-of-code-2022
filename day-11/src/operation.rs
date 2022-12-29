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
    pub fn calculate_worry(&self, a: u64, worry: Option<u64>) -> u64 {
        let val = match self {
            Operation::Add(OpVal::Itself) => a + a,
            Operation::Add(OpVal::Val(n)) => a + n,
            Operation::Mult(OpVal::Itself) => a * a,
            Operation::Mult(OpVal::Val(n)) => a * n,
        };

        match worry {
            Some(lcm) => val % lcm,
            None => val / 3,
        }
    }
}
