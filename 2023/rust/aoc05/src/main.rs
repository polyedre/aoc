use std::io;
use std::collections::HashMap;

fn main() {
    let mut total = 0;
    let mut total_part_2 = 0;

    let mut mapping: Vec<HashMap<u32, u32>> = Vec::new();

    let mut all_lines = io::stdin().lines();

    let line = all_lines.next().unwrap().unwrap();

    let seeds: Vec<u32> = line.split(':')
                              .last()
                              .unwrap()
                              .trim()
                              .split_whitespace()
                              .map(|seed| seed.parse().unwrap())
                              .collect();

    let mut current_association = HashMap::new();

    for line in all_lines {
        let line = line.unwrap();

        if line.is_empty() { continue; }

        if line.contains("map") {
            mapping.push(current_association.clone());
            current_association = HashMap::new();
        } else {
            let [dst_start, src_start, length]: [u32; 3] = line.trim().split_whitespace()
                                                            .map(|num| num.parse().unwrap())
                                                            .collect::<Vec<u32>>().try_into().unwrap();

            for index in 0..length {
                current_association.insert(src_start + index, dst_start + index);
            }
        }
    }

    mapping.push(current_association.clone());

    let mut locations = Vec::new();

    for seed in seeds {

        let mut element_number = seed;
        // println!("New seed {}", seed);

        for association in &mapping {
            let new_element = *association.get(&element_number).unwrap_or(&element_number);
            // println!("Element {} is associated with {}", element_number, new_element);
            element_number = new_element;
        }

        locations.push(element_number);
    }

    println!("Solution of part 1: {}", locations.iter().min().unwrap());
    println!("Total part 2: {}", total_part_2);
}
