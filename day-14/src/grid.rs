use core::panic;
use std::collections::HashSet;

pub struct Grid {
    // (row, col)
    sand: (u32, u32),
    rocks: HashSet<(u32, u32)>,
}

impl Grid {
    const ORIGIN: (u32, u32) = (0, 500);

    pub fn new(rocks: HashSet<(u32, u32)>) -> Grid {
        Grid {
            sand: Grid::ORIGIN,
            rocks,
        }
    }

    pub fn simulate(&mut self, base: u32) -> u32 {
        let mut counter: u32 = 0;

        while let Some(true) = self.simulate_sand(base) {
            // println!("Sand: {:2}", counter);
            counter += 1;
        }

        counter
    }

    // Returns option of if the sand landed somewhere
    fn simulate_sand(&mut self, base: u32) -> Option<bool> {
        if self.sand.1 == 0 {
            panic!("We are at the left edge! What do?");
        }

        loop {
            // row below the lowest rock. will fall forever.
            if self.sand.0 >= base {
                return Some(false); // fall forever
            }

            // stopped sand location ignored -- maybe pt2
            if let Some(_ss_loc) = self.step() {
                // reset
                self.sand = Grid::ORIGIN;
                return Some(true);
            }
        }
    }

    // return the coordiate if the falling sand stops
    fn step(&mut self) -> Option<(u32, u32)> {
        // if rock is below
        if self.rocks.contains(&(self.sand.0 + 1, self.sand.1)) {
            // check one step down and to the left
            if self.rocks.contains(&(self.sand.0 + 1, self.sand.1 - 1)) {
                // check one step down and to the right
                if self.rocks.contains(&(self.sand.0 + 1, self.sand.1 + 1)) {
                    // stopped sand here
                    let ss: (u32, u32) = self.sand;
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
