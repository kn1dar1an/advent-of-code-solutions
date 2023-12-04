use crate::configuration::Configuration;
use std::io::{self, ErrorKind};

use self::schematic::Schematic;

mod schematic;

static NUMBERS: [char; 10] = ['1', '2', '3', '4', '5', '6', '7', '8', '9', '0'];

pub fn run(config: Configuration) -> io::Result<String> {
    if let Some(input_buf) = config.input_file_buffer {
        let schematic = Schematic::new(input_buf)?;

        let part1 = schematic.find_component_sum();
        let part2 = schematic.find_gear_ratio_sum();

        Ok(format!("part1: {}, part2: {}", part1, part2))
    } else {
        Err(io::Error::new(ErrorKind::Other, "Input file required"))
    }
}
