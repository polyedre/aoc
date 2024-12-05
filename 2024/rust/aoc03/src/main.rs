use std::{io::{self, Read}, process::exit};
use regex::Regex;

fn run_instructions(instructions: &str) -> i32 {
    let mul_re = Regex::new(r"mul\((\d{1,3}),(\d{1,3})\)").unwrap();

    mul_re.captures_iter(instructions)
          .map(|c| c.extract())
          .map(|(_, [first, second])|
               first.parse::<i32>().unwrap() * second.parse::<i32>().unwrap()).sum::<i32>()

}

fn main() {
    let mut input = String::new();
    if let Err(err) = io::stdin().read_to_string(&mut input) {
        eprintln!("Failure to read the input: {err}");
        exit(1)
    }

    println!("part 1: {}", run_instructions(input.as_str()));

    let part2: i32 = input.split("do()").map(|s| s.split("don't()").next().unwrap()).map(|s| run_instructions(s)).sum();
    println!("part 2: {}", part2);
}
