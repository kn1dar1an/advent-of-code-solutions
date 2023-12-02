mod configuration;
mod solution;
use std::io::{self, stdin, Read};
use std::{env, time::Instant};

use configuration::Configuration;

fn main() -> io::Result<()>{
    let args: Vec<String> = env::args().collect::<Vec<String>>();
    let config = Configuration::new(args)?;

    println!("Press enter key to start");
    stdin().read_exact(&mut [0u8]).unwrap();
    
    let now = Instant::now();

    let result = solution::run(config)?;

    let elapsed_time = now.elapsed();

    println!("Result: {}", result);
    println!("\nElapsed time: {}", format!("{:?}", elapsed_time));


    Ok(())
}
