use std::cmp;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Pos(pub i32, pub i32);

#[derive(Debug, Clone, Copy)]
pub enum Movement {
    Up,
    Down,
    Left,
    Right,
    // diagonal movements
    LU,
    RU,
    LD,
    RD,
}

impl Movement {
    pub fn inverse(&self) -> &Movement {
        match self {
            Movement::Down => &Movement::Up,
            Movement::Up => &Movement::Down,
            Movement::Right => &Movement::Left,
            Movement::Left => &Movement::Right,
            Movement::LD => &Movement::RU,
            Movement::LU => &Movement::RD,
            Movement::RD => &Movement::LU,
            Movement::RU => &Movement::LD,
        }
    }
}

impl Pos {
    pub fn apply_move(&mut self, m: &Movement) {
        match m {
            Movement::Up => {
                self.1 += 1;
            }
            Movement::Down => {
                self.1 -= 1;
            }
            Movement::Left => {
                self.0 -= 1;
            }
            Movement::Right => {
                self.0 += 1;
            }
            Movement::LD => {
                self.apply_move(&Movement::Left);
                self.apply_move(&Movement::Down);
            }
            Movement::LU => {
                self.apply_move(&Movement::Left);
                self.apply_move(&Movement::Up);
            }
            Movement::RD => {
                self.apply_move(&Movement::Right);
                self.apply_move(&Movement::Down);
            }
            Movement::RU => {
                self.apply_move(&Movement::Right);
                self.apply_move(&Movement::Up);
            }
        }
    }

    pub fn get_dir_and_dist(&self, point: &Pos) -> (Movement, i32) {
        let dx: i32 = point.0 - self.0;
        let dy: i32 = point.1 - self.1;

        return (Self::get_dir(dx, dy), cmp::max(dx.abs(), dy.abs()));
    }

    fn get_dir(dx: i32, dy: i32) -> Movement {
        // head moved diagonally. the tail needs to move diagonally to catch up
        if 4 - dx.abs() - dy.abs() == 0 {
            match (dx.is_positive(), dy.is_positive()) {
                (true, true) => {
                    return Movement::RU;
                }
                (true, false) => {
                    return Movement::RD;
                }
                (false, true) => {
                    return Movement::LU;
                }
                (false, false) => {
                    return Movement::LD;
                }
            }
        }

        if dx.abs() > dy.abs() {
            if dx.is_positive() {
                return Movement::Right;
            } else {
                return Movement::Left;
            }
        } else {
            if dy.is_positive() {
                return Movement::Up;
            } else {
                return Movement::Down;
            }
        }
    }
}
