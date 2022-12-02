use std::io::BufRead;
use crate::configuration::Configuration;

pub fn run(config: Configuration) -> Result<String, String>{

    if config.input_file_buffer.is_some() {
        for line in config.input_file_buffer.unwrap().lines() {
            if let Ok(s) = line { println!("{}", s); }
        }
    } else {
        println!("Hello Small World!");
    }
    
    Ok(String::new())
}
