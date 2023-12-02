use std::{io, str::FromStr};

use super::{BLUE_CUBES_AMOUNT, GREEN_CUBES_AMOUNT, RED_CUBES_AMOUNT};

pub struct Round {
    pub red: Option<usize>,
    pub green: Option<usize>,
    pub blue: Option<usize>,
}

impl Round {
    pub fn new() -> Self {
        Round {
            red: None,
            green: None,
            blue: None,
        }
    }

    pub fn is_round_possible(&self) -> bool {
        if let Some(red_amount) = self.red {
            if red_amount > RED_CUBES_AMOUNT {
                return false;
            }
        }
        if let Some(green_amount) = self.green {
            if green_amount > GREEN_CUBES_AMOUNT {
                return false;
            }
        }
        if let Some(blue_amount) = self.blue {
            if blue_amount > BLUE_CUBES_AMOUNT {
                return false;
            }
        }

        true
    }
}

impl FromStr for Round {
    type Err = io::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut round = Round::new();

        for item in s.replace(" ", "").split(",") {
            // Try red
            if let Some(index) = item.find("red") {
                if let Ok(amount) = item[..=index - 1].parse::<usize>() {
                    round.red = Some(amount);
                }
            }

            // Try green
            if let Some(index) = item.find("green") {
                if let Ok(amount) = item[..=index - 1].parse::<usize>() {
                    round.green = Some(amount);
                }
            }

            // Try blue
            if let Some(index) = item.find("blue") {
                if let Ok(amount) = item[..=index - 1].parse::<usize>() {
                    round.blue = Some(amount);
                }
            }
        }

        Ok(round)
    }
}
