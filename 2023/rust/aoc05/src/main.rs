use std::io;

#[derive(Clone)]
struct Association {
    dst_start: u64,
    src_start: u64,
    length: u64
}

impl Association {
    fn find_association(&self, key: u64) -> Option<u64> {
        if key >= self.src_start && key < self.src_start + self.length {
            return Some(key - self.src_start + self.dst_start);
        }
                                           return None;
    }
}

fn main() {
    let mut mapping: Vec<Vec<Association>> = Vec::new();

    let mut all_lines = io::stdin().lines();

    let line = all_lines.next().unwrap().unwrap();

    let mut seed_elements = line.split(':').last().unwrap().trim().split_whitespace();
    let seeds: Vec<u64> = seed_elements.clone().map(|seed| seed.parse().unwrap()).collect();
    let mut seeds_part_2: Vec<(u64, u64)> = Vec::new();

    loop {
        match &seed_elements.next() {
            Some(seed_id) => { seeds_part_2.push((seed_id.parse().unwrap(), seed_elements.next().unwrap().parse().unwrap())); },
            None => break
        }
    }

    let mut current_associations = Vec::new();

    for line in all_lines {
        let line = line.unwrap();

        if line.is_empty() { continue; }

        if line.contains("map") {
            mapping.push(current_associations.clone());
            current_associations = Vec::new();
        } else {
            let [dst_start, src_start, length]: [u64; 3] = line.trim().split_whitespace()
                                                            .map(|num| num.parse().unwrap())
                                                            .collect::<Vec<u64>>().try_into().unwrap();
            current_associations.push(Association { dst_start, src_start, length })
        }
    }

    mapping.push(current_associations.clone());

    // Part 1

    let mut locations = Vec::new();

    for seed in seeds {

        let mut element_number = seed;
        // println!("New seed {}", seed);

        'outer: for associations in &mapping {
            for association in associations {
               match association.find_association(element_number) {
                   Some(value) => { element_number = value; continue 'outer; },
                   None => {}
               }
            }
        }

        locations.push(element_number);
    }

    // Part 2

    let mut min_location: u64 = u64::MAX;

    for (seed_start, length) in seeds_part_2 {

        for seed in seed_start..(seed_start + length) {
            let mut element_number = seed;

            'outer: for associations in &mapping {
                for association in associations {
                match association.find_association(element_number) {
                    Some(value) => { element_number = value; continue 'outer; },
                    None => {}
                }
                }
            }

            min_location = u64::min(min_location, element_number);
        }

    }

    println!("Solution of part 1: {}", locations.iter().min().unwrap());
    println!("Solution of part 2: {}", min_location);
}
