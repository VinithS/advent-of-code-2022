pub fn process_part1(input: &str) -> String {
    todo!()
}

use std::cmp::Reverse;

// Calorie Counting
pub fn puzzle(file_contents: &str) {
    let mut richest_elf: &Elf;

    let mut elves = create_elf_colony(file_contents);

    let mut max_cal = 0;
    for (i, elf) in elves.iter().enumerate() {
        let calories = elf.foods.iter().sum();
        if calories > max_cal {
            max_cal = calories;
            richest_elf = elf;
            println!(
                "{:?} {:?} has the most calories with {:?}",
                i + 1,
                richest_elf,
                max_cal
            );
        }
    }

    elves.sort_by_cached_key(|e| Reverse(e.foods.iter().sum::<i32>()));

    // println!("{:?}", &elves[0..3]); // top 3

    let top_3_cal_sum: i32 = elves[0..3]
        .iter()
        .map(|e| e.foods.iter().sum::<i32>())
        .sum();

    println!("Top three elves have total of {:?} calories", top_3_cal_sum);
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

    #[test]
    fn it_works() {
        let input = "1000
        2000
        3000
        
        4000
        
        5000
        6000
        
        7000
        8000
        9000
        
        10000";

        let result = process_part1(input);
        assert_eq!(result, 24000);
    }
}
