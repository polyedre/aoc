use std::io;

fn main() {
    let mut total = 0;

    for line in io::stdin().lines() {
        let line = line.unwrap();

        let mut all_numbers = line.split(':').last().unwrap().split('|');

        let mut winning_numbers: Vec<u32> = Vec::new();
        for number in all_numbers.next().unwrap().trim().split_whitespace() {
            winning_numbers.push(number.to_string().parse().unwrap())
        }

        let mut our_numbers: Vec<u32> = Vec::new();
        for number in all_numbers.next().unwrap().trim().split_whitespace() {
            our_numbers.push(number.to_string().parse().unwrap())
        }

        let number_of_win = our_numbers.iter().filter(|num| winning_numbers.contains(num)).count() as u32;

        println!("Total number of win: {}", number_of_win);

        if number_of_win > 0 {
            total += 2_u32.pow(number_of_win - 1);
        }
    }

    println!("Total part 1: {}", total);
}
