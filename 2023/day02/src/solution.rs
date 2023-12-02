use crate::configuration::Configuration;
use std::{
    io::{self, BufRead, ErrorKind},
    str::FromStr,
};

use self::game::Game;

mod game;
mod round;

const RED_CUBES_AMOUNT: usize = 12;
const GREEN_CUBES_AMOUNT: usize = 13;
const BLUE_CUBES_AMOUNT: usize = 14;

pub fn run(config: Configuration) -> io::Result<String> {
    if let Some(input_buf) = config.input_file_buffer {
        let mut sum = 0usize;
        let mut powers = 0usize;

        for (index, line) in input_buf.lines().enumerate() {
            if let Ok(s) = line {
                let game = Game::from_str(get_game_str(&s))?;

                if game.is_game_possible() {
                    // use index instead of parsing game number
                    sum += index + 1
                }

                powers += game.find_min_power();
            }
        }
        Ok(format!("Part 1: {}, Part 2: {}", sum, powers))
    } else {
        Err(io::Error::new(ErrorKind::Other, "Input file required"))
    }
}

fn get_game_str(line: &'_ str) -> &'_ str {
    let game_split_index = line.find(":").expect("Expected colon");

    &line[game_split_index + 2..]
}
