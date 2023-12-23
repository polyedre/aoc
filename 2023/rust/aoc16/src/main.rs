#[derive(Clone)]
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

    // println!("Simulating [{x}, {y}], {direction:?}, {}", tiles[x][y].value);

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
                if y < height - 1 { simulate_beam(tiles, x, y + 1, Direction::Down, cache); }
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
                if y < height - 1 { simulate_beam(tiles, x, y + 1, Direction::Down, cache); }
            } else if tiles[y][x].value == '\\' {
                if y < height - 1 { simulate_beam(tiles, x , y + 1, Direction::Down, cache); }
            } else if tiles[y][x].value == '/' {
                if y > 0 { simulate_beam(tiles, x, y - 1, Direction::Up, cache); }
            } else {
                if x < width - 1 {
                    simulate_beam(tiles, x + 1, y, Direction::Right, cache);
                }
            }
        },
    }
}

fn compute_panel_score(tiles: &Vec<Vec<Tile>>) -> usize {
    tiles.iter().map(|row| row.iter().filter(|tile| tile.energized).count()).sum()
}

fn main() {
    let mut tiles: Vec<Vec<Tile>> = Vec::new();

    for line in std::io::stdin().lines() {
        let line = line.unwrap();
        tiles.push(line.chars().map(|char| Tile::new(char)).collect());
    }

    // Part 1

    let mut part_1_tiles = tiles.clone();

    simulate_beam(&mut part_1_tiles, 0, 0, Direction::Right, &mut Vec::new());
    let result_part_1 = compute_panel_score(&part_1_tiles);

    println!("Part 1: {result_part_1}");

    // Part 2

    let mut max_energized_tiles = 0;

    for position in 0..tiles[0].len() {
        // Down
        let mut cloned_panel = tiles.clone();
        simulate_beam(&mut cloned_panel, position, 0, Direction::Down, &mut Vec::new());
        let score = compute_panel_score(&cloned_panel);
        max_energized_tiles = usize::max(score, max_energized_tiles);

        // Up
        let mut cloned_panel = tiles.clone();
        simulate_beam(&mut cloned_panel, position, tiles.len() - 1, Direction::Up, &mut Vec::new());
        let score = compute_panel_score(&cloned_panel);
        max_energized_tiles = usize::max(score, max_energized_tiles);
    }

    for position in 0..tiles.len() {
        // Right
        let mut cloned_panel = tiles.clone();
        simulate_beam(&mut cloned_panel, 0, position, Direction::Right, &mut Vec::new());
        let score = compute_panel_score(&cloned_panel);
        max_energized_tiles = usize::max(score, max_energized_tiles);

        // Up
        let mut cloned_panel = tiles.clone();
        simulate_beam(&mut cloned_panel, tiles[0].len() - 1, position, Direction::Left, &mut Vec::new());
        let score = compute_panel_score(&cloned_panel);
        max_energized_tiles = usize::max(score, max_energized_tiles);
    }

    println!("Part 2: {max_energized_tiles}");
}
