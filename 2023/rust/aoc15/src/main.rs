fn hash(input: &str) -> u8 {
    let mut result: usize = 0;
    for char in input.chars() {
        result += char as usize;
        result *= 17;
        result %= 256;
    }

    result.try_into().unwrap()
}

fn main() {
    let result: usize = std::io::stdin()
        .lines().next().unwrap().unwrap()
                       .split(",").map(|command| hash(command) as usize).sum();
    println!("Part 1: {result}");
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
