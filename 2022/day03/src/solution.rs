use crate::configuration::Configuration;
use std::{collections::HashSet, io::BufRead};

pub fn run(config: Configuration) -> Result<String, String> {
    if config.input_file_buffer.is_some() {
        /* Part 1
        let mut total_priority: i32 = 0;
        for line in config.input_file_buffer.unwrap().lines() {
            if let Ok(s) = line {
                let errors = find_errors(s);

                errors.iter().for_each(|item| total_priority += find_priority(item));
        } else if let Err(e) = line {
                return Err(format!("Couldn't read line: {}", e));
            }
        }
        */

        /* Part 2
         */
        let mut iterator = config
            .input_file_buffer
            .unwrap()
            .lines()
            .map(|l| l.expect("Couldn't read line"))
            .peekable();
        let mut total_priority: i32 = 0;
        loop {
            if iterator.peek().is_none() {
                break;
            }
            total_priority += find_priority(&find_badge_item_type(
                iterator
                    .next()
                    .unwrap(),
                iterator.next(),
                iterator.next(),
            ));
        }

        return Ok(total_priority.to_string());
    } else {
        panic!("A file was not provided");
    }
}

/*
Part 1
*/
#[allow(dead_code)]
pub fn find_errors(line: String) -> HashSet<u8> {
    let mut errors: HashSet<u8> = HashSet::new();
    let rucksack: &[u8] = line.as_bytes();
    let rucksack_size = rucksack.len();
    let compartment_size = rucksack_size / 2;

    for i in 0..compartment_size {
        for k in compartment_size..rucksack_size {
            if *rucksack.get(i).expect("Invalid item") == *rucksack.get(k).expect("Invalid item") {
                errors.insert(rucksack.get(i).unwrap().clone());
            }
        }
    }

    errors
}

pub fn find_badge_item_type(line1: String, line2: Option<String>, line3: Option<String>) -> u8 {
    let mut badge: u8 = 0u8;
    let rucksack1: &[u8] = line1.as_bytes();
    //let rucksack2: &[u8] = line2.as_bytes();
    //let rucksack3: &[u8] = line3.as_bytes();

    for item in rucksack1.iter() {
        if (line2.is_none() || line2.as_ref().unwrap().as_bytes().contains(item))
            && (line3.is_none() || line3.as_ref().unwrap().as_bytes().contains(item))
        {
            badge = *item;
            break;
        }
    }

    badge
}

/*
    Lowercase item types a through z have priorities 1 through 26.
        ASCII 97 - 122
    Uppercase item types A through Z have priorities 27 through 52.
        ASCII 65 - 90
*/
pub fn find_priority(item: &u8) -> i32 {
    let prio: u8 = if *item > 90u8 {
        //lowercase
        *item - 96
    } else {
        //uppercase
        *item - 38
    };

    prio as i32
}
