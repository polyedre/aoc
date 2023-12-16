#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
enum Reflexion {
    Vertical,
    Horizontal
}

fn check_for_vertical_split(pattern: &Vec<Vec<char>>, allowed_smudges: usize) -> Vec<usize> {
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

    let mut possible_solutions = Vec::new();

    println!("{required_smudges_per_split_position:?}");
    for (index, smudges) in required_smudges_per_split_position.iter().enumerate() {
        if *smudges <= allowed_smudges {
            possible_solutions.push(index);
        }
    }

    possible_solutions
}

fn check_for_horizontal_split(pattern: &Vec<Vec<char>>, allowed_smudges: usize) -> Vec<usize> {
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

    let mut possible_solutions = Vec::new();

    println!("{required_smudges_per_split_position:?}");
    for (index, smudges) in required_smudges_per_split_position.iter().enumerate() {
        if *smudges <= allowed_smudges {
            possible_solutions.push(index);
        }
    }
    possible_solutions
}

fn find_reflexions(pattern: &Vec<Vec<char>>, allowed_smudges: usize) -> Vec<(Reflexion, usize)> {

    let mut reflexions = Vec::new();

    for reflexion in check_for_vertical_split(pattern, allowed_smudges) {
        reflexions.push((Reflexion::Vertical, reflexion));
    }
    for reflexion in check_for_horizontal_split(pattern, allowed_smudges) {
        reflexions.push((Reflexion::Horizontal, reflexion));
    }

    reflexions
}

fn main() {
    let mut total_part_1: usize = 0;
    let mut total_part_2: usize = 0;
    let mut pattern_id = 0;

    let mut pattern: Vec<Vec<char>> = Vec::new();
    for line in std::io::stdin().lines() {
        let line = line.unwrap();
        if line.is_empty() {
            println!("Pattern {pattern_id}");
            pattern_id += 1;

            let reflexions_without_smudge = find_reflexions(&pattern, 0);

            if reflexions_without_smudge.len() != 1 {
                panic!("Expected only 1 reflexion. Got {reflexions_without_smudge:?}");
            }

            total_part_1 += match reflexions_without_smudge[0] {
                (Reflexion::Vertical, position) => position + 1,
                (Reflexion::Horizontal, position) => (position + 1) * 100,
            };

            let mut reflexions_with_smudge = find_reflexions(&pattern, 1);
            reflexions_with_smudge.retain(|reflexion| reflexion != &reflexions_without_smudge[0]);

            if reflexions_with_smudge.len() != 1 {
                panic!("Expected only 1 reflexion. Got {reflexions_without_smudge:?}");
            }

            total_part_2 += match reflexions_with_smudge[0] {
                (Reflexion::Vertical, position) => position + 1,
                (Reflexion::Horizontal, position) => (position + 1) * 100,
            };

            pattern = Vec::new();
        } else {
            pattern.push(line.chars().collect())
        }
    }

    println!("Pattern {pattern_id}");
    let reflexions_without_smudge = find_reflexions(&pattern, 0);

    if reflexions_without_smudge.len() != 1 {
        panic!("Expected only 1 reflexion. Got {reflexions_without_smudge:?}");
    }

    total_part_1 += match reflexions_without_smudge[0] {
        (Reflexion::Vertical, position) => (position + 1),
        (Reflexion::Horizontal, position) => (position + 1) * 100,
    };

    let mut reflexions_with_smudge = find_reflexions(&pattern, 1);
    reflexions_with_smudge.retain(|reflexion| reflexion != &reflexions_without_smudge[0]);

    if reflexions_with_smudge.len() != 1 {
        panic!("Expected only 1 reflexion. Got {reflexions_without_smudge:?}");
    }

    total_part_2 += match reflexions_with_smudge[0] {
        (Reflexion::Vertical, position) => position + 1,
        (Reflexion::Horizontal, position) => (position + 1) * 100,
    };

    println!("Part 1: {total_part_1}, part 2: {total_part_2}");
}
