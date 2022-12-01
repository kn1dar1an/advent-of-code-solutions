mod configuration;
mod solution;
use std::fs::File;
use std::io::{self, BufRead, BufReader};
use std::{env, time::Instant};

use configuration::Configuration;
//use crate::configuration::Configuration;
fn main() {
    let args: Vec<String> = env::args().collect::<Vec<String>>();
    let config: configuration::Configuration = configure(args);
    
    let now = Instant::now();

    solution::run(config);

    let elapsed_time = now.elapsed();
    println!("\nTook: {} microseconds.", elapsed_time.as_micros());
}

pub fn configure(args: Vec<String>) -> configuration::Configuration {
    if args.len() == 2 {
        let path = &args[1];
        let input_file: Option<File> = match File::open(&path) {
            Ok(f) => Some(f),
            Err(reason) => panic!("Couldn't open file {:?}: {}", path, reason)
        };
        Configuration { input_file_buffer: Some(BufReader::new(input_file.unwrap())) }
    } else { 
        Configuration { input_file_buffer: None }
    }
}
