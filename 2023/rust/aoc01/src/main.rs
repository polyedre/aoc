use std::io;
use regex::Regex;

const DIGITS_ASSOCIATION: [(&str, &str); 9] = [
    ("one", "1"),
    ("two", "2"),
    ("three", "3"),
    ("four", "4"),
    ("five", "5"),
    ("six", "6"),
    ("seven", "7"),
    ("eight", "8"),
    ("nine", "9")
];

fn match_to_digit(digit: &str) -> u32 {
    match digit {
        "one" => 1,
        "two" => 2,
        "three" => 3,
        "four" => 4,
        "five" => 5,
        "six" => 6,
        "seven" => 7,
        "eight" => 8,
        "nine" => 9,
        "1" => 1,
        "2" => 2,
        "3" => 3,
        "4" => 4,
        "5" => 5,
        "6" => 6,
        "7" => 7,
        "8" => 8,
        "9" => 9,
        // ...
        _ => 0
    }
}

fn main() {
    let mut total = 0;

    let first_regex = Regex::new(r"(one|two|three|four|five|six|seven|eight|nine|\d)").expect("failed to compile regex");
    let last_regex = Regex::new(r"(?:.*)(one|two|three|four|five|six|seven|eight|nine|\d)").expect("failed to compile regex");

    for line in io::stdin().lines() {
        let line = line.unwrap();
        let first = first_regex.find(line.as_str()).map(|x| x.as_str()).unwrap();
        let first_digit: u32 = match_to_digit(first);
        let last = last_regex.captures(line.as_str()).unwrap().get(1).unwrap().as_str();
        let last_digit: u32 = match_to_digit(last);
        total += 10*first_digit + last_digit;

        // let mut string_line = line.unwrap();
        // let mut result: String = String::new();

        // 'outer: while !string_line.is_empty() {
        //     println!("string_line: {}, result: {}", string_line, result);
        //     for (spelled_digit, digit) in DIGITS_ASSOCIATION {
        //         if string_line.starts_with(spelled_digit) {
        //             result = format!("{}{}", result, digit);
        //             string_line = string_line[spelled_digit.len()..].to_string();
        //             continue 'outer
        //         }
        //     }
        //     result = format!("{}{}", result, string_line.chars().next().unwrap().to_string());
        //     string_line = string_line[1..].to_string();
        //     println!("Just removed one char: string_line: {}, result: {}", string_line, result);

        // }

        // println!("string_line: {}, result: {}", string_line, result);

        // let digits: Vec<char> = result.chars().filter(|x| x.is_numeric()).collect();
        // let coord: u32 = format!("{}{}", digits.first().unwrap(), digits.last().unwrap()).parse().unwrap();
        // println!("Number is {}", coord);
        // total += coord;
    }

    println!("{}", total)
}
