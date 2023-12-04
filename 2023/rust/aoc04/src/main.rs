use std::io;

#[derive(Clone, Debug)]
struct Scratchcard {
    number_of_copies: u32,
    number_of_wins: u32
}

fn main() {
    let mut total = 0;
    let mut total_part_2 = 0;

    let mut scratchcards: Vec<Scratchcard> = Vec::new();

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

        scratchcards.push(Scratchcard { number_of_copies: 1, number_of_wins: number_of_win });

        if number_of_win > 0 {
            total += 2_u32.pow(number_of_win - 1);
        }
    }

    for current_id in 0..scratchcards.len() {

        let scratchcard = scratchcards[current_id].clone();

        for offset in 1..=(scratchcard.number_of_wins as usize) {
            scratchcards[current_id + offset].number_of_copies += scratchcard.number_of_copies;
        }

        println!("Card {} has {} instances", current_id + 1, scratchcard.number_of_copies);

        total_part_2 += scratchcard.number_of_copies;
    }

    println!("Total part 1: {}", total);
    println!("Total part 2: {}", total_part_2);
}
