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

    let seeds: Vec<u64> = line.split(':')
                              .last()
                              .unwrap()
                              .trim()
                              .split_whitespace()
                              .map(|seed| seed.parse().unwrap())
                              .collect();

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

    println!("Solution of part 1: {}", locations.iter().min().unwrap());
}
