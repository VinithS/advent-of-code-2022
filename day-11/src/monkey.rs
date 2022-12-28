use crate::parser::Operation;

pub struct Monkey<'a> {
    items: Vec<i32>,
    tval: i32, // divide by for test value
    op: Operation,
    true_m: &'a mut Monkey<'a>,
    false_m: &'a mut Monkey<'a>,
}

impl Monkey<'_> {
    fn play_round(&self) {
        for item in &self.items {
            self.op.calculate_worry(*item);
        }
    }

    fn send_to_another_monkey(&mut self, item: i32, test_condition: bool) {
        if test_condition {
            self.true_m.items.push(item);
        } else {
            self.false_m.items.push(item);
        }
    }
}

struct Colony<'a> {
    monke: Vec<Monkey<'a>>,
}

pub struct Test<'a> {
    div: i32,
    tc: &'a Monkey<'a>,
    fc: &'a Monkey<'a>,
}
