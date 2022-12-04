use std::fs;

fn item_type_to_priority(item_type: &char) -> u32 {
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
        _ => item_type_to_priority(&item_type.to_ascii_lowercase()) + 26
    };
}

fn main() {
    let input_path = "input.txt";
    let input = match fs::read_to_string(input_path) {
        Ok(file) => file,
        Err(error) => panic!("Problem opening the file: {:?}", error)
    };
    let mut total_priority: u32 = 0;
    let mut group_counter: u8 = 0;
    let mut group_badges_possibilities: Vec<char> = vec![];
    for line in input.lines() {
        group_counter += 1;

        if group_counter == 1 {
            group_badges_possibilities = vec![];
            for c in line.chars() {
                group_badges_possibilities.push(c);
            }
        } else {
            for i in (0..group_badges_possibilities.len()).rev() {
                if !line.contains(&group_badges_possibilities.get(i).unwrap().to_string()) {
                    group_badges_possibilities.remove(i);
                }
            }

            if group_counter == 3 {
                group_counter = 0;
                total_priority += item_type_to_priority(group_badges_possibilities.get(0).unwrap());
            }
        }
    }
    println!("Total priority: {}", total_priority);
}