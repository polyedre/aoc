use std::io;

fn get_next_number(numbers: &Vec<i32>) -> i32 {
    if numbers.iter().sum::<i32>() == 0 { return 0 }

    let mut differences: Vec<i32> = Vec::new();

    for index in 0..(numbers.len() - 1) {
        differences.push(numbers[index + 1] - numbers[index]);
    }

    let new_difference = get_next_number(&differences);
    return numbers.last().unwrap() + new_difference;
}


fn get_previous_number(numbers: &Vec<i32>) -> i32 {
    if numbers.iter().sum::<i32>() == 0 { return 0 }

    let mut differences: Vec<i32> = Vec::new();

    for index in 0..(numbers.len() - 1) {
        differences.push(numbers[index + 1] - numbers[index]);
    }

    let new_difference = get_previous_number(&differences);
    return numbers.first().unwrap() - new_difference;
}

fn main() {
    let mut total_part_1 = 0;
    let mut total_part_2 = 0;

    for line in io::stdin().lines() {
        let line = line.unwrap();

        let numbers: Vec<i32> = line.split_whitespace().map(|num| num.parse().unwrap()).collect();

        let next_number = get_next_number(&numbers);
        let previous_number = get_previous_number(&numbers);

        println!("Sequence {previous_number} {numbers:?} {next_number}");

        total_part_1 += next_number;
        total_part_2 += previous_number;
    }

    println!("Part 1 solution is {total_part_1}");
    println!("Part 2 solution is {total_part_2}");
}
