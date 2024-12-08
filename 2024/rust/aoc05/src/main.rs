use core::cmp::Ordering;
use std::io;

fn main() {
    let mut rules: Vec<(i32, i32)> = Vec::new();
    let mut updates: Vec<Vec<i32>> = Vec::new();

    let lines = io::stdin().lines().into_iter();
    for line in lines {
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

    println!("Part 1: {}", part1);
    println!("Part 2: {}", part2);
}

fn get_middle(update: &Vec<i32>) -> i32 {
    return update[update.len() / 2]
}

fn is_ordered(update: &Vec<i32>, rules: &Vec<(i32, i32)>) -> bool {
    for i in 1..update.len() {
        for &(left, right) in rules {
            if left == update[i] {
                if (&update[0..i]).contains(&right) {
                    println!("DEBUG: {:?} is invalid because of rule {:?}", update, (left, right));
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
                    println!("DEBUG: {:?} is invalid at index {i} because of rule {:?}. {right} has been found at position {destination}. We need to swap.", update, (left, right));
                    let tmp = update[i];
                    update[i] = update[destination + i];
                    update[destination + i] = tmp;
                    println!("DEBUG: Swapped elements {i} with {destination}: {:?}.", update);
                    continue 'outer;
                }
            }
        }

        i += 1;
    }
}
