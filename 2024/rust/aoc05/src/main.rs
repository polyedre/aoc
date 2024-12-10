use core::fmt;
use core::result::Result;
use core::result::Result::Err;
use std::io::BufReader;
use std::io::prelude::*;
use std::fs::File;
use std::process::exit;
use std::time::Instant;

#[derive(Debug, Clone)]
struct UnsortableError;

impl fmt::Display for UnsortableError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Sequence is unsortable")
    }
}

fn main() {
    let filename: &str = &std::env::args().nth(1).expect("Expected filename");

    let now = Instant::now();
    let (part1, part2) = basic_implementation(filename);
    let elapsed = now.elapsed();

    println!("Basic implementation::");
    println!("Part 1: {}", part1);
    println!("Part 2: {}", part2);
    println!("Elapsed: {:.2?}", elapsed);

    let now = Instant::now();
    let (part1, part2) = optimized_implementation(filename);
    let elapsed = now.elapsed();

    println!("Optimized:");
    println!("Part 1: {}", part1);
    println!("Part 2: {}", part2);
    println!("Elapsed: {:.2?}", elapsed);
}

// Basic implementation

fn basic_implementation(filename: &str) -> (i32, i32) {
    let mut rules: Vec<(i32, i32)> = Vec::new();
    let mut updates: Vec<Vec<i32>> = Vec::new();

    let f = File::open(filename).expect("failed to open file");
    let f = BufReader::new(f);
    for line in f.lines() {
        let line: String = String::from(line.unwrap());

        if let Some((left, right)) = line.split_once('|') {
            rules.push((left.parse().unwrap(), right.parse().unwrap()));
        }

        match line.split(',').collect::<Vec<&str>>() {
            list if list.len() > 1 => updates.push(list.iter().map(|elem| elem.parse::<i32>().unwrap()).collect()),
            _ => {}
        }
    }

    let mut part1: i32 = 0;
    let mut part2: i32 = 0;

    for mut update in updates {
        if is_ordered(&update, &rules) {
            part1 += get_middle(&update);
        } else {
            sort(&mut update, &rules);
            part2 += get_middle(&update);
        }
    }

    (part1, part2)
}

fn is_ordered(update: &Vec<i32>, rules: &Vec<(i32, i32)>) -> bool {
    for i in 1..update.len() {
        for &(left, right) in rules {
            if left == update[i] {
                if (&update[0..i]).contains(&right) {
                    // println!("DEBUG: {:?} is invalid because of rule {:?}", update, (left, right));
                    return false;
                }
            }
        }
    }

    true
}

fn sort(update: &mut Vec<i32>, rules: &Vec<(i32, i32)>) -> () {
    let mut i: usize = 0;
    'outer: while i < update.len() {
        for &(left, right) in rules {
            if left == update[i] {
                if let Some(destination) = (&update[i..update.len()]).iter().position(|elem| elem == &right) {
                    // println!("DEBUG: {:?} is invalid at index {i} because of rule {:?}. {right} has been found at position {destination}. We need to swap.", update, (left, right));
                    let tmp = update[i];
                    update[i] = update[destination + i];
                    update[destination + i] = tmp;
                    // println!("DEBUG: Swapped elements {i} with {destination}: {:?}.", update);
                    continue 'outer;
                }
            }
        }

        i += 1;
    }
}

// Optimized

fn optimized_implementation(filename: &str) -> (u32, u32) {
    let mut adjacents: [Vec<u8>; 255] = std::iter::repeat_with(|| Vec::new())
        .take(255)
        .collect::<Vec<_>>()
        .try_into()
        .unwrap();
    let mut sequences: Vec<Vec<u8>> = Vec::new();

    let f = File::open(filename).expect("failed to open file");
    let f = BufReader::new(f);
    for line in f.lines() {
        let line: String = String::from(line.unwrap());

        if let Some((left, right)) = line.split_once('|') {
            adjacents[left.parse::<usize>().unwrap() as usize].push(right.parse().unwrap());
        }

        match line.split(',').collect::<Vec<&str>>() {
            list if list.len() > 1 => sequences.push(list.iter().map(|elem| elem.parse().unwrap()).collect()),
            _ => {}
        }
    }

    let mut part1 = 0;
    let mut part2 = 0;

    for mut page_sequence in sequences {
        if is_topologically_sorted(&page_sequence, &adjacents) {
            part1 += get_middle(&page_sequence) as u32;
        } else {
            match sort_topologically(&mut page_sequence, &adjacents) {
                Ok(sorted_sequence) => {
                    // println!("{:?} has been sorted to {:?}", page_sequence, sorted_sequence);
                    part2 += get_middle(&sorted_sequence) as u32 },
                Err(e) => { eprintln!("ERROR: {e}"); exit(1); }
            };
        }
    }

    (part1, part2)
}

fn get_middle<T>(update: &Vec<T>) -> T where T: Copy {
    return update[update.len() / 2]
}

fn is_topologically_sorted(page_sequence: &Vec<u8>, adjacents: &[Vec<u8>; 255]) -> bool {
    let mut indegree: [u8; 255] = [0; 255];

    compute_indegree(page_sequence, adjacents, &mut indegree);

    for page in page_sequence {
        if indegree[*page as usize] == 0 {
            for &adj in &adjacents[*page as usize] {
                indegree[adj as usize] -= 1;
            }
        } else {
            return false;
        }
    }
    true
}

fn compute_indegree(page_sequence: &Vec<u8>, adjacents: &[Vec<u8>; 255], indegree: &mut [u8; 255]) {
    for page in page_sequence {
        for &adj in &adjacents[*page as usize] {
            indegree[adj as usize] += 1;
        }
    }
}

fn sort_topologically(page_sequence: &mut Vec<u8>, adjacents: &[Vec<u8>; 255]) -> Result<Box<Vec<u8>>, UnsortableError> {
    let mut indegree: [u8; 255] = [0; 255];
    let queue = page_sequence;
    let mut sorted_sequence: Box<Vec<u8>> = Box::new(Vec::new());

    compute_indegree(queue, adjacents, &mut indegree);

    'outer: while !queue.is_empty() {
        for (index, page) in queue.iter().enumerate() {
            if indegree[*page as usize] == 0 {
                for &adj in &adjacents[*page as usize] {
                    indegree[adj as usize] -= 1;
                }

                sorted_sequence.push(queue.remove(index));
                continue 'outer;
            }
        }

        // println!("{:?}", queue.iter().map(|page| (*page, *indegree.get(&page).unwrap_or(&42))).collect::<Vec<(u8, u8)>>());
        return Err(UnsortableError)
    }

    Ok(sorted_sequence)
}
