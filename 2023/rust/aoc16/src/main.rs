struct Tile {
    value: char,
    energized: bool
}

#[derive(Debug, PartialEq, Copy, Clone)]
enum Direction {
    Up,
    Down,
    Left,
    Right
}

impl Tile {
    fn new(value: char) -> Self {
        Tile { value, energized: false }
    }
}

fn simulate_beam(tiles: &mut Vec<Vec<Tile>>, x: usize, y: usize, direction: Direction, cache: &mut Vec<(usize, usize, Direction)>) {
    tiles[y][x].energized = true;

    if cache.contains(&(x, y, direction)) {
        return
    }

    println!("Simulating [{x}, {y}], {direction:?}, {}", tiles[x][y].value);

    cache.push((x, y, direction));

    let width = tiles[y].len();
    let height = tiles.len();

    match direction {
        Direction::Up => {
            if tiles[y][x].value == '-' {
                if x > 0 { simulate_beam(tiles, x - 1 , y, Direction::Left, cache); }
                if x < width - 1 { simulate_beam(tiles, x + 1, y, Direction::Right, cache); }
            } else if tiles[y][x].value == '\\' {
                if x > 0 { simulate_beam(tiles, x - 1 , y, Direction::Left, cache); }
            } else if tiles[y][x].value == '/' {
                if x < width - 1 { simulate_beam(tiles, x + 1, y, Direction::Right, cache); }
            } else {
                if y > 0 {
                    simulate_beam(tiles, x, y - 1, Direction::Up, cache);
                }
            }
        },
        Direction::Down => {
            if tiles[y][x].value == '-' {
                if x > 0 { simulate_beam(tiles, x - 1 , y, Direction::Left, cache); }
                if x < width - 1 { simulate_beam(tiles, x + 1, y, Direction::Right, cache); }
            } else if tiles[y][x].value == '\\' {
                if x < width - 1 { simulate_beam(tiles, x + 1 , y, Direction::Right, cache); }
            } else if tiles[y][x].value == '/' {
                if x > 0 { simulate_beam(tiles, x - 1, y, Direction::Left, cache); }
            } else {
                if y < height - 1 {
                    simulate_beam(tiles, x, y + 1, Direction::Down, cache);
                }
            }
        },
        Direction::Left => {
            if tiles[y][x].value == '|' {
                if y > 0 { simulate_beam(tiles, x , y - 1, Direction::Up, cache); }
                if x < height - 1 { simulate_beam(tiles, x, y + 1, Direction::Down, cache); }
            } else if tiles[y][x].value == '\\' {
                if y > 0 { simulate_beam(tiles, x , y - 1, Direction::Up, cache); }
            } else if tiles[y][x].value == '/' {
                if y < height - 1 { simulate_beam(tiles, x, y + 1, Direction::Down, cache); }
            } else {
                if x > 0 {
                    simulate_beam(tiles, x - 1, y, Direction::Left, cache);
                }
            }
        },
        Direction::Right => {
            if tiles[y][x].value == '|' {
                if y > 0 { simulate_beam(tiles, x , y - 1, Direction::Up, cache); }
                if x < height - 1 { simulate_beam(tiles, x, y + 1, Direction::Down, cache); }
            } else if tiles[y][x].value == '\\' {
                if y < height - 1 { simulate_beam(tiles, x , y + 1, Direction::Down, cache); }
            } else if tiles[y][x].value == '/' {
                if y > 0 { simulate_beam(tiles, x, y - 1, Direction::Up, cache); }
            } else {
                println!("Right at [{x}, {y}]");
                if x < width - 1 {
                    simulate_beam(tiles, x + 1, y, Direction::Right, cache);
                }
            }
        },
    }
}

fn main() {
    let mut tiles: Vec<Vec<Tile>> = Vec::new();

    for line in std::io::stdin().lines() {
        let line = line.unwrap();
        tiles.push(line.chars().map(|char| Tile::new(char)).collect());
    }

    simulate_beam(&mut tiles, 0, 0, Direction::Right, &mut Vec::new());

    let mut result_part_1 = 0;
    for row in tiles {
        println!("{}", row.iter().map(|tile| if tile.energized { "#" } else { "." }).collect::<String>());
        result_part_1 += row.iter().filter(|tile| tile.energized).count()
    }

    println!("Part 1: {result_part_1}");
}
