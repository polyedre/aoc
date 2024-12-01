use std::io;
use std::iter::zip;

fn main() {
    let mut left_col = Vec::new();
    let mut right_col = Vec::new();

    // Parsing the input

    for line in io::stdin().lines() {
        let line = line.unwrap();
        let mut line_elements = line.split("   ");

        let left = line_elements.next().expect("Failed to get next element").parse::<i32>().unwrap();
        let right = line_elements.next().expect("Failed to get next element").parse::<i32>().unwrap();

        left_col.push(left);
        right_col.push(right);
    }

    // Part One

    left_col.sort();
    right_col.sort();

    let result1: i32 = zip(left_col.iter(), right_col.iter()).map(|(&l, &r)| (r - l).abs()).sum();

    println!("Part One: {}", result1);

    let result2: i32 = left_col.iter().map(|&elem| elem * right_col.iter().filter(|&n| *n == elem).count() as i32).sum();

    println!("Part Two: {}", result2);
}
