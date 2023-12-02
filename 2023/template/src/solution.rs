use std::io::{BufRead, self, ErrorKind};
use crate::configuration::Configuration;

pub fn run(config: Configuration) -> io::Result<&'static str> {
    if let Some(input_buf) = config.input_file_buffer {
        for line in input_buf.lines() {
            if let Ok(s) = line { println!("{}", s); }
        }
        Ok("")
    } else {
        Err(io::Error::new(ErrorKind::Other, "Input file required"))
    }
}
