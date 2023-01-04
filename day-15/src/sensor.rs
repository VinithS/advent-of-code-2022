#[derive(Debug, PartialEq, Eq)]
pub struct Sensor {
    pub pos: (isize, isize),
    pub beacon: (isize, isize),
    pub dist: isize,
}

impl Sensor {
    pub fn man_dist_to(&self, pos: (isize, isize)) -> isize {
        (pos.0 - self.pos.0).abs() + (pos.1 - self.pos.1).abs()
    }

    pub fn is_in_range(&self, pos: (isize, isize)) -> bool {
        self.man_dist_to(pos) <= self.dist
    }

    pub fn is_occupied(&self, pos: (isize, isize)) -> bool {
        pos == self.pos || pos == self.beacon
    }
}
