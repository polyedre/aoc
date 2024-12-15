use std::io::BufReader;
use std::io::prelude::*;
use std::{fs::File, time::Instant};

fn is_possible(equation: &[u64], sum: u64, operations: &Vec<fn(u64, u64) -> u64>) -> bool {
    match equation {
        [] => sum == 0,
        [elem] => *elem == sum,
        [first, second, remainder @ ..]  => {
            operations.iter().any(|op| is_possible(&[&[op(*first, *second)], remainder].concat(), sum, operations))
        }
    }
}

fn main() {
    let filename: &str = &std::env::args().nth(1).expect("Expected filename");
    let f = File::open(filename).expect("failed to open file");
    let f = BufReader::new(f);

    let now = Instant::now();

   let input: Vec<(u64, Vec<u64>)> = f.lines()
        .map(|line| line.unwrap())
        .map(|line| {
            let (sum, equation) = line.split_once(':').unwrap();
            let sum = sum.parse::<u64>().unwrap();
            let equation = equation.split_whitespace()
                .map(|elem| elem.parse().unwrap())
                .collect::<Vec<u64>>();
            (sum, equation)
        })
        .collect();

    let add = |a: u64, b: u64| a + b;
    let multiply = |a: u64, b: u64| a * b;
    let concat: fn(u64, u64) -> u64 = |a: u64, b: u64| (a.to_string().to_owned() + &b.to_string().to_owned()).parse().expect("Failure to parse concatenated string as u64");

    let part1: u64 = input.iter()
        .filter(|(sum, eq)| is_possible(&eq, *sum, &vec![add, multiply]))
        .map(|(sum, _)| sum)
        .sum();

    let part2: u64 = input.iter()
        .filter(|(sum, eq)| is_possible(&eq, *sum, &vec![add, multiply, concat]))
        .map(|(sum, _)| sum)
        .sum();


    let elapsed = now.elapsed();

    println!("Optimized:");
    println!("Part 1: {}", part1);
    println!("Part 2: {}", part2);
    println!("Elapsed: {:.2?}", elapsed);
}
