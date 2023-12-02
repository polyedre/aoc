use std::io;

const RED_MAX: u32 = 12;
const GREEN_MAX: u32 = 13;
const BLUE_MAX: u32 = 14;

fn main() {
    let mut total_part_1 = 0;
    let mut total_part_2 = 0;

    for (identifier, line) in io::stdin().lines().enumerate() {

        let mut possible = true;

        let line = line.unwrap();
        let mut parse_result = line.split(':');
        let _game_str = parse_result.next();

        let mut min_red = 0;
        let mut min_green = 0;
        let mut min_blue = 0;

        for subset in parse_result.next().unwrap().split(';') {
            for single_cube_draw in subset.trim().split(',') {
                let mut cube = single_cube_draw.trim().split(' ');
                let amount: &str = cube.next().unwrap();

                let amount: u32 = amount.parse().unwrap();
                let color = cube.next().unwrap();

                match color {
                    "red" => {
                        min_red = std::cmp::max(min_red, amount);
                        if amount > RED_MAX { possible = false }
                    },
                    "green" => {
                        min_green = std::cmp::max(min_green, amount);
                        if amount > GREEN_MAX { possible = false }
                    },
                    "blue" => {
                        min_blue = std::cmp::max(min_blue, amount);
                        if amount > BLUE_MAX { possible = false }
                    },
                    _ => panic!("Color {} uncovered", color)
                }
            }

            println!("{}", subset);
        }

        total_part_2 += min_red * min_green * min_blue;

        if possible {
            println!("Id is {}", identifier + 1);
            total_part_1 += identifier + 1;
        }
    }

    println!("Total part 1: {}", total_part_1);
    println!("Total part 2: {}", total_part_2);
}
