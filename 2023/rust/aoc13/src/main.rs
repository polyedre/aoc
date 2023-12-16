fn analyze(pattern: &Vec<Vec<char>>) -> usize {

    // Check vertical split

    // Array of number of left column for a possible split
    let mut possible_vertical_split: Vec<usize> = (0..(pattern[0].len() - 1)).collect();

    for line in pattern {
        for split_position in possible_vertical_split.clone() {
            let (mut cur_left, mut cur_right) = (split_position, split_position + 1);

            while cur_right < line.len() {
                if line[cur_left] != line[cur_right] {
                    let position = possible_vertical_split.iter().position(|val| *val == split_position).unwrap();
                    possible_vertical_split.remove(position);
                    break;
                }

                if cur_left == 0 { break; }
                cur_left -= 1;
                cur_right += 1;
            }
        }
    }

    if !possible_vertical_split.is_empty() {
        if possible_vertical_split.len() == 1 {
            return possible_vertical_split[0] + 1;
        } else {
            panic!("Multiple vertical possibilies! {possible_vertical_split:?}");
        }
    }


    // Check horizontal split

    // Array of number of left column for a possible split
    let mut possible_horizontal_split: Vec<usize> = (0..(pattern.len() - 1)).collect();

    for col_id in 0..pattern[0].len() {
        for split_position in possible_horizontal_split.clone() {

            let (mut cur_left, mut cur_right) = (split_position, split_position + 1);

            while cur_right < pattern.len() {
                if pattern[cur_left][col_id] != pattern[cur_right][col_id] {
                    let position = possible_horizontal_split.iter().position(|val| *val == split_position).unwrap();
                    possible_horizontal_split.remove(position);
                    break;
                }

                if cur_left == 0 { break; }
                cur_left -= 1;
                cur_right += 1;
            }
        }
    }

    if !possible_horizontal_split.is_empty() {
        if possible_horizontal_split.len() == 1 {
            return 100 * (possible_horizontal_split[0] + 1);
        } else {
            panic!("Multiple horizontal possibilies! {possible_horizontal_split:?}");
        }
    }

    panic!("No solution found!")
}

fn main() {
    let mut total: usize = 0;
    let mut pattern_id = 0;

    let mut pattern: Vec<Vec<char>> = Vec::new();
    for line in std::io::stdin().lines() {
        let line = line.unwrap();
        if line.is_empty() {
            println!("Pattern {pattern_id}");
            pattern_id += 1;
            total += analyze(&pattern);
            pattern = Vec::new();
        } else {
            pattern.push(line.chars().collect())
        }
    }

    println!("Pattern {pattern_id}");
    total += analyze(&pattern);

    println!("{total}")
}
