use std::io::BufRead;
use crate::configuration::Configuration;

pub fn run(config: Configuration) -> Result<String, String> {
    if config.input_file_buffer.is_some() {
        let mut total_points: i32 = 0;

        for line in config.input_file_buffer.unwrap().lines() {
            if let Ok((left, right)) = split_round(line.unwrap()) {
                //total_points += play_pt1(left, right);
                total_points += play_pt2(left, right);
            }
        }

        Ok(total_points.to_string())
    } else {
        panic!("A file was not provided");
    }
}

fn split_round(str: String) -> Result<(char, char), String> {
    let items: Vec<&str> = str.split_whitespace().collect::<Vec<_>>();

    if items.len() == 2 {
        let left = items.get(0);
        let right = items.get(1);

        if left.is_some() && right.is_some() {
            let c_left: char = left.unwrap().to_string().chars().nth(0) .unwrap();
            let c_right: char = right.unwrap().to_string().chars().nth(0).unwrap();


            return Ok((c_left, c_right));
        };
    }


    let error: String = format!("Could not split string: {}", str);
    Err(error)
}

// PART 1
fn play_pt1(oponent: char, player: char) -> i32 {
    //let mut round_points: i32 = 0;
    let defeater: char = get_defeater(&oponent);

    let outcome = if player == defeater { 6 } else if player == get_equivalent(&oponent) { 3 } else { 0 };

    outcome + get_move_points(&player)
}

/* PART 2 
outcome: X = Lose, Y = Draw, Z = Win
*/
fn play_pt2(oponent: char, outcome: char) -> i32 {
    let points: i32 = match outcome {
        'X' => get_move_points(&get_loser(&oponent)),
        'Y' => 3 + get_move_points(&oponent),
        'Z' => 6 + get_move_points(&get_defeater(&oponent)),
        _ => panic!("Uknown parameter"),
    };

    points
}

fn get_move_points(play: &char) -> i32 {
    match play {
       'A' => 1,
       'B' => 2,
       'C' => 3,
       'X' => 1,
       'Y' => 2,
       'Z' => 3,
       _ => panic!("Uknown parameter"),
    }
}

fn get_defeater(play: &char) -> char {
    match play {
        'A' => 'Y',
        'B' => 'Z',
        'C' => 'X',
        _ => panic!("Uknown parameter"),
    }
}

fn get_loser(play: &char) -> char  {
    match play {
        'A' => 'Z',
        'B' => 'X',
        'C' => 'Y',
        _ => panic!("Uknown parameter"),
    }
}

fn get_equivalent(play: &char) -> char {
    match play {
        'A' => 'X',
        'B' => 'Y',
        'C' => 'Z',
        _ => panic!("Uknown parameter"),
    }

}
