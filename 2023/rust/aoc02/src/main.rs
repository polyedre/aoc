use std::io;

const RED_MAX: u32 = 12;
const GREEN_MAX: u32 = 13;
const BLUE_MAX: u32 = 14;

fn main() {
    let mut total = 0;

    for (identifier, line) in io::stdin().lines().enumerate() {

        let mut possible = true;

        let line = line.unwrap();
        let mut parse_result = line.split(':');
        let _game_str = parse_result.next();

        for subset in parse_result.next().unwrap().split(';') {
            for single_cube_draw in subset.trim().split(',') {
                let mut cube = single_cube_draw.trim().split(' ');
                let amount: &str = cube.next().unwrap();

                let amount: u32 = amount.parse().unwrap();
                let color = cube.next().unwrap();

                match color {
                    "blue" => { if amount > BLUE_MAX { possible = false } },
                    "red" => { if amount > RED_MAX { possible = false } },
                    "green" => { if amount > GREEN_MAX { possible = false } },
                    _ => panic!("Color {} uncovered", color)
                }

            }

            println!("{}", subset);
        }

        if possible {
            println!("Id is {}", identifier + 1);
            total += identifier + 1;
        }
    }

    println!("{}", total)
}
