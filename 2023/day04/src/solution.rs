mod card;

use crate::configuration::Configuration;
use std::io::{self, BufRead, ErrorKind};

use self::card::Card;

pub fn run(config: Configuration) -> io::Result<String> {
    if let Some(input_buf) = config.input_file_buffer {
        let mut cards: Vec<Card> = vec![];

        for line in input_buf.lines() {
            if let Ok(s) = line {
                cards.push(Card::from_str(s)?);
            }
        }

        let mut part1 = 0usize;
        for card in &cards {
            part1 += card.count_points();
        }

        let part2 = count_recursively(&cards, 0, cards.len() - 1);

        Ok(format!("part1: {}, part2: {}", part1, part2))
    } else {
        Err(io::Error::new(ErrorKind::Other, "Input file required"))
    }
}

pub fn count_recursively(cards: &Vec<Card>, l_range_limit: usize, h_range_limit: usize) -> usize {
    let mut sum = 0usize;

    for (i, card) in cards[l_range_limit..=h_range_limit].iter().enumerate() {
        sum += 1;

        let matches = card.count_matches();

        if matches != 0 {
            let new_l_range_limit = if l_range_limit + i > cards.len() - 1 {
                cards.len() - 1
            } else {
                l_range_limit + i + 1
            };
            let new_h_range_limit = if l_range_limit + i + matches > cards.len() - 1 {
                cards.len() - 1
            } else {
                l_range_limit + i + matches
            };
            sum += count_recursively(cards, new_l_range_limit, new_h_range_limit);
        }
    }

    sum
}
