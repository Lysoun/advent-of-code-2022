use std::fs;

fn item_type_to_priority(item_type: char) -> u32 {
    return match item_type {
        'a' => 1,
        'b' => 2,
        'c' => 3,
        'd' => 4,
        'e' => 5,
        'f' => 6,
        'g' => 7,
        'h' => 8,
        'i' => 9,
        'j' => 10,
        'k' => 11,
        'l' => 12,
        'm' => 13,
        'n' => 14,
        'o' => 15,
        'p' => 16,
        'q' => 17,
        'r' => 18,
        's' => 19,
        't' => 20,
        'u' => 21,
        'v' => 22,
        'w' => 23,
        'x' => 24,
        'y' => 25,
        'z' => 26,
        _ => item_type_to_priority(item_type.to_ascii_lowercase()) + 26
    };
}

fn find_item_type_in_both_parts(first_part: &str, second_part: &str) -> char {
    for c in first_part.chars() {
        if second_part.contains(c) {
            return c;
        }
    }

    return 'a';
}

fn main() {
    let input_path = "input.txt";
    let input = match fs::read_to_string(input_path) {
        Ok(file) => file,
        Err(error) => panic!("Problem opening the file: {:?}", error)
    };
    let mut total_priority: u32 = 0;
    for line in input.lines() {
        let middle_index = line.len() / 2;
        let first_part = line.get(0..middle_index);
        let second_part = line.get(middle_index..line.len());
        let item_type_in_both_parts = find_item_type_in_both_parts(
            first_part.unwrap(),
            second_part.unwrap(),
        );
        total_priority += item_type_to_priority(item_type_in_both_parts);
    }
    println!("Total priority: {}", total_priority);
}