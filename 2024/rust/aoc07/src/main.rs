use std::io::BufReader;
use std::io::prelude::*;
use std::{fs::File, time::Instant};

fn is_possible(equation: &[u64], sum: u64, operations: &[fn(u64, u64) -> Option<u64>]) -> bool {
    match equation {
        [] => panic!("Should not happen"),
        [elem] => sum == *elem,
        [remainder @ .., last] => {
            operations.iter().any(|op| {
                match op(sum, *last) {
                    Some(sum) => is_possible(remainder, sum, operations),
                    None => false,
                }
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

    let subs: fn(u64, u64) -> Option<u64> = |a, b| if a > b { Some(a - b) } else { None };
    let divide: fn(u64, u64) -> Option<u64> = |a, b| if a % b == 0 { Some(a / b) } else { None };
    let unconcat: fn(u64, u64) -> Option<u64> = |a, b| {
        let mask = 10u64.pow(b.ilog10() + 1);
        if a % mask == b { Some(a / mask) } else { None }
    };

    let operations: &[fn(u64, u64) -> Option<u64>] = &[subs, divide, unconcat];

    let now = Instant::now();
    let part1: u64 = input.iter()
        .filter_map(|(sum, eq)| is_possible(&eq, *sum, &operations[..2]).then_some(sum))
        .sum();

    let part2: u64 = input.iter()
        .filter_map(|(sum, eq)| is_possible(&eq, *sum, operations).then_some(sum))
        .sum();
    let elapsed = now.elapsed();

    println!("Part 1: {}", part1);
    println!("Part 2: {}", part2);
    println!("Elapsed: {:.2?}", elapsed);
}
