use std::collections::HashSet;
use std::fs;

fn is_valid_marker(sequence: &str) -> bool {
    let mut characters_set: HashSet<char> = HashSet::new();
    let chars = sequence.chars();
    for c in chars {
        if !characters_set.insert(c) {
            return false;
        }
    }

    return true;
}

fn main() {
    let input_path = "input.txt";
    let input = match fs::read_to_string(input_path) {
        Ok(file) => file,
        Err(error) => panic!("Problem opening the file: {:?}", error)
    };

    let mut position: usize = 0;
    let mut found = false;

    while !found {
        if !is_valid_marker(input.get(position..(position + 4)).unwrap()) {
            position += 1;
        } else {
            found = true;
        }
    }

    println!("Result: {}", position + 4);
}
