use std::io::BufRead;
use crate::configuration::Configuration;

pub fn run(config: Configuration) -> Result<String, String>{

    if config.input_file_buffer.is_some() {
        let mut count: i32 = 0;
        for line in config.input_file_buffer.unwrap().lines() {
            if let Ok(s) = line { 
                // Part 1 
                // if is_one_fully_contained(get_assignments(s)) { count += 1; }
                // Part 2
                if are_overlapping(get_assignments(s)) { count += 1 }
             }
        }

        Ok(count.to_string())
    } else {
        panic!("A file was not provided");
    }
    
}

/* Part 1 */
#[allow(dead_code)]
pub fn is_one_fully_contained(a: ((u8, u8), (u8, u8))) -> bool {
    if a.0.0 == a.1.0 || a.0.1 == a.1.1 { 
        true 
    } else if a.0.0 > a.1.0 && a.0.1 <= a.1.1 {
        true
    } else if a.0.0 < a.1.0 && a.0.1 >= a.1.1 {
        true
    } else {
        false
    }
}

pub fn are_overlapping(a: ((u8, u8), (u8, u8))) -> bool {
    if a.0.1 >= a.1.0 && a.0.0 <= a.1.1 {
        true
    } else if a.0.0 >= a.1.0 && a.0.0 <= a.1.1 {
        true
    } else {
        false
    }
}

pub fn get_assignments(line: String) -> ((u8, u8), (u8, u8)) {
    let pair: Vec<(u8, u8)> = line
    .split(',')
    .map(|s| { //3-6
        let sections: Vec<u8> = s
        .split('-')
        .map(|s| s.parse::<u8>().unwrap())
        .collect::<Vec<_>>();

        (sections.get(0).unwrap().clone(), sections.get(1).unwrap().clone())
    })
    .collect::<Vec<_>>();

    ((pair[0].0, pair[0].1),(pair[1].0 , pair[1].1))
}