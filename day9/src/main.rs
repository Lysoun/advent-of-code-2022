use std::cmp::max;
use std::collections::HashSet;
use std::fs;

fn safe_file_read(path: &str) -> String {
    return match fs::read_to_string(path) {
        Ok(file) => file,
        Err(error) => panic!("Problem opening the file: {:?}", error)
    };
}

fn compute_new_position(direction: char, position: (i16, i16)) -> (i16, i16) {
    return match direction {
        'U' => (position.0 + 1, position.1),
        'D' => (position.0 - 1, position.1),
        'L' => (position.0, position.1 - 1),
        _ => (position.0, position.1 + 1)
    };
}

fn compute_distance(pos1: (i16, i16), pos2: (i16, i16)) -> i16 {
    return max(
        (pos1.1 - pos2.1).abs(),
        (pos1.0 - pos2.0).abs()
    )
}

fn compute_new_tail_position(head_position: (i16, i16), tail_position: (i16, i16)) -> (i16, i16) {
    if head_position.0 == tail_position.0 {
        if head_position.1 > tail_position.1 {
            return (head_position.0, head_position.1 - 1);
        }
        return  (head_position.0, head_position.1 + 1);
    }

    if head_position.1 == tail_position.1 {
        if head_position.0 > tail_position.0 {
            return (head_position.0 - 1, head_position.1);
        }
        return (head_position.0 + 1, head_position.1);
    }

    let mut movement: (i16, i16) = (0, 0);

    if head_position.0 > tail_position.0 {
        movement.0 = 1;
    } else {
        movement.0 = -1;
    }

    if head_position.1 > tail_position.1 {
        movement.1 = 1;
    } else {
        movement.1 = -1;
    }

    return (tail_position.0 + movement.0, tail_position.1 + movement.1);
}

fn main() {
    let input = safe_file_read("input.txt");
    let input_lines = input.lines();
    let knots_number: usize = 10;
    let mut rope_knots_positions: Vec<(i16, i16)> = vec![];

    for _i in 0..knots_number {
        rope_knots_positions.push((0, 0));
    }

    let mut visited_positions: HashSet<(i16, i16)> = HashSet::new();
    visited_positions.insert((0, 0));

    for line in input_lines {
        let line_split: Vec<&str> = line.split_whitespace().collect();
        let direction = line_split[0].chars().next().unwrap();
        let steps_number: i16 =  match line_split[1].parse() {
            Ok(n) => n,
            Err(error) =>  panic!("Problem reading line: {:?}", error)
        };

        for _i in 0..steps_number {
            rope_knots_positions[0] = compute_new_position(direction, rope_knots_positions[0]);

            for j in 1..knots_number {
                if compute_distance(rope_knots_positions[j - 1], rope_knots_positions[j]) > 1 {
                    rope_knots_positions[j] = compute_new_tail_position(rope_knots_positions[j - 1], rope_knots_positions[j]);
                }
            }
            visited_positions.insert(rope_knots_positions[knots_number - 1]);
        }
    }

    println!("Number of visited positions by the rope tail: {}", visited_positions.len());
}
