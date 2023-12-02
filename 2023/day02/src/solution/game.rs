use std::{io, str::FromStr};

use super::round::Round;

pub struct Game {
    rounds: Vec<Round>,
}

impl Game {
    fn new() -> Self {
        Self { rounds: vec![] }
    }

    pub fn is_game_possible(&self) -> bool {
        for round in self.rounds.iter() {
            if !round.is_round_possible() {
                return false;
            }
        }

        true
    }

    pub fn find_min_power(&self) -> usize {
        let mut min_red = 0usize;
        let mut min_green = 0usize;
        let mut min_blue = 0usize;

        self.rounds.iter().for_each(|round| {
            if let Some(amount) = round.red {
                if amount > min_red {
                    min_red = amount;
                }
            }
            if let Some(amount) = round.green {
                if amount > min_green {
                    min_green = amount;
                }
            }
            if let Some(amount) = round.blue {
                if amount > min_blue {
                    min_blue = amount;
                }
            }
        });

        min_red * min_green * min_blue
    }
}

impl FromStr for Game {
    type Err = io::Error;

    fn from_str(game_str: &str) -> Result<Self, Self::Err> {
        let mut game = Game::new();

        game_str
            .split(";")
            .for_each(|round_str| game.rounds.push(Round::from_str(round_str).unwrap()));

        Ok(game)
    }
}
