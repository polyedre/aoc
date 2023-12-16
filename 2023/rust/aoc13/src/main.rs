fn check_for_vertical_split(pattern: &Vec<Vec<char>>) -> Vec<usize> {
    let mut required_smudges_per_split_position: Vec<usize> = vec![0; pattern[0].len() - 1];

    for line in pattern {
        for split_position in 0..required_smudges_per_split_position.len() {
            let (mut cur_left, mut cur_right) = (split_position, split_position + 1);

            while cur_right < line.len() {
                if line[cur_left] != line[cur_right] {
                    required_smudges_per_split_position[split_position] += 1;
                }

                if cur_left == 0 { break; }
                cur_left -= 1;
                cur_right += 1;
            }
        }
    }

    required_smudges_per_split_position
}

fn check_for_horizontal_split(pattern: &Vec<Vec<char>>) -> Vec<usize> {
    let mut required_smudges_per_split_position: Vec<usize> = vec![0; pattern.len() - 1];

    for col_id in 0..pattern[0].len() {
        for split_position in 0..required_smudges_per_split_position.len() {

            let (mut cur_left, mut cur_right) = (split_position, split_position + 1);

            while cur_right < pattern.len() {
                if pattern[cur_left][col_id] != pattern[cur_right][col_id] {
                    required_smudges_per_split_position[split_position] += 1;
                }

                if cur_left == 0 { break; }
                cur_left -= 1;
                cur_right += 1;
            }
        }
    }

    required_smudges_per_split_position
}

fn find_reflexions(pattern: &Vec<Vec<char>>) -> (Vec<usize>, Vec<usize>) {
    (check_for_vertical_split(pattern), check_for_horizontal_split(pattern))
}

fn compute_pattern(pattern: &Vec<Vec<char>>, total_part_1: &mut usize, total_part_2: &mut usize) {
    let (vertical_reflexions, horizontal_reflexions) = find_reflexions(&pattern);

    *total_part_1 += &vertical_reflexions.
        iter()
        .enumerate()
        .filter(|(_, smudge)| smudge == &&0)
        .map(|(position, _)| position + 1)
        .sum();

    *total_part_1 += &horizontal_reflexions.
        iter()
        .enumerate()
        .filter(|(_, smudge)| smudge == &&0)
        .map(|(position, _)| (position + 1) * 100)
        .sum();

    *total_part_2 += &vertical_reflexions.
        iter()
        .enumerate()
        .filter(|(_, smudge)| smudge == &&1)
        .map(|(position, _)| position + 1)
        .sum();

    *total_part_2 += &horizontal_reflexions.
        iter()
        .enumerate()
        .filter(|(_, smudge)| smudge == &&1)
        .map(|(position, _)| (position + 1) * 100)
        .sum();
}

fn main() {
    let mut total_part_1: usize = 0;
    let mut total_part_2: usize = 0;

    let mut pattern: Vec<Vec<char>> = Vec::new();
    for line in std::io::stdin().lines() {
        let line = line.unwrap();
        if line.is_empty() {
            compute_pattern(&pattern, &mut total_part_1, &mut total_part_2);
            pattern = Vec::new();
        } else {
            pattern.push(line.chars().collect())
        }
    }

    compute_pattern(&pattern, &mut total_part_1, &mut total_part_2);

    println!("Part 1: {total_part_1}, part 2: {total_part_2}");
}
