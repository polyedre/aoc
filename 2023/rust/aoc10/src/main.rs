use std::{io, collections::HashMap};

#[derive(Debug)]
enum TileState {
    UpPipe,
    GroundOrUselessPipes,
    Pipe,
    DownPipe,
}

fn main() {
    let mut total_part_1 = 0;
    let mut total_part_2 = 0;
    let mut total_part_2_bis = 0;

    let pipes: Vec<Vec<char>> = io::stdin().lines().map(|line| line.unwrap().chars().collect()).collect();
    let mut directions: Vec<Vec<TileState>> = Vec::new();

    for row in 0..pipes.len() {
        let mut line = Vec::new();
        for _col in  0..pipes[row].len() {
            line.push(TileState::GroundOrUselessPipes);
        }
        directions.push(line);
    }

    let (mut previous_x, mut previous_y) = (0, 0);
    let (mut current_x, mut current_y) = (0, 0);
    let (mut next_x, mut next_y) = (0, 0);

    let height = pipes.len();
    let width = pipes[0].len();
    for y in 0..height {
        for x in 0..width {
            if pipes[y][x] == 'S' {
                (current_x, current_y) = (x, y);
            }
        }
    }

    while pipes[next_y][next_x] != 'S' {
        let current_pipe = pipes[current_y][current_x];
        (next_x, next_y) = match current_pipe {
            'S' => {
                let mut result = (0, 0);
                // Try left
                // println!("Testing '{}' ({current_y}, {})", pipes[current_y][current_x + 1], current_x + 1);
                if current_x < width && (pipes[current_y][current_x + 1] == '-' || pipes[current_y][current_x + 1] == 'J' || pipes[current_y][current_x + 1] == '7'){
                    result = (current_x + 1, current_y);
                }

                // Try right
                // println!("Testing '{}' ({current_y}, {})", pipes[current_y][current_x - 1], current_x - 1);
                if current_x > 0 && (pipes[current_y][current_x - 1] == '-' || pipes[current_y][current_x - 1] == 'L' || pipes[current_y][current_x - 1] == 'F'){
                    result = (current_x - 1, current_y);
                }

                result
            },
            'J' if current_x == previous_x => { directions[current_y][current_x] = TileState::Pipe; (current_x - 1, current_y) },
            'J' if current_y == previous_y => { directions[current_y][current_x] = TileState::Pipe; (current_x, current_y - 1) },
            'F' if current_x == previous_x => { directions[current_y][current_x] = TileState::UpPipe; (current_x + 1, current_y) },
            'F' if current_y == previous_y => { directions[current_y][current_x] = TileState::DownPipe; (current_x, current_y + 1) },
            '7' if current_x == previous_x => { directions[current_y][current_x] = TileState::UpPipe; (current_x - 1, current_y) },
            '7' if current_y == previous_y => { directions[current_y][current_x] = TileState::DownPipe; (current_x, current_y + 1) },
            'L' if current_x == previous_x => { directions[current_y][current_x] = TileState::Pipe; (current_x + 1, current_y) },
            'L' if current_y == previous_y => { directions[current_y][current_x] = TileState::Pipe ; (current_x, current_y - 1)},
            '-' if current_x < previous_x =>  { directions[current_y][current_x] = TileState::Pipe ; (current_x - 1, current_y)},
            '-' if current_x > previous_x =>  { directions[current_y][current_x] = TileState::Pipe ; (current_x + 1, current_y)},
            '|' if current_y < previous_y => { directions[current_y][current_x] = TileState::UpPipe; (current_x, current_y - 1) },
            '|' if current_y > previous_y => { directions[current_y][current_x] = TileState::DownPipe; (current_x, current_y + 1) },
            '.' => (current_x + 1, current_y),
            _ => panic!("Char '{}' is a unknown pipe type.", current_pipe.to_string())
        };

        (previous_x, previous_y) = (current_x, current_y);
        (current_x, current_y) = (next_x, next_y);

        total_part_1 += 1;

        println!("Going '{}' ({next_x}, {next_y})", pipes[next_y][next_x]);
    }

    total_part_1 /= 2;
    println!("Part 1 solution is {total_part_1}");

    if previous_y < next_y {
        directions[next_y][next_x] = TileState::DownPipe;
    } else if previous_y > next_y {
        directions[next_y][next_x] = TileState::UpPipe;
    }

    for y in 0..height {
        let mut inside_depht = 0;
        let mut inside = 0;
        for x in 0..width {
            // println!("Looking at '{}' ({}, {}) {:?}. Inside depth: {}", pipes[y][x].to_string(), y, x, directions[y][x], inside_depht);
            match directions[y][x] {
                TileState::DownPipe => { if inside == 0 { inside = 1; } else { inside = 0; } ; inside_depht -= 1; print!("{}", pipes[y][x]) },
                TileState::GroundOrUselessPipes => {
                    if inside_depht != 0 {
                        print!(".");
                        total_part_2 += 1;
                        total_part_2_bis += inside;
                    } else {
                        print!(" ");
                    }
                },
                TileState::UpPipe => {  if inside == 0 { inside = 1; } else { inside = 0; }; inside_depht += 1; print!("{}", pipes[y][x]); },
                TileState::Pipe => { print!("{}", pipes[y][x]); },
            }
        }

        print!("\n");
    }

    println!("Part 2 solution is {total_part_2}");
    println!("Part 2 bis solution is {total_part_2_bis}");
}
