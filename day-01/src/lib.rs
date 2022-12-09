use std::cmp::Reverse;

pub fn puzzle1(input: &str) -> String {
    let mut richest_elf = &Elf { foods: vec![0] };

    let elves = create_elf_colony(input);

    let mut max_cal = 0;
    for (_, elf) in elves.iter().enumerate() {
        let calories = elf.foods.iter().sum();
        if calories > max_cal {
            max_cal = calories;
            richest_elf = elf;
            // println!(
            //     "{:?} {:?} has the most calories with {:?}",
            //     i + 1,
            //     richest_elf,
            //     max_cal
            // );
        }
    }

    return richest_elf.foods.iter().sum::<i32>().to_string();
}

pub fn puzzle2(input: &str) -> String {
    let mut elves = create_elf_colony(input);
    elves.sort_by_cached_key(|e| Reverse(e.foods.iter().sum::<i32>()));

    // println!("{:?}", &elves[0..3]); // top 3

    let top_3_cal_sum: i32 = elves[0..3]
        .iter()
        .map(|e| e.foods.iter().sum::<i32>())
        .sum();

    println!("Top three elves have total of {:?} calories", top_3_cal_sum);

    return top_3_cal_sum.to_string();
}

fn create_elf_colony(file_contents: &str) -> Vec<Elf> {
    let all_elves = file_contents
        .split("\n\n")
        .map(|s: &str| convert_to_int_list(s))
        .map(|list: Vec<i32>| Elf {
            foods: list.to_owned(),
        })
        .collect();

    return all_elves;
}

fn convert_to_int_list(slice: &str) -> Vec<i32> {
    return slice
        .split("\n")
        .filter(|s| !s.is_empty())
        .map(|s| s.parse().unwrap())
        .collect();
}

#[derive(Debug, Hash, Eq, PartialEq)] // lets us print using {:?}
struct Elf {
    foods: Vec<i32>,
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "1000
2000
3000

4000

5000
6000

7000
8000
9000

10000";

    #[test]
    fn test_puzzle1() {
        let result = puzzle1(INPUT);
        assert_eq!(result, "24000");
    }

    #[test]
    fn test_puzzle2() {
        let result2 = puzzle2(INPUT);
        assert_eq!(result2, "45000")
    }
}
