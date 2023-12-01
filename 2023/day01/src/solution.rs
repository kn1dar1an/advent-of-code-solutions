use crate::configuration::Configuration;
use std::io::BufRead;

static NUMBERS: [&str; 9] = [
    "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
];

enum SearchPos {
    Beg,
    End,
}

pub fn run(config: Configuration) -> Result<String, String> {
    if config.input_file_buffer.is_some() {
        let mut sum = 0u32;

        for line in config.input_file_buffer.unwrap().lines() {
            if let Ok(s) = line {
                if let Some(first_number) = first_number_occurrence(&s, SearchPos::Beg) {
                    sum += first_number * 10;
                }
                if let Some(last_number) = first_number_occurrence(&s, SearchPos::End) {
                    sum += last_number;
                }
            }
        }
        Ok(sum.to_string())
    } else {
        Err(String::from("Need input file!"))
    }
}

fn first_number_occurrence(buf: &String, starting_pos: SearchPos) -> Option<u32> {
    let last_ix = buf.len() - 1;
    let mut start_ix = if let SearchPos::Beg = starting_pos {
        0
    } else {
        last_ix
    };

    loop {
        if let Ok(digit) = buf[start_ix..=start_ix].parse::<u32>() {
            return Some(digit);
        }

        let end_ix = if last_ix - start_ix > 5 {
            start_ix + 4
        } else {
            last_ix
        };

        let slice = &buf[start_ix..=end_ix];

        for (i, number_slice) in NUMBERS.into_iter().enumerate() {
            if slice.starts_with(number_slice) {
                return Some(i as u32 + 1);
            }
        }

        match starting_pos {
            SearchPos::Beg => {
                if start_ix == last_ix {
                    return None;
                } else {
                    start_ix += 1;
                }
            }
            SearchPos::End => {
                if start_ix == 0 {
                    return None;
                } else {
                    start_ix -= 1;
                }
            }
        }
    }
}
