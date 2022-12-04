use std::fs::File;
use std::io;

pub struct Configuration {
    pub input_file_buffer: Option<io::BufReader<File>>,
}
