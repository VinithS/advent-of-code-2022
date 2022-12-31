use core::panic;
use std::{collections::HashSet, fmt::Display};

pub struct Grid {
    // (row, col)
    sand: (u32, i32),
    rocks: HashSet<(u32, i32)>,
}

impl Display for Grid {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut disp = String::new();
        for i in 0..=11 {
            for j in 0..=503 {
                if (i, j) == self.sand {
                    disp.push('o');
                } else if self.rocks.contains(&(i, j)) {
                    disp.push('#');
                } else {
                    disp.push('.');
                }
            }
            disp.push('\n');
        }

        write!(f, "{}", disp)
    }
}

impl Grid {
    const ORIGIN: (u32, i32) = (0, 500);

    pub fn new(rocks: HashSet<(u32, i32)>) -> Grid {
        Grid {
            sand: Grid::ORIGIN,
            rocks,
        }
    }

    pub fn simulate(&mut self, base: u32) -> u32 {
        let mut counter: u32 = 0;

        while let Some(ss_loc) = self.simulate_sand(base) {
            counter += 1;
            if ss_loc == Grid::ORIGIN {
                return counter;
            }
        }

        counter
    }

    // Returns option of if the sand landed somewhere
    fn simulate_sand(&mut self, base: u32) -> Option<(u32, i32)> {
        if self.sand.1 == 0 {
            panic!("We are at the left edge! What do?");
        }

        loop {
            // row below the lowest rock. will fall forever.
            if self.sand.0 > base {
                return None; // fall forever
            }

            if let Some(ss_loc) = self.step() {
                // reset
                self.sand = Grid::ORIGIN;
                return Some(ss_loc);
            }
        }
    }

    // return the coordiate if the falling sand stops
    fn step(&mut self) -> Option<(u32, i32)> {
        // if rock is below
        if self.rocks.contains(&(self.sand.0 + 1, self.sand.1)) {
            // check one step down and to the left
            if self.rocks.contains(&(self.sand.0 + 1, self.sand.1 - 1)) {
                // check one step down and to the right
                if self.rocks.contains(&(self.sand.0 + 1, self.sand.1 + 1)) {
                    // stopped sand here
                    let ss: (u32, i32) = self.sand;
                    self.rocks.insert(ss);

                    Some(ss)
                } else {
                    self.sand.0 += 1;
                    self.sand.1 += 1;

                    None
                }
            } else {
                self.sand.0 += 1;
                self.sand.1 -= 1;
                None
            }
        } else {
            self.sand.0 += 1;

            None
        }
    }
}
