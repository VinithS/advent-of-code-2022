use crate::operation::Operation;

#[derive(Debug, PartialEq)]
pub struct Colony {
    pub monkies: Vec<Monkey>,
}

#[derive(Debug, PartialEq)]
pub struct Monkey {
    pub items: Vec<u64>,
    pub op: Operation,
    pub test: Test,
    pub inspected: u64,
}

#[derive(Debug, PartialEq)]
pub struct Test {
    pub div: u64,
    pub pass_condition: usize,
    pub fail_condition: usize,
}

impl Colony {
    pub fn get_meditation_number(&self) -> u64 {
        self.monkies.iter().map(|m| m.test.div).product::<u64>()
    }

    pub fn play_turn(&mut self, idx: usize, worry: Option<u64>) {
        let item_worries: Vec<u64> = self.get_item_worries(idx, worry);

        let m_test: &Test = &self.monkies.get_mut(idx).unwrap().test;

        let item_to_recipient: Vec<(u64, usize)> = item_worries
            .iter()
            .map(|i: &u64| (*i, m_test.run_test(i)))
            .map(|i: (u64, bool)| {
                (
                    i.0,
                    match i.1 {
                        true => m_test.pass_condition,
                        false => m_test.fail_condition,
                    },
                )
            })
            .collect();

        // dbg!(&item_to_recipient);
        // send each
        item_to_recipient
            .iter()
            .for_each(|(item_worry, rec)| self.send_to_another_monkey(*item_worry, *rec));
    }

    // sorts the colony
    pub fn get_top_n_inspections(&mut self, n: usize) -> Vec<u64> {
        if n > self.monkies.len() {
            panic!("There are not enough monkies.");
        }
        self.monkies.sort_unstable_by_key(|m| m.inspected);

        let inspections: Vec<u64> = self.monkies.iter().rev().map(|m| m.inspected).collect();

        inspections[0..n].to_owned()
    }

    fn get_item_worries(&mut self, idx: usize, worry: Option<u64>) -> Vec<u64> {
        let monkey: &mut Monkey = self.monkies.get_mut(idx).unwrap();
        let new_items: Vec<u64> = monkey
            .items
            .iter()
            .map(|i| monkey.op.calculate_worry(*i, worry))
            .collect();

        // clear the current monkey's items
        monkey.items.clear();
        monkey.inspected += new_items.len() as u64;

        return new_items;
    }

    fn send_to_another_monkey(&mut self, item: u64, idx: usize) {
        let m: &mut Monkey = self.monkies.get_mut(idx).unwrap();
        m.items.push(item);

        // println!("Pushing {} to {}", item, idx);
        // dbg!(&m.items);
    }
}

impl Test {
    fn run_test(&self, worry_level: &u64) -> bool {
        worry_level % self.div == 0
    }
}
