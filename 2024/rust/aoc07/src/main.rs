use std::io::BufReader;
use std::io::prelude::*;
use std::{fs::File, time::Instant};

fn is_possible(equation: &[u64], sum: u64, operations: &[fn(u64, u64) -> u64]) -> bool {
    match equation {
        [] => sum == 0,
        [elem] => *elem == sum,
        [first, second, remainder @ ..] => {
            operations.iter().any(|op| {
                let next = op(*first, *second);
                let new_equation: Vec<u64> = std::iter::once(next).chain(remainder.iter().cloned()).collect();
                is_possible(&new_equation, sum, operations)
            })
        }
    }
}

fn parse_input(filename: &str) -> Vec<(u64, Vec<u64>)> {
    let f = File::open(filename).expect("failed to open file");
    let f = BufReader::new(f);

    f.lines()
        .map(|line| {
            let line = line.unwrap();
            let (sum, equation) = line.split_once(':').unwrap();
            let sum = sum.parse::<u64>().unwrap();
            let equation = equation.split_whitespace()
                .map(|elem| elem.parse().unwrap())
                .collect();
            (sum, equation)
        })
        .collect()
}

fn main() {
    let filename: &str = &std::env::args().nth(1).expect("Expected filename");
    let input = parse_input(filename);

    let add = |a: u64, b: u64| a + b;
    let multiply = |a: u64, b: u64| a * b;
    let concat: fn(u64, u64) -> u64 = |a, b| {
        (a.to_string() + &b.to_string())
            .parse()
            .expect("Concatenation failed: invalid number format")
    };

    let operations: &[fn(u64, u64) -> u64] = &[add, multiply, concat];

    let now = Instant::now();
    let part1: u64 = input.iter()
        .filter(|(sum, eq)| is_possible(&eq, *sum, &operations[..2]))
        .map(|(sum, _)| *sum)
        .sum();

    let part2: u64 = input.iter()
        .filter(|(sum, eq)| is_possible(&eq, *sum, operations))
        .map(|(sum, _)| *sum)
        .sum();
    let elapsed = now.elapsed();

    println!("Optimized:");
    println!("Part 1: {}", part1);
    println!("Part 2: {}", part2);
    println!("Elapsed: {:.2?}", elapsed);
}
