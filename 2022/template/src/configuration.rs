pub struct Configuration {
    pub input_file_path: Option<String>,
}

pub fn parse_args_to_config(args: Vec<String>) -> Configuration {
    if args.len() > 1 {    
        let path = args[1].clone();
        Configuration { input_file_path: Some(path) }
    } else { 
        Configuration { input_file_path: None }
    }
}
