fn hash(input: &str) -> u8 {
    let mut result: usize = 0;
    for char in input.chars() {
        result += char as usize;
        result *= 17;
        result %= 256;
    }

    result.try_into().unwrap()
}

fn display_boxes(boxes: &Vec<Vec<(&str, u8)>>) {
    for (id, single_box) in boxes.iter().enumerate() {
        if !single_box.is_empty() {
            println!(
                "Box {id}: [{}]",
                single_box.iter().map(|(key, value)| format!("{key} {value}")).collect::<String>()
            )
        }
    }
}

fn main() {
    let mut result_part_1: usize = 0;

    let mut boxes: Vec<Vec<(&str, u8)>> = Vec::new();

    for _ in 0..256 {
        boxes.push(Vec::new());
    }

    let line = std::io::stdin().lines().next().unwrap().unwrap();

    for step in line.split(",") {
        result_part_1 += hash(step) as usize;

        if step.contains("=") {
            let (label, focal_length) = step.split_once("=").unwrap();
            let current_box = &mut boxes[hash(label) as usize];

            let mut modified = false;
            for (index, (len_label, _)) in current_box.clone().iter().enumerate() {
                if *len_label == label {
                    current_box[index] = (label, focal_length.parse().unwrap());
                    modified = true;
                }
            }

            if !modified {
                current_box.push((label, focal_length.parse().unwrap()))
            }
        } else if step.contains("-") {

            let label = &step[..step.find("-").unwrap()];
            boxes[hash(label) as usize].retain(|(len_label, _)| *len_label != label);
        }

        println!("After \"{}\"", step);
        display_boxes(&boxes);
        println!("");
    }

    println!("Part 1: {result_part_1}");

    let mut result_part_2: usize = 0;

    for (box_id, single_box) in boxes.iter().enumerate() {
        for (len_id, (_, focal_length)) in single_box.iter().enumerate() {
            result_part_2 += (1 + box_id) * (len_id + 1) * *focal_length as usize;
        }
    }

    println!("Part 2: {result_part_2}");
}

#[test]
fn test_hash() {
    let test_pairs = [
        ("rn", 0),
        ("HASH", 52),
        ("rn=1", 30),
        ("cm-", 253),
        ("qp=3", 97)
    ];

    for (input, hashed_input) in test_pairs {
        assert_eq!(hash(input), hashed_input);
    }
}
