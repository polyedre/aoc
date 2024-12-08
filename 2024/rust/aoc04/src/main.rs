use std::io;

fn check_mas(map: &Vec<Vec<char>>, positions: &[((i32, i32), char)]) -> usize {
    if positions.iter()
                .all(|((col, row), c)|
                     *col >= 0 &&
                     *row >= 0 &&
                     *col < map[0].len() as i32 &&
                     *row < map.len() as i32 &&
                     map[*row as usize][*col as usize] == *c
                ) {
        return 1
    }
    0
}

fn main() {
    let mut map: Vec<Vec<char>> = Vec::new();

    for line in io::stdin().lines() {
        map.push(line.unwrap().chars().collect())
    }

    let width = map[0].len();
    let height = map.len();

    let mut xmass: usize = 0;
    let mut x_mass: usize = 0;

    for col in 0..width {
        for row in 0..height {
            if map[row][col] == 'X' {
                let col = col as i32;
                let row = row as i32;
                xmass += check_mas(&map, &[((col-1, row), 'M'),    ((col-2, row), 'A'), ((col-3, row), 'S')]);
                xmass += check_mas(&map, &[((col+1, row), 'M'),    ((col+2, row), 'A'), ((col+3, row), 'S')]);
                xmass += check_mas(&map, &[((col-1, row+1), 'M'),  ((col-2, row+2), 'A'), ((col-3, row+3), 'S')]);
                xmass += check_mas(&map, &[((col+1, row-1), 'M'),  ((col+2, row-2), 'A'), ((col+3, row-3), 'S')]);
                xmass += check_mas(&map, &[((col, row-1), 'M'),    ((col, row-2), 'A'), ((col, row-3), 'S')]);
                xmass += check_mas(&map, &[((col, row+1), 'M'),    ((col, row+2), 'A'), ((col, row+3), 'S')]);
                xmass += check_mas(&map, &[((col+1, row+1), 'M'),  ((col+2, row+2), 'A'), ((col+3, row+3), 'S')]);
                xmass += check_mas(&map, &[((col-1, row-1), 'M'),  ((col-2, row-2), 'A'), ((col-3, row-3), 'S')]);
            }
            if map[row][col] == 'A' {
                let col = col as i32;
                let row = row as i32;
                x_mass += check_mas(&map, &[((col-1, row-1), 'M'),  ((col-1, row+1), 'M'), ((col+1, row-1), 'S'),  ((col+1, row+1), 'S')]);
                x_mass += check_mas(&map, &[((col-1, row-1), 'S'),  ((col-1, row+1), 'S'), ((col+1, row-1), 'M'),  ((col+1, row+1), 'M')]);
                x_mass += check_mas(&map, &[((col-1, row-1), 'S'),  ((col-1, row+1), 'M'), ((col+1, row-1), 'S'),  ((col+1, row+1), 'M')]);
                x_mass += check_mas(&map, &[((col-1, row-1), 'M'),  ((col-1, row+1), 'S'), ((col+1, row-1), 'M'),  ((col+1, row+1), 'S')]);
            }
        }
    }

    println!("Part 1: {}", xmass);
    println!("Part 2: {}", x_mass);
}
