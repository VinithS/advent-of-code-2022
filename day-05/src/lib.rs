pub fn puzzle1(input: &str) -> String {
    let separated: Vec<String> = input.split("\n\n").map(|m| m.to_owned()).collect();

    let board_str = separated.get(0).unwrap().to_owned();

    let mut board: Vec<Vec<String>> = create_board(&board_str);

    // iterator over the instructions
    for instruction in separated.get(1).unwrap().to_owned().lines() {
        execute_instruction(instruction, &mut board, false);
    }

    // dbg!("BOARD AT THE END: ", &board);

    return get_top_boxes(&board);
}

// multi crate crane
pub fn puzzle2(input: &str) -> String {
    let separated: Vec<String> = input.split("\n\n").map(|m| m.to_owned()).collect();

    let board_str = separated.get(0).unwrap().to_owned();

    let mut board: Vec<Vec<String>> = create_board(&board_str);

    // iterator over the instructions
    for instruction in separated.get(1).unwrap().to_owned().lines() {
        execute_instruction(instruction, &mut board, true);
    }

    return get_top_boxes(&board);
}

fn execute_instruction(inst: &str, board: &mut Vec<Vec<String>>, is_multi_crate: bool) {
    // println!("Executing Instrction {:?}", inst);

    let parsed_inst: Vec<&str> = inst.split(" ").collect();

    let src: usize = (*parsed_inst.get(3).unwrap()).parse::<usize>().unwrap() - 1; // elf indexing is 1
    let dest: usize = (*parsed_inst.get(5).unwrap()).parse::<usize>().unwrap() - 1;
    let amt: usize = (*parsed_inst.get(1).unwrap()).parse::<usize>().unwrap();

    // dbg!(&src, &dest, &amt);

    let src_stack: &mut Vec<String> = board.get_mut(src).unwrap();

    // dbg!("source stack: ", &src_stack);

    let mut temp_queue: Vec<String> = vec![];

    // dbg!("temp queue before: ", &temp_queue);

    for _ in 0..amt {
        let popped: String = src_stack.pop().unwrap();
        temp_queue.push(popped);
    }

    // dbg!("temp queue after: ", &temp_queue);

    let dest_stack: &mut Vec<String> = board.get_mut(dest).unwrap();

    // dbg!("dest stack before: ", &dest_stack);

    let crane = temp_queue.iter();

    if is_multi_crate {
        crane.rev().for_each(|b| dest_stack.push(b.to_string()));
    } else {
        crane.for_each(|b| dest_stack.push(b.to_string()));
    }

    // dbg!("dest stack after: ", &dest_stack);
}

fn create_board(board_input: &str) -> Vec<Vec<String>> {
    let mut b_reader = board_input.lines().rev();

    // dbg!(&b_reader);

    let mut board: Vec<Vec<String>> = b_reader
        .next()
        .unwrap()
        .split(" ")
        .filter(|s| !s.is_empty())
        .map(|s| vec![s.to_owned()])
        .collect();

    // dbg!(&board);
    for row in b_reader {
        // index error is per row
        let mut idx_err: i32 = 0;

        let pieces = row.split(" ");
        // dbg!(&row, &pieces, &board);
        for (idx, piece) in pieces.enumerate() {
            if piece == "" {
                idx_err += 1; // [A] --- [B] -- split(" ") --> [[A],"","", "",[B]] <- three chars
                continue;
            } else {
                // if piece is the base number, skip it.
                if piece.len() == 1 {
                    continue;
                }

                // iterator index - empty count + 1 empty  for every four spaces missed
                let true_idx = idx as i32 - idx_err + idx_err / 4;
                // dbg!(&idx, &idx_err, &true_idx, &piece);

                let stack: &mut Vec<String> = board
                    .get_mut((true_idx) as usize) // casting X_X
                    .unwrap();

                // dbg!("Before push:", &stack);
                stack.push(piece.to_owned());
                // dbg!("After push:", &stack);
            }
        }
        // dbg!("board after row:");
        // dbg!(&board);
    }
    // dbg!(&board);
    return board;
}

fn get_top_boxes(board: &Vec<Vec<String>>) -> String {
    board
        .iter()
        .map(|stack| stack.last().unwrap())
        .map(|b| b.chars().nth(1).unwrap().to_string())
        .collect::<String>()
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "    [D]
[N] [C]
[Z] [M] [P]
 1   2   3

move 1 from 2 to 1
move 3 from 1 to 3
move 2 from 2 to 1
move 1 from 1 to 2";
    //     const INPUT: &str = "    [G] [R]                 [P]
    //     [H] [W]     [T] [P]     [H]
    //     [F] [T] [P] [B] [D]     [N]
    // [L] [T] [M] [Q] [L] [C]     [Z]
    // [C] [C] [N] [V] [S] [H]     [V] [G]
    // [G] [L] [F] [D] [M] [V] [T] [J] [H]
    // [M] [D] [J] [F] [F] [N] [C] [S] [F]
    // [Q] [R] [V] [J] [N] [R] [H] [G] [Z]
    //  1   2   3   4   5   6   7   8   9

    // move 5 from 8 to 2
    // move 2 from 4 to 5
    // move 3 from 3 to 9";

    #[test]
    fn test_puzzle1() {
        let result = puzzle1(INPUT);
        assert_eq!(result, "CMZ");
    }

    #[test]
    fn test_puzzle2() {
        let result = puzzle2(INPUT);
        assert_eq!(result, "MCD");
    }
}
