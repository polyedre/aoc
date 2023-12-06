use std::io;

fn main() {
    let mut all_lines = io::stdin().lines();

    let times: Vec<u32> = all_lines.next().unwrap().unwrap().split(':').last().unwrap().split_whitespace().map(|num| num.parse().unwrap()).collect();
    let distance: Vec<u32> = all_lines.next().unwrap().unwrap().split(':').last().unwrap().split_whitespace().map(|num| num.parse().unwrap()).collect();

    let mut part_1_solution = 1;
    for (time, last_record_distance) in std::iter::zip(times, distance) {
        let mut number_of_ways_to_win = 0;
        for hold_duration in 0..time {
            if ((time - hold_duration) * hold_duration) > last_record_distance {
                number_of_ways_to_win += 1;
            }
        }

        part_1_solution *= number_of_ways_to_win;
    }

    println!("Part 1 solution: {}", part_1_solution);
}
