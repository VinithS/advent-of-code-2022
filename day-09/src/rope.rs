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

#[derive(Debug, Clone)]
pub struct Rope {
    knots: Vec<Pos>,
    pub vis: HashSet<Pos>, // tail visisted
}

impl Rope {
    pub fn new(n: usize) -> Rope {
        Rope {
            knots: vec![Pos(0, 0); n],
            vis: HashSet::new(),
        }
    }

    pub fn move_head_n(&mut self, m: Movement, n: u32) {
        for _ in 0..n {
            self.move_head(m)
        }
    }

    pub fn move_head(&mut self, m: Movement) {
        self.knots.get_mut(0).unwrap().apply_move(&m); // mutate head first

        for i in 0..self.knots.len() - 1 {
            let h: Pos = self.knots.get(i).unwrap().to_owned(); // get immutable head ref
            let t: &mut Pos = self.knots.get_mut(i + 1).unwrap(); // get mutable tail ref

            // if tail doesn't move, we don't need to continue with the iteration
            if !Self::adj_tail(h, t, &m) {
                println!("Tail doesn't move head at {:?}", &h);
                break;
            }

            dbg!(&h, &t);
        }

        dbg!(&self.knots);

        self.vis.insert(self.knots.last().unwrap().clone());
    }

    fn adj_tail(h: Pos, t: &mut Pos, m: &Movement) -> bool {
        println!("checking head:{:?} againt tail:{:?}", &h, &t);

        let dx = h.0 - t.0;
        let dy = h.1 - t.1;

        dbg!(dx, dy);

        let dist = cmp::max(dx.abs(), dy.abs());

        // dbg!(&h, &t, dist);
        if dist >= 2 {
            let new_mvmnt: &Movement = if dx > dy {
                match m {
                    Movement::Up | Movement::Right => m.inverse(),
                    _ => {
                        return false;
                    }
                }
            } else if dx < 0 && dy > 0 {
                // top left
                match m {
                    Movement::Up | Movement::Left => m.inverse(),
                    _ => {
                        return false;
                    }
                }
            } else if dx < 0 && dy < 0 {
                // bot left
                match m {
                    Movement::Down | Movement::Left => m.inverse(),
                    _ => {
                        return false;
                    }
                }
            } else if dx > 0 && dy < 0 {
                // bot right
                match m {
                    Movement::Down | Movement::Right => m.inverse(),
                    _ => {
                        return false;
                    }
                }
            } else {
                m.inverse()
            };

            // is there a better way to do this
            t.0 = h.0;
            t.1 = h.1;

            println!("Moved tail to {:?}, applying {:?}", &t, &new_mvmnt);
            t.apply_move(&new_mvmnt);

            return true;
        }

        // tail doesn't move.
        return false;
    }

    fn get_last(&self) -> &Pos {
        &self.knots.last().unwrap()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic_movements() {
        let mut rope = Rope {
            knots: vec![Pos(0, 0); 2],
            vis: HashSet::new(),
        };

        rope.move_head(Movement::Up);
        assert_eq!(rope.get_last(), &Pos(0, 0));

        rope.move_head(Movement::Up);
        assert_eq!(rope.get_last(), &Pos(0, 1));

        rope.move_head(Movement::Up);
        assert_eq!(rope.get_last(), &Pos(0, 2));

        rope.move_head(Movement::Down); // head is now overlapping with tail
        assert_eq!(rope.get_last(), &Pos(0, 2));

        rope.move_head(Movement::Down);
        assert_eq!(rope.get_last(), &Pos(0, 2));
    }

    #[test]
    fn test_diagonal_movement() {
        let mut rope = Rope {
            knots: vec![Pos(0, 0); 2],
            vis: HashSet::new(),
        };

        rope.move_head(Movement::Up);
        assert_eq!(rope.get_last(), &Pos(0, 0));
        rope.move_head(Movement::Right);
        assert_eq!(rope.get_last(), &Pos(0, 0));

        rope.move_head(Movement::Up);
        assert_eq!(rope.get_last(), &Pos(1, 1));

        rope.move_head(Movement::Right); // is now diagonal
        assert_eq!(rope.get_last(), &Pos(1, 1));
        rope.move_head(Movement::Right);
        assert_eq!(rope.get_last(), &Pos(2, 2));
    }

    #[test]
    fn test_negative_distance() {
        let mut rope = Rope {
            knots: vec![Pos(0, 0); 2],
            vis: HashSet::new(),
        };

        rope.move_head_n(Movement::Right, 3);
        assert_eq!(rope.get_last(), &Pos(2, 0));
        rope.move_head_n(Movement::Up, 3);
        assert_eq!(rope.get_last(), &Pos(3, 2));

        rope.move_head(Movement::Left);
        assert_eq!(rope.get_last(), &Pos(3, 2));

        rope.move_head(Movement::Left); // this is the absolute value test case
        assert_eq!(rope.get_last(), &Pos(2, 3));
    }

    #[test]
    fn test_three_knots() {
        let mut rope = Rope {
            knots: vec![Pos(0, 0); 3], // 3 knots
            vis: HashSet::new(),
        };

        rope.move_head_n(Movement::Right, 3);
        assert_eq!(rope.get_last(), &Pos(1, 0));
        rope.move_head_n(Movement::Up, 3);
        assert_eq!(rope.get_last(), &Pos(3, 2));

        rope.move_head(Movement::Left);
        assert_eq!(rope.get_last(), &Pos(3, 2));

        rope.move_head(Movement::Left); // this is the absolute value test case
        assert_eq!(rope.get_last(), &Pos(2, 3));
    }

    #[test]
    fn test_small_puzzle_input() {
        let mut rope = Rope {
            knots: vec![Pos(0, 0); 10], // 10 knots
            vis: HashSet::new(),
        };

        // Right 5
        rope.move_head_n(Movement::Right, 5);

        // ......
        // ......
        // ......
        // ......
        // 4321H.  (4 covers 5, 6, 7, 8, 9, s)
        assert_eq!(
            rope.knots,
            vec![
                Pos(4, 0),
                Pos(3, 0),
                Pos(2, 0),
                Pos(1, 0),
                Pos(0, 0),
                Pos(0, 0),
                Pos(0, 0),
                Pos(0, 0),
                Pos(0, 0),
                Pos(0, 0)
            ]
        );

        // Up 4
        rope.move_head_n(Movement::Up, 4);

        // ....H.
        // ....1.
        // ..432.
        // .5....
        // 6.....  (6 covers 7, 8, 9, s)
        assert_eq!(
            rope.knots,
            vec![
                Pos(4, 4),
                Pos(4, 3),
                Pos(4, 2),
                Pos(3, 2),
                Pos(2, 2),
                Pos(1, 1),
                Pos(0, 0),
                Pos(0, 0),
                Pos(0, 0),
                Pos(0, 0)
            ]
        );

        // Left 3
        rope.move_head_n(Movement::Left, 3);
        // .H1...
        // ...2..
        // ..43..
        // .5....
        // 6.....  (6 covers 7, 8, 9, s)
        assert_eq!(
            rope.knots,
            vec![
                Pos(1, 4),
                Pos(2, 4),
                Pos(3, 3),
                Pos(3, 2),
                Pos(2, 2),
                Pos(1, 1),
                Pos(0, 0),
                Pos(0, 0),
                Pos(0, 0),
                Pos(0, 0)
            ]
        );

        // Down 1
        rope.move_head_n(Movement::Down, 1);
        // ..1...
        // .H.2..
        // ..43..
        // .5....
        // 6.....  (6 covers 7, 8, 9, s)
        assert_eq!(
            rope.knots,
            vec![
                Pos(1, 3),
                Pos(2, 4),
                Pos(3, 3),
                Pos(3, 2),
                Pos(2, 2),
                Pos(1, 1),
                Pos(0, 0),
                Pos(0, 0),
                Pos(0, 0),
                Pos(0, 0)
            ]
        );

        // Right 4
        rope.move_head_n(Movement::Right, 4);
        // ......
        // ...21H
        // ..43..
        // .5....
        // 6.....  (6 covers 7, 8, 9, s)
        assert_eq!(
            rope.knots,
            vec![
                Pos(5, 3),
                Pos(4, 3),
                Pos(3, 3),
                Pos(3, 2),
                Pos(2, 2),
                Pos(1, 1),
                Pos(0, 0),
                Pos(0, 0),
                Pos(0, 0),
                Pos(0, 0)
            ]
        );

        // Down 1
        rope.move_head_n(Movement::Down, 1);
        // ......
        // ...21.
        // ..43.H
        // .5....
        // 6.....  (6 covers 7, 8, 9, s)
        assert_eq!(
            rope.knots,
            vec![
                Pos(5, 2),
                Pos(4, 3),
                Pos(3, 3),
                Pos(3, 2),
                Pos(2, 2),
                Pos(1, 1),
                Pos(0, 0),
                Pos(0, 0),
                Pos(0, 0),
                Pos(0, 0)
            ]
        );

        // Left 5
        rope.move_head_n(Movement::Left, 5);
        // ......
        // ......
        // H123..  (2 covers 4)
        // .5....
        // 6.....  (6 covers 7, 8, 9, s)
        assert_eq!(
            rope.knots,
            vec![
                Pos(0, 2),
                Pos(1, 2),
                Pos(2, 2),
                Pos(3, 2),
                Pos(2, 2),
                Pos(1, 1),
                Pos(0, 0),
                Pos(0, 0),
                Pos(0, 0),
                Pos(0, 0)
            ]
        );

        // Right 2
        rope.move_head_n(Movement::Right, 2);
        // ......
        // ......
        // .1H3..  (H covers 2, 4)
        // .5....
        // 6.....  (6 covers 7, 8, 9, s)
        assert_eq!(
            rope.knots,
            vec![
                Pos(2, 2),
                Pos(1, 2),
                Pos(2, 2),
                Pos(3, 2),
                Pos(2, 2),
                Pos(1, 1),
                Pos(0, 0),
                Pos(0, 0),
                Pos(0, 0),
                Pos(0, 0)
            ]
        );
    }
}
