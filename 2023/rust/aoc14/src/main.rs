fn main() {
    let mut platform: Vec<Vec<char>> = Vec::new();
    let mut total_part_1: usize = 0;

    for line in std::io::stdin().lines() {
        let line = line.unwrap();
        platform.push(line.chars().collect());
    }

    for x in 0..platform.len() {
        for y in 0..platform[0].len() {
            if platform[y][x] == 'O' {
                let mut new_y = y;
                while new_y > 0 && platform[new_y - 1][x] == '.' {
                    new_y -= 1;
                }

                platform[y][x] = '.';
                platform[new_y][x] = 'O';

                total_part_1 += platform.len() - new_y
            }
        }
    }

    println!("Part 1: {}", total_part_1);
}
