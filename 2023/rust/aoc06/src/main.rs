use std::io;

fn main() {
    let mut all_lines = io::stdin().lines();

    let times: Vec<u32> = all_lines.next().unwrap().unwrap().split(':').last().unwrap().split_whitespace().map(|num| num.parse().unwrap()).collect();
    let distance: Vec<u32> = all_lines.next().unwrap().unwrap().split(':').last().unwrap().split_whitespace().map(|num| num.parse().unwrap()).collect();

    let mut part_1_solution = 1;
    for (time, last_record_distance) in std::iter::zip(times.clone(), distance.clone()) {
        let mut number_of_ways_to_win = 0;
        for hold_duration in 0..time {
            if ((time - hold_duration) * hold_duration) > last_record_distance {
                number_of_ways_to_win += 1;
            }
        }

        part_1_solution *= number_of_ways_to_win;
    }

    println!("Part 1 solution: {}", part_1_solution);

    // Part 2

    let time: u64 = times.iter().fold("".to_string(), |a, b| format!("{}{}", a, b)).parse().unwrap();
    let last_record_distance = distance.iter().fold("".to_string(), |a, b| format!("{}{}", a, b));
    let last_record_distance: u64 = last_record_distance.parse().unwrap();

    let mut number_of_ways_to_win = 0;
    for hold_duration in 0..time {
        if ((time - hold_duration) * hold_duration) > last_record_distance {
            number_of_ways_to_win += 1;
        }
    }

    println!("Part 2 solution: {}", number_of_ways_to_win);
}
