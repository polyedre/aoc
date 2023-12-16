use std::collections::HashMap;

use itertools::Itertools;

fn count_possible_solutions(springs: &Vec<char>, blocks: &Vec<usize>, depth: usize, cache: &mut HashMap<(Vec<char>, Vec<usize>), usize>) -> usize {
    // println!("{depth} Counting {:?} with blocks {:?}", springs, blocks);
    if springs.is_empty() {
        if blocks.is_empty() {
            // println!("{depth} Returning 1");
            return 1
        } else {
            // println!("{depth} Returning 0");
            return 0
        }
    };

    if blocks.is_empty() {
        if !springs.contains(&'#') {
            // println!("{depth} Returning 1");
            return 1
        } else {
            // println!("{depth} Returning 0");
            return 0
        }
    };

    if let Some(result) = cache.get(&(springs.clone(), blocks.clone())) {
        return *result;
    }

    let first_char: char = springs.first().unwrap().clone();

    let mut total = 0;

    if first_char == '.' || first_char == '?' {
        total += count_possible_solutions(&springs[1..].to_vec(), blocks, depth + 1, cache);
    };

    if first_char == '?' || first_char == '#' {

        if springs.len() == blocks[0] && !springs[..blocks[0]].contains(&'.') {
            total += count_possible_solutions(&springs[blocks[0]..].to_vec(), &blocks[1..].to_vec(), depth + 1, cache);
        } else if blocks[0] <= springs.len() && !springs[..blocks[0]].contains(&'.') && springs[blocks[0]] != '#' {
            total += count_possible_solutions(&springs[blocks[0] + 1..].to_vec(), &blocks[1..].to_vec(), depth + 1, cache)
        }

    }

    cache.insert((springs.clone(), blocks.clone()), total);
    total
}

fn main() {
    let lines: Vec<String> = std::io::stdin()
        .lines().map(|line| line.unwrap()).collect();

    let mut cache: HashMap<(Vec<char>, Vec<usize>), usize> = HashMap::new();

    let total_part_1 = lines.clone().iter()
        .map(|line| {
            let (springs, blocks) = line.split_once(' ').unwrap();
            count_possible_solutions(&springs.chars().collect(), &blocks.split(',').map(|num| num.parse().unwrap()).collect(), 0, &mut cache)
        })
        .sum::<usize>();

    println!("Part 1: {total_part_1}");

    let total_part_2 = lines.clone().iter()
        .map(|line| {
            let (springs, blocks) = line.split_once(' ').unwrap();
            let springs = std::iter::once(springs).cycle().take(5).join("?");
            let blocks: Vec<usize> = blocks.split(',').map(|num| num.parse().unwrap()).collect();
            let blocks_len = blocks.len();
            let blocks: Vec<usize> = blocks.into_iter().cycle().take(5 * blocks_len).collect();
            count_possible_solutions(&springs.chars().collect(), &blocks, 0, &mut cache)
        })
        .sum::<usize>();

    println!("Part 2: {total_part_2}");
}
