use std::fs::{File};

use crate::configuration::Configuration;

pub fn run(config: &Configuration) -> Option<String>{
    let input_file: File;
    if config.input_file_path.is_some() {
        input_file = match File::open(config.input_file_path.as_ref().unwrap()) {
            Ok(f) => f,
            Err(reason) => panic!("Couldn't open file {:?}: {}", config.input_file_path.as_ref().unwrap(), reason)
        };
    };

    println!("Hello world!");

    Some(String::new())
}
