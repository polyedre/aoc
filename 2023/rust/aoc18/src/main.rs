use core::panic;

fn shoelace_formula(points: &Vec<(i64, i64)>) -> i64 {
    let mut sum = 0;
    let mut points = points.clone();
    points.push(points[0]);

    for i in 0..(points.len() - 1) {
        sum += (points[i + 1].0 - points[i].0) * (points[i + 1].1 + points[i].1);
    }

    i64::abs(sum / 2)
}

fn main() {
    let mut part_1_instructions: Vec<(char, usize)> = Vec::new();
    let mut part_2_instructions = Vec::new();

    for line in std::io::stdin().lines() {
        let line = line.unwrap();

        let mut splited_line = line.split_whitespace();
        let direction: char = splited_line.next().unwrap().parse().unwrap();
        let length: usize = splited_line.next().unwrap().parse().unwrap();
        let hexa = String::from(&splited_line.next().unwrap()[2..8]);

        part_1_instructions.push((direction, length));
        part_2_instructions.push(hexa);
    }

    let mut points: Vec<(i64, i64)> = Vec::new();

    let mut x = 0;
    let mut y = 0;

    for (direction, length) in &part_1_instructions {
        points.push((x, y));

        match direction {
            'R' => x += (*length) as i64,
            'L' => x -= (*length) as i64,
            'U' => y -= (*length) as i64,
            'D' => y += (*length) as i64,
            _ => panic!("Unknown direction!")
        }
    }

    let inside_area = shoelace_formula(&points);
    let perimeter: i64 = part_1_instructions.iter().map(|(_, lenght)| *lenght as i64).sum();

    println!("Part 1 area: {}", inside_area + perimeter / 2 + 1);

    // --- Part 2

    let mut points: Vec<(i64, i64)> = Vec::new();

    let mut x = 0;
    let mut y = 0;

    let mut perimeter: i64 = 0;

    for hexa_value in part_2_instructions {
        let length = i64::from_str_radix(&hexa_value[0..5], 16).unwrap();
        points.push((x, y));

        match hexa_value.chars().nth(5).unwrap() {
            '0' => x += length,
            '1' => y += length,
            '2' => x -= length,
            '3' => y -= length,
            _ => panic!("Unknown direction!")
        }

        perimeter += length;
    }

    let inside_area = shoelace_formula(&points);

    println!("Part 1 area: {}", inside_area + perimeter / 2 + 1);
}
