use sensor::Sensor;

mod parser;
mod sensor;

pub fn puzzle1(input: &str, test_y: isize) -> isize {
    let (_, sensors) = parser::sensor_parser(input).unwrap();

    let (min_x, max_x) = get_min_max_x(&sensors);

    // actually (true not_possible) - (# sensor or beacon)
    let mut not_possible: isize = 0;

    for x in min_x..=max_x {
        let pos = (x, test_y);

        for s in &sensors {
            if s.is_in_range(pos) && !s.is_occupied(pos) {
                not_possible += 1;
                break;
            }
        }
    }

    not_possible
}

pub fn puzzle2(input: &str, bound: (isize, isize)) -> usize {
    let (_, sensors) = parser::sensor_parser(input).unwrap();

    for y in 0..=bound.1 {
        let mut x = 0;

        'col: while x <= bound.0 {
            let pos = (x, y);

            for s in &sensors {
                let mut dx = 0;

                if s.is_in_range(pos) {
                    // move x past the range of s
                    if s.pos.0 > x {
                        dx += (s.pos.0 - x) * 2;
                    } else {
                        // it could be on the right diagonal edge
                        dx += std::cmp::max(1, s.dist - s.man_dist_to(pos));
                    }
                    x += dx;
                    continue 'col;
                }
            }
            // should never reach here unless all sensor are out of each
            return (x as i64 * 4_000_000 + y as i64) as usize;
        }
    }

    // should never reach here unless test case fails.
    unreachable!()
}

fn get_min_max_x(sensors: &Vec<Sensor>) -> (isize, isize) {
    let mut min_x = isize::max_value();
    let mut max_x = isize::min_value();

    for s in sensors {
        min_x = std::cmp::min(s.pos.0 - s.dist, min_x);
        max_x = std::cmp::max(s.pos.0 + s.dist, max_x);
    }

    (min_x, max_x)
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = include_str!("../test.txt");

    #[test]
    fn test_puzzle1() {
        assert_eq!(puzzle1(INPUT, 09), 25);
        assert_eq!(puzzle1(INPUT, 10), 26);
        assert_eq!(puzzle1(INPUT, 11), 27);
    }

    #[test]
    fn test_puzzle2() {
        let result = puzzle2(INPUT, (20, 20));
        assert_eq!(result, 56000011);
    }
}
