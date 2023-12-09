use std::io;
use std::collections::HashMap;

struct Instruction {
    left: String,
    right: String
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

    let mut current_node = &"AAA".to_string();
    let mut step_count = 0;
    'outer: loop {
        for choice in direction.chars() {
            let next_node = match choice {
                'L' => &map.get(current_node).unwrap().left,
                'R' => &map.get(current_node).unwrap().right,
                _ => panic!("Choice {choice} is invalid!")
            };
            println!("Current node is {}, going {} to {}", current_node, choice.to_string(), next_node);
            current_node = next_node;
            step_count += 1;

            if *current_node == "ZZZ" { break 'outer }
        }
    }

    println!("Solution part 1: {}", step_count);
}
