use std::collections::HashMap;
use std::fs;

fn display_stack(stack: &Vec<char>) {
    stack.iter().for_each(|c| print!("{}", c));
    println!();
}

fn display_stacks(stacks: &HashMap<u8, Vec<char>>) {
    stacks.values().for_each(|stack| display_stack(stack));
    println!("End of stacks display");
}

fn main() {
    let mut stacks: HashMap<u8, Vec<char>> = HashMap::new();
    let mut stacks_parsed = false;

    let input_path = "input.txt";
    let input = match fs::read_to_string(input_path) {
        Ok(file) => file,
        Err(error) => panic!("Problem opening the file: {:?}", error)
    };
    let max_stack: u8 = 9;

    for i in 1..(max_stack + 1) {
        stacks.insert(i, vec![]);
    }

    for line in input.lines() {
        if stacks_parsed {
            let line_split = line.split_whitespace()
                .collect::<Vec<&str>>();
            let number_to_move = line_split[1].parse::<u8>().unwrap_or_else(|_| panic!("Could not parse as int"));
            let from_stack = line_split[3].parse::<u8>().unwrap_or_else(|_| panic!("Could not parse as int"));
            let to_stack = line_split[5].parse::<u8>().unwrap_or_else(|_| panic!("Could not parse as int"));
            let mut crates_to_move: Vec<char> = vec![];
            for _i in 0..number_to_move {
                let crate_moved = stacks.get_mut(&from_stack).unwrap().pop();
                crate_moved.map(|c| crates_to_move.push(c));
            }
            crates_to_move.reverse();

            for crate_to_move in crates_to_move {
                stacks.get_mut(&to_stack).unwrap().push(crate_to_move);
            }

            } else {
            if line.len() == 0 {
                for i in 1..(max_stack + 1) {
                    stacks.get_mut(&i).unwrap().reverse();
                }
                stacks_parsed = true;
                continue;
            } else {
                for i in 0..(max_stack + 1) {
                    let index = 1 + (4 * i) as usize;
                    line.chars().nth(index)
                        .filter(|c| c.is_alphabetic())
                        .map(|value| stacks.get_mut(&(i + 1)).unwrap().push(value));
                }
            }
        }
    }

    for i in 1..(max_stack + 1) {
        let stack = stacks.get(&i).unwrap();
        print!("{}", stack[stack.len() - 1]);
    }
}

