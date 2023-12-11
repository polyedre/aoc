use std::io;

fn main() {
    let universe: Vec<String> = io::stdin().lines().map(|line| line.unwrap()).collect();

    let height = universe.len();
    let width = universe[0].len();

    for (part, expansion_constant) in vec![1, 999999].iter().enumerate() {

        let mut galaxies: Vec<(usize, usize)> = Vec::new();

        for row in 0..height {
            for col in 0..width {
                if universe[row].chars().nth(col).unwrap() == '#' {
                    galaxies.push((row, col));
                }
            }
        }

        // Expansion

        let mut expanded_galaxies = galaxies.clone();

        for (id, row) in universe.iter().enumerate() {
            if !row.contains("#") {
                for (galaxy_id, (galaxy_row, _)) in galaxies.iter().enumerate() {
                    if galaxy_row > &id {
                        expanded_galaxies[galaxy_id] = (expanded_galaxies[galaxy_id].0 + expansion_constant, expanded_galaxies[galaxy_id].1);
                    }
                }
            }
        }

        for current_col in 0..universe[0].len() {
            if (0..height).all(|row| universe[row].chars().nth(current_col).unwrap() == '.') {
                for (galaxy_id, (_, galaxy_col)) in galaxies.iter().enumerate() {
                    if galaxy_col > &current_col {
                        expanded_galaxies[galaxy_id] = (expanded_galaxies[galaxy_id].0, expanded_galaxies[galaxy_id].1 + expansion_constant);
                    }
                }
            }
        }

        let mut sum_distance = 0;
        for start_galaxy_id in 0..expanded_galaxies.len() {
            for dst_galaxy_id in (start_galaxy_id + 1)..expanded_galaxies.len() {
                let (start_row, start_col) = expanded_galaxies[start_galaxy_id];
                let (dst_row, dst_col) = expanded_galaxies[dst_galaxy_id];

                sum_distance += i64::abs_diff(start_row as i64, dst_row as i64) + i64::abs_diff(start_col as i64, dst_col as i64);
            }
        }

        println!("Solution part {}: {sum_distance}", part + 1);
    }
}
