use crate::configuration::Configuration;
use std::io::{self, ErrorKind};

use self::almanac::Almanac;

mod almanac;

pub fn run(config: Configuration) -> io::Result<String> {
    if let Some(input_buf) = config.input_file_buffer {
        let almanac = Almanac::new_from_buf(input_buf)?;

        let part1 = almanac.get_lowest_location();
        let part2 = almanac.get_lowest_location_with_seed_range();

        Ok(format!("part1: {}, part2: {}", part1, part2))
    } else {
        Err(io::Error::new(ErrorKind::Other, "Input file required"))
    }
}
