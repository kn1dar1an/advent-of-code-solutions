use once_cell::sync::Lazy;

use crate::configuration::Configuration;
use std::{
    cmp::Ordering,
    collections::HashMap,
    io::{self, BufRead, ErrorKind},
    str::FromStr,
};

static CARD_VALUES: Lazy<HashMap<char, usize>> = Lazy::new(|| {
    HashMap::from([
        ('A', 13),
        ('K', 12),
        ('Q', 11),
        ('J', 10),
        ('T', 9),
        ('9', 8),
        ('8', 7),
        ('7', 6),
        ('6', 5),
        ('5', 4),
        ('4', 3),
        ('3', 2),
        ('2', 1),
    ])
});

static CARD_VALUES_WITH_JOKER: Lazy<HashMap<char, usize>> = Lazy::new(|| {
    HashMap::from([
        ('A', 13),
        ('K', 12),
        ('Q', 11),
        ('T', 9),
        ('9', 8),
        ('8', 7),
        ('7', 6),
        ('6', 5),
        ('5', 4),
        ('4', 3),
        ('3', 2),
        ('2', 1),
        ('J', 0),
    ])
});

#[derive(PartialEq, Eq, PartialOrd, Ord)]
enum HandRank {
    None,
    HighCard,
    OnePair,
    TwoPairs,
    ThreeOfAKind,
    FullHouse,
    FourOfAKind,
    FiveOfAKind,
}

pub fn run(config: Configuration) -> io::Result<String> {
    if let Some(input_buf) = config.input_file_buffer {
        let mut hands: Vec<Hand> = vec![];

        for line in input_buf.lines() {
            if let Ok(s) = line {
                hands.push(Hand::from_str(&s)?);
            }
        }

        hands.iter_mut().for_each(|h| h.set_hand_rank(false));
        hands.sort();
        let mut p1_winnings = 0usize;
        hands
            .iter()
            .enumerate()
            .for_each(|(index, hand)| p1_winnings += hand.bid * (index + 1));

        hands.iter_mut().for_each(|h| h.set_hand_rank(true));
        hands.sort();
        let mut p2_winnings = 0usize;
        hands
            .iter()
            .enumerate()
            .for_each(|(index, hand)| p2_winnings += hand.bid * (index + 1));

        Ok(format!("part1: {}, part2: {}", p1_winnings, p2_winnings))
    } else {
        Err(io::Error::new(ErrorKind::Other, "Input file required"))
    }
}

#[derive(Eq)]
struct Hand {
    hand_string: String,
    histogram: HashMap<char, usize>,
    hand_rank: HandRank,
    bid: usize,
    j_as_joker: bool,
}

impl Hand {
    pub fn new(hand_str: &str) -> Self {
        let histogram: HashMap<char, usize> = [
            ('A', 0),
            ('K', 0),
            ('Q', 0),
            ('J', 0),
            ('T', 0),
            ('9', 0),
            ('8', 0),
            ('7', 0),
            ('6', 0),
            ('5', 0),
            ('4', 0),
            ('3', 0),
            ('2', 0),
        ]
        .iter()
        .cloned()
        .collect();

        Self {
            hand_string: String::from(hand_str),
            histogram,
            hand_rank: HandRank::None,
            bid: 0,
            j_as_joker: false,
        }
    }

    pub fn set_hand_rank(&mut self, j_as_joker: bool) {
        let mut buf = String::new();
        let mut counts = Vec::<usize>::new();
        let mut j_count = 0usize;

        self.histogram.iter().for_each(|(k, v)| {
            if *v != 0 {
                if j_as_joker && *k == 'J' {
                    j_count = *v;
                } else {
                    counts.push(*v);
                }
            }
        });

        if j_as_joker {
            if let Some(max) = counts.iter_mut().max() {
                *max += j_count;
            } else {
                counts.push(j_count);
            }
        }

        counts.sort();
        counts
            .iter()
            .for_each(|count| buf.push_str(&count.to_string()));

        self.j_as_joker = j_as_joker;
        self.hand_rank = match buf.as_str() {
            "5" => HandRank::FiveOfAKind,
            "14" => HandRank::FourOfAKind,
            "23" => HandRank::FullHouse,
            "113" => HandRank::ThreeOfAKind,
            "122" => HandRank::TwoPairs,
            "1112" => HandRank::OnePair,
            "11111" => HandRank::HighCard,
            _ => HandRank::None,
        }
    }
}

impl PartialEq for Hand {
    fn eq(&self, other: &Self) -> bool {
        self.hand_rank == other.hand_rank
    }
}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Hand {
    fn cmp(&self, other: &Self) -> Ordering {
        let ord = self.hand_rank.cmp(&other.hand_rank);
        if let Ordering::Equal = ord {
            let self_hand_bytes = self.hand_string.as_bytes();
            let other_hand_bytes = other.hand_string.as_bytes();
            for i in 0..5 {
                let self_char = self_hand_bytes[i] as char;
                let other_char = other_hand_bytes[i] as char;
                let char_ord = if self.j_as_joker {
                    CARD_VALUES_WITH_JOKER
                        .get(&self_char)
                        .unwrap()
                        .cmp(&CARD_VALUES_WITH_JOKER.get(&other_char).unwrap())
                } else {
                    CARD_VALUES
                        .get(&self_char)
                        .unwrap()
                        .cmp(&CARD_VALUES.get(&other_char).unwrap())
                };
                if let Ordering::Equal = char_ord {
                    continue;
                } else {
                    return char_ord;
                }
            }
            Ordering::Equal
        } else {
            ord
        }
    }
}

impl FromStr for Hand {
    type Err = io::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let split = s.split(" ").collect::<Vec<_>>();
        let hand_str = split[0];
        let bid_str = split[1];
        let mut hand = Hand::new(hand_str);
        for c in hand_str.chars() {
            if let Some(n) = hand.histogram.get_mut(&c) {
                *n += 1;
            }
        }

        hand.bid = bid_str.parse::<usize>().unwrap();

        Ok(hand)
    }
}
