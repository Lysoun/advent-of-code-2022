use std::fs;

fn convert_my_play_to_u32(play: char) -> u32 {
    return match play {
        'X' => 1,
        'Y' => 2,
        _ => 3
    }
}

fn convert_opponent_play_to_u32(play: char) -> u32 {
    return match play {
        'A' => 1,
        'B' => 2,
        _ => 3
    }
}

fn compute_score(my_play:char, opponent_play: char) -> u32 {
    let my_play_u32 = convert_my_play_to_u32(my_play);
    let opponent_play_u32 = convert_opponent_play_to_u32(opponent_play);
    let match_outcome: u32 = if (my_play_u32 + 1) % 3 + 1 == opponent_play_u32 {
        6
    } else {
        if my_play_u32 == opponent_play_u32 {
            3
        } else {
            0
        }
    };
    return match_outcome + my_play_u32
}

fn main() {
    let input_path = "input.txt";
    let input = match fs::read_to_string(input_path) {
        Ok(file) => file,
        Err(error) => panic!("Problem opening the file: {:?}", error)
    };
    let mut total_score: u32 = 0;
    for line in input.lines() {
        let mut chars = line.chars();
        let opponent_play = chars.next().unwrap();
        chars.next();
        let my_play = chars.next().unwrap();
        let line_score = compute_score(my_play, opponent_play);
        total_score += line_score
    }
    println!("Total score: {}", total_score);
}

