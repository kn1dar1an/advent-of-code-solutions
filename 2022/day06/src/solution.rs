use std::{io::{BufReader, Read}, fs::File};
use crate::configuration::Configuration;

pub fn run(config: Configuration) -> Result<String, String>{

    if config.input_file_buffer.is_some() {
        if let Some(buf) = config.input_file_buffer {
            // Part 1
            // let res = find_marker(buf, 4);
            // Part 2
            let res = find_marker(buf, 14);
            Ok(res.to_string())
        } else {
            Err("Could not open file".to_string())
        }
    } else {
        panic!("No file provided!");
    }
}

fn find_marker(mut transmission: BufReader<File>, pattern_size: usize) -> usize {
    let mut start: usize = 0;
    let mut buf: Vec<u8> = vec![];

    if let Ok(_) = transmission.read_to_end(&mut buf) {
        for (i, chars) in buf.windows(pattern_size).enumerate() {
            if is_start(chars) { 
                start = i; 
                break; 
            }
        }

        // start + pattern_size as the iterator only iterates until the first element
        start + pattern_size
    } else {
        0
    }
}

fn is_start(s: &[u8]) -> bool {
    let mut are_duplicates = false;
    for i in 0..s.len() {
        for k in i + 1..s.len() {
            if s[i] == s[k] { are_duplicates = true; }
        }
    }

    !are_duplicates
}
