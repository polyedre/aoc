use std::io;

struct Galaxy {
    col: u32,
    row: u32
}

fn main() {

    let mut universe: Vec<String> = io::stdin().lines().map(|line| line.unwrap()).collect();

    let mut height = universe.len();
    let mut width = universe[0].len();

    // Expansion

    let mut current_row = 0;
    while current_row < height {
        if !universe[current_row].contains("#") {
            universe.insert(current_row, universe[current_row].clone());
            current_row += 1;
            height += 1;
        }

        current_row += 1;
    }

    let mut current_col = 0;
    while current_col < width {
        if (0..height).all(|row| universe[row].chars().nth(current_col).unwrap() == '.') {
            for row_id in 0..height {
                universe[row_id].insert(current_col, '.');
            }
            current_col += 1;
            width += 1;
        }

        current_col += 1;
    }

    let mut galaxies: Vec<(usize, usize)> = Vec::new();

    for row in 0..height {
        for col in 0..width {
            if universe[row].chars().nth(col).unwrap() == '#' {
                galaxies.push((row, col));
            }
        }
    }

    let mut sum_distance = 0;
    for start_galaxy_id in 0..galaxies.len() {
        for dst_galaxy_id in (start_galaxy_id + 1)..galaxies.len() {
            let (start_row, start_col) = galaxies[start_galaxy_id];
            let (dst_row, dst_col) = galaxies[dst_galaxy_id];

            sum_distance += i32::abs_diff(start_row as i32, dst_row as i32) + i32::abs_diff(start_col as i32, dst_col as i32);
        }
    }

    println!("Solution part 1: {sum_distance}");
}
