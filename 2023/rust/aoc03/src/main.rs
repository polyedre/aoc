use std::io;

fn check_board_for_number(row: usize, col: usize, board: &mut Vec<Vec<char>>) -> u32 {
    if !board[row][col].is_numeric() { return 0; }

    // Identify the first byte of the number
    let mut start_col = col;
    while start_col >= 1 && board[row][start_col - 1].is_numeric() {
        start_col -= 1;
    }

    let mut end_col = col;
    while end_col < board[row].len() - 1 && board[row][end_col + 1].is_numeric() {
        end_col += 1;
    }

    let engine_id = board[row][start_col..=end_col].iter().collect::<String>();
    println!("Found engine ID {}", engine_id);

    // Erase the number
    for index in start_col..=end_col {
        board[row][index] = '.';
    }

    return engine_id.parse().unwrap();
}

fn main() {
    let mut total = 0;
    let mut total_part_2 = 0;

    let mut board: Vec<Vec<char>> = Vec::new();

    for line in io::stdin().lines() {
        board.push(line.unwrap().chars().collect())
    }

    let width = board[0].len();
    let height = board.len();

    for (row, line) in board.clone().iter().enumerate() {
        for (col, cell_value) in line.iter().enumerate() {
            let mut is_gear = false;
            let mut adjacent_engine_parts: Vec<u32> = Vec::new();

            if *cell_value != '.' && !cell_value.is_numeric() {

                if *cell_value == '*' {
                    is_gear = true;
                }

                let mut cloned_board = board.clone();
                println!("Symbol '{}' at position ({}, {})", cell_value, row, col);
                for cursor_row in (row as i32 - 1)..=(row as i32 + 1) {
                    for cursor_col in (col as i32 - 1)..=(col as i32 + 1) {
                        println!("Looking for ({},{})", cursor_row, cursor_col);
                        if cursor_col >= 0 && cursor_col < width as i32 && cursor_row >= 0 && cursor_row < height as i32 {
                            let part_number: u32 = check_board_for_number(cursor_row.try_into().unwrap(), cursor_col.try_into().unwrap(), &mut cloned_board);
                            if part_number != 0 {
                                adjacent_engine_parts.push(part_number);
                            }
                            println!("Adding {}", part_number);
                            total += part_number;
                        }
                    }
                }

                println!("{}", adjacent_engine_parts.len());
                if is_gear && adjacent_engine_parts.len() == 2 {
                    total_part_2 += adjacent_engine_parts.iter().fold(1, |a, b| a * b);
                }
            }
        }
    }

    println!("Total part 1: {}", total);
    println!("Total part 2: {}", total_part_2);
}
