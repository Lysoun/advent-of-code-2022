use std::cmp::max;
use std::fs;

fn main() {
    let input_path = "src/day1/input.txt";
    let input = match fs::read_to_string(input_path) {
        Ok(file) => file,
        Err(error) => panic!("Problem opening the file: {:?}", error)
    };
    let mut max_calories: u32 = 0;
    let mut elf_calories_number: u32 = 0;
    for line in input.lines() {
        if line == "" {
            max_calories = max(max_calories, elf_calories_number);
            elf_calories_number = 0;
        } else {
            let line_calories_number: u32 = match line.parse() {
                Ok(calories_number) => calories_number,
                Err(error) =>  panic!("Problem reading line: {:?}", error)
            };
            elf_calories_number += line_calories_number;
        }
    }
    max_calories = max(max_calories, elf_calories_number);

    println!("max calories: {}", max_calories)
}
