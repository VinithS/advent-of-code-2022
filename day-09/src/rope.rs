use std::{cmp, collections::HashSet};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Pos(i32, i32);

#[derive(Debug, Clone, Copy)]
pub enum Movement {
    Up,
    Down,
    Left,
    Right,
}

impl Movement {
    fn inverse(&self) -> &Movement {
        match self {
            Movement::Down => &Movement::Up,
            Movement::Up => &Movement::Down,
            Movement::Right => &Movement::Left,
            Movement::Left => &Movement::Right,
        }
    }
}

impl Pos {
    fn apply_move(&mut self, m: &Movement) {
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
        }
    }
}

#[derive(Debug)]
pub struct Rope {
    head: Pos,
    tail: Pos,
    pub visited: HashSet<Pos>,
}

impl Rope {
    pub fn new() -> Rope {
        Rope {
            head: Pos(0, 0),
            tail: Pos(0, 0),
            visited: HashSet::new(),
        }
    }

    pub fn move_head(&mut self, m: Movement) {
        self.head.apply_move(&m);

        let ad = cmp::max(
            (self.head.0 - self.tail.0).abs(),
            (self.head.1 - self.tail.1).abs(),
        );

        if ad >= 2 {
            self.tail = self.head; // reference??
            self.tail.apply_move(&m.inverse()); // would this affect head? - fine because Pos implements Copy
        }
        self.visited.insert(self.tail);
        // dbg!(ad, &self.head, &self.tail, &self.visited);
        println!("{:?}", self.tail);
    }

    pub fn move_head_n(&mut self, m: Movement, n: u32) {
        for _ in 0..n {
            self.move_head(m)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic_movements() {
        let mut rope = Rope {
            head: Pos(0, 0),
            tail: Pos(0, 0),
            visited: HashSet::new(),
        };

        rope.move_head(Movement::Up);
        assert_eq!(rope.tail, Pos(0, 0));

        rope.move_head(Movement::Up);
        assert_eq!(rope.tail, Pos(0, 1));

        rope.move_head(Movement::Up);
        assert_eq!(rope.tail, Pos(0, 2));

        rope.move_head(Movement::Down); // head is now overlapping with tail
        assert_eq!(rope.tail, Pos(0, 2));

        rope.move_head(Movement::Down);
        assert_eq!(rope.tail, Pos(0, 2));
    }

    #[test]
    fn test_diagonal_movement() {
        let mut rope = Rope {
            head: Pos(0, 0),
            tail: Pos(0, 0),
            visited: HashSet::new(),
        };

        rope.move_head(Movement::Up);
        assert_eq!(rope.tail, Pos(0, 0));
        rope.move_head(Movement::Right);
        assert_eq!(rope.tail, Pos(0, 0));

        rope.move_head(Movement::Up);
        assert_eq!(rope.tail, Pos(1, 1));

        rope.move_head(Movement::Right); // is now diagonal
        assert_eq!(rope.tail, Pos(1, 1));
        rope.move_head(Movement::Right);
        assert_eq!(rope.tail, Pos(2, 2));
    }

    #[test]
    fn test_negative_distance() {
        let mut rope = Rope {
            head: Pos(0, 0),
            tail: Pos(0, 0),
            visited: HashSet::new(),
        };

        rope.move_head_n(Movement::Right, 3);
        assert_eq!(rope.tail, Pos(2, 0));
        rope.move_head_n(Movement::Up, 3);
        assert_eq!(rope.tail, Pos(3, 2));

        rope.move_head(Movement::Left);
        assert_eq!(rope.tail, Pos(3, 2));

        rope.move_head(Movement::Left); // this is the absolute value test case
        assert_eq!(rope.tail, Pos(2, 3));
    }
}
