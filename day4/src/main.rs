use std::fs;

#[derive(Clone, Copy)]
struct Assignment {
    section_range_beginning: u8,
    section_range_end: u8
}

fn is_fully_included_in(assignment1: Assignment, assignment2: Assignment) -> bool {
    return assignment1.section_range_beginning >= assignment2.section_range_beginning &&
        assignment1.section_range_end <= assignment2.section_range_end;
}

fn main() {
    let input_path = "input.txt";
    let input = match fs::read_to_string(input_path) {
        Ok(file) => file,
        Err(error) => panic!("Problem opening the file: {:?}", error)
    };
    let mut answer: u16 = 0;
    for line in input.lines() {
        let assignments = line
            .split(',')
            .flat_map(|section| section.split('-').collect::<Vec<&str>>())
            .map(|section| {
                section
                    .parse::<u8>()
                    .unwrap_or_else(|_| panic!("Could not parse '{}' as int", section))
            }).collect::<Vec<u8>>();
        let first_assignment = Assignment {
            section_range_beginning: assignments[0],
            section_range_end: assignments[1]
        };
        let second_assignment = Assignment {
            section_range_beginning: assignments[2],
            section_range_end: assignments[3]
        };
        if is_fully_included_in(first_assignment, second_assignment) ||
            is_fully_included_in(second_assignment, first_assignment) {
            answer += 1;
        }
    }
    println!("Number of pairs in which one assignment fully contains the one: {}", answer);
}
