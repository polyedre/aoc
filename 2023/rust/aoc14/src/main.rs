fn cycle(mut platform: Vec<Vec<char>>) -> Vec<Vec<char>> {
    // North
    for x in 0..platform[0].len() {
        for y in 0..platform.len() {
            if platform[y][x] == 'O' {
                let mut new_y = y;
                while new_y > 0 && platform[new_y - 1][x] == '.' {
                    new_y -= 1;
                }

                platform[y][x] = '.';
                platform[new_y][x] = 'O';
            }
        }
    }

    // West
    for y in 0..platform.len() {
        for x in 0..platform[0].len() {
            if platform[y][x] == 'O' {
                let mut new_x = x;
                while new_x > 0 && platform[y][new_x - 1] == '.' {
                    new_x -= 1;
                }

                platform[y][x] = '.';
                platform[y][new_x] = 'O';
            }
        }
    }

    // South
    for x in 0..platform[0].len() {
        for y in (0..platform.len()).rev() {
            if platform[y][x] == 'O' {
                let mut new_y = y;
                while new_y < (platform.len() - 1) && platform[new_y + 1][x] == '.' {
                    new_y += 1;
                }

                platform[y][x] = '.';
                platform[new_y][x] = 'O';
            }
        }
    }

    // East
    for x in (0..platform[0].len()).rev() {
        for y in 0..platform.len() {
            if platform[y][x] == 'O' {
                let mut new_x = x;
                while new_x < (platform[0].len() - 1) && platform[y][new_x + 1] == '.' {
                    new_x += 1;
                }

                platform[y][x] = '.';
                platform[y][new_x] = 'O';
            }
        }
    }

    platform
}

fn main() {
    let mut platform: Vec<Vec<char>> = Vec::new();

    for line in std::io::stdin().lines() {
        let line = line.unwrap();
        platform.push(line.chars().collect());
    }

    let mut previous_platforms = Vec::new();

    let mut iteration = 0;
    while iteration < 1_000_000_000 {
        iteration += 1;

        let cycled_platform = cycle(platform.clone());

        match previous_platforms.iter().position(|platform: &Vec<Vec<char>>| *platform == cycled_platform) {
            Some(position) => {
                let mut cursor = position;
                while iteration < 1_000_000_000 {
                    iteration += 1;
                    cursor += 1;
                    if cursor >= previous_platforms.len() {
                        cursor = position;
                    }
                }

                platform = previous_platforms[cursor].clone();
                break;
            },
            None =>  {
                platform = cycled_platform;
                previous_platforms.push(platform.clone());
            }
        }
    }

    let mut platform_score = 0;
    for x in 0..platform[0].len() {
        for y in 0..platform.len() {
            if platform[y][x] == 'O' {
                platform_score += platform.len() - y
            }
        }
    }
    println!("{platform_score}");
}
