use std::io;
use std::collections::HashMap;
use num::integer::lcm;

struct Instruction {
    left: String,
    right: String
}

fn get_counts_to_z<'a>(direction: &'a String, map: &'a HashMap<String, Instruction>, initial_node: &'a String) -> (u64, &'a String) {
    let mut current_node = initial_node;
    let mut step_count = 0;
    'outer: loop {
        for choice in direction.chars() {
            let next_node = match choice {
                'L' => &map.get(current_node).unwrap().left,
                'R' => &map.get(current_node).unwrap().right,
                _ => panic!("Choice {choice} is invalid!")
            };
            // println!("Current node is {}, going {} to {}", current_node, choice.to_string(), next_node);
            current_node = next_node;
            step_count += 1;

            if current_node.ends_with("Z") { break 'outer }
        }
    }

    return (step_count, current_node);
}

fn main() {
    let mut map: HashMap<String, Instruction> = HashMap::new();

    let mut input = io::stdin().lines();

    let direction = input.next().unwrap().unwrap();
    let _empty_line = input.next().unwrap().unwrap();

    for node in input {
        let node = node.unwrap();

        let node_id = &node[..node.find(' ').unwrap()];
        let left = &node[(node.find('(').unwrap() + 1)..node.find(',').unwrap()];
        let right = &node[(node.rfind(' ').unwrap() + 1)..node.find(')').unwrap()];

        map.insert(node_id.to_string(), Instruction { left: left.to_string(), right: right.to_string() });
    }

    if map.contains_key("AAA") {
        let (step_count, _last_node) = get_counts_to_z(&direction, &map, &"AAA".to_string());
        println!("Solution part 1: {}", step_count);
    }

    let initial_nodes: Vec<&String> = map.keys().filter(|node_id| node_id.ends_with("A")).collect();

    println!("Initial nodes: {:?}", initial_nodes);

    let mut steps = Vec::new();
    // let mut product: u64 = 1;

    for initial_node in initial_nodes {
        let (step_count_to_first_z, first_z_node) = get_counts_to_z(&direction, &map, initial_node);
        let (step_count_to_second_z, second_z_node) = get_counts_to_z(&direction, &map, first_z_node);
        println!("{initial_node} can reach {first_z_node} node in {step_count_to_first_z} steps, and then {step_count_to_second_z} steps to {second_z_node}");
        steps.push(step_count_to_first_z);
    }

    let mut solution = steps.pop().unwrap();

    for step in steps {
        solution = lcm(solution, step);
    }

    println!("Solution part 2: {:?}", solution);
}
