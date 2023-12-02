use std::fs::File;
use std::io;

pub struct Configuration {
    pub input_file_buffer: Option<io::BufReader<File>>,
}

impl Configuration {
    pub fn new(args: Vec<String>) -> io::Result<Self> {
        if args.len() == 2 {
            let path = &args[1];
            let input_file = File::open(&path)?;
            
            Ok(Configuration { input_file_buffer: Some(io::BufReader::new(input_file)) })
        } else { 
            Ok(Configuration { input_file_buffer: None })
        }
    }
}
