use std::fs;

fn convert_opponent_play_to_u32(play: char) -> u32 {
    return match play {
        'A' => 1,
        'B' => 2,
        _ => 3
    }
}

fn compute_score(round_result: char, opponent_play: char) -> u32 {
    let opponent_play_u32 = convert_opponent_play_to_u32(opponent_play);
    println!("round result: {}", round_result);
    return match round_result {
        'X' => match opponent_play_u32 {
            1 => 3,
            2 => 1,
            _3 => 2
        }
        'Y' => 3 + opponent_play_u32,
        _ => 6 + match opponent_play_u32 {
            1 => 2,
            2 => 3,
            _ => 1
        }
    };
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
        let round_result = chars.next().unwrap();
        let line_score = compute_score(round_result, opponent_play);
        println!("Line score: {}", line_score);
        total_score += line_score
    }
    println!("Total score: {}", total_score);
}

