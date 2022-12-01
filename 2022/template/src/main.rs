use std::{env, time::Instant};
mod configuration;
mod solution;

fn main() {
    let args: Vec<String> = env::args().collect::<Vec<String>>();
   
    let mut config: configuration::Configuration = configuration::parse_args_to_config(args);
    
    let now = Instant::now();

    solution::run(&mut config);

    let elapsed_time = now.elapsed();
    println!("\nTook: {} microseconds.", elapsed_time.as_micros());
}
