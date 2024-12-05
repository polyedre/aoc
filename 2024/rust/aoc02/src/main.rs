use std::io;

fn is_safe(report: &Vec<u32>) -> bool {
    let differences: Vec<i32> = report.windows(2).map(|w| w[0] as i32 - w[1] as i32).collect();
    return differences.iter().all(|&x| 4 > x && x > 0) || differences.iter().all(|&x| 0 > x && x > -4)
}

fn is_safe_enough(report: &Vec<u32>) -> bool {
    (0..report.len()).any(
        |i| is_safe([&report[0..i], &report[i+1..]].concat().as_ref())
    )
}

fn main() {
    let data: Vec<Vec<u32>> = io::stdin().lines()
        .map(|line| {
            line.unwrap().split_whitespace().map(|s| s.parse::<u32>().unwrap()).collect()
        }).collect();

    let part1 = data.iter().filter(|report| is_safe(&report)).count();
    println!("part 1: {:?}", part1);

    let part2 = data.iter().filter(|report| is_safe_enough(&report)).count();
    println!("part 2: {:?}", part2);
}
