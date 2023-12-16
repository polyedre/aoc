fn count_possible_solutions(springs: &Vec<char>, blocks: &Vec<usize>, depth: usize) -> usize {
    println!("{depth} Counting {:?} with blocks {:?}", springs, blocks);
    if springs.is_empty() {
        if blocks.is_empty() {
            println!("{depth} Returning 1");
            return 1
        } else {
            println!("{depth} Returning 0");
            return 0
        }
    };

    if blocks.is_empty() {
        if !springs.contains(&'#') {
            println!("{depth} Returning 1");
            return 1
        } else {
            println!("{depth} Returning 0");
            return 0
        }
    };

    let first_char: char = springs.first().unwrap().clone();

    let mut total = 0;

    if first_char == '.' || first_char == '?' {
        total += count_possible_solutions(&springs[1..].to_vec(), blocks, depth + 1)
    };

    if first_char == '?' || first_char == '#' {

        if springs.len() == blocks[0] && !springs[..blocks[0]].contains(&'.') {
            total += count_possible_solutions(&springs[blocks[0]..].to_vec(), &blocks[1..].to_vec(), depth + 1)
        } else if blocks[0] <= springs.len() && !springs[..blocks[0]].contains(&'.') && springs[blocks[0]] != '#' {
            total += count_possible_solutions(&springs[blocks[0] + 1..].to_vec(), &blocks[1..].to_vec(), depth + 1)
        }

    }

    println!("{depth} Returning {total}");
    total
}

fn main() {
    let total = std::io::stdin()
        .lines()
        .map(|line| {
            let line = line.unwrap();
            let (springs, blocks) = line.split_once(' ').unwrap();
            count_possible_solutions(&springs.chars().collect(), &blocks.split(',').map(|num| num.parse().unwrap()).collect(), 0)
        })
        .sum::<usize>();

    println!("Part 1: {total}");
}
