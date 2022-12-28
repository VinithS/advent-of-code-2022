use std::collections::VecDeque;

use crate::instructions::{Executable, Instruction};

pub struct CPU {
    pub clk: i32,
    pub register: i32,
    inst: VecDeque<Instruction>,
    disp: String,
}

impl CPU {
    pub fn new(i: VecDeque<Instruction>) -> CPU {
        CPU {
            clk: 0,      // clk starts at cycle 0
            register: 1, // reg init 1
            inst: i,
            disp: String::new(),
        }
    }

    pub fn render(&self) -> String {
        return self.disp.clone();
    }
}

pub trait Controller {
    fn increment_clk(&mut self);
    fn get_register_value(&self) -> i32;
    fn execute_instruction(&mut self);
    fn draw_to_crt(&mut self);
}

impl Controller for CPU {
    fn increment_clk(&mut self) {
        self.clk += 1;

        self.draw_to_crt();

        self.execute_instruction();
    }

    fn draw_to_crt(&mut self) {
        let sprite_range = (self.register - 1)..=(self.register + 1);
        let pos: i32 = (self.clk - 1) % 40;

        if sprite_range.contains(&pos) {
            self.disp.push('#');
        } else {
            self.disp.push('.');
        };

        if self.clk % 40 == 0 {
            self.disp.push('\n');
        }
    }

    fn get_register_value(&self) -> i32 {
        self.register
    }

    fn execute_instruction(&mut self) {
        match self.inst.get_mut(0) {
            Some(inst) => {
                let val = inst.execute();

                if inst.cost == 0 {
                    // println!("{:?}", i);
                    self.inst.pop_front();
                    self.register += val;
                }
            }
            None => {
                // println!("No more instructions!")
            }
        }
    }
}
