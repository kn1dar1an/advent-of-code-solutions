use std::io::BufRead;
use std::collections::BinaryHeap;
use crate::configuration::Configuration;

pub fn run(config: Configuration) -> Result<String, String>{
    let mut heap = BinaryHeap::new();

    if config.input_file_buffer.is_some() {
        let mut temp_total: i32 = 0;
        for line in config.input_file_buffer.unwrap().lines() {
            if let Ok(calories) = line {
                if calories == "" {
                    heap.push(temp_total);
                    temp_total = 0;
                } else {
                    if let Ok(n) = calories.parse::<i32>() {
                        temp_total += n;
                    }
                }
            }
        }
        
        let mut ret: i32 = 0;
        for _ in 1..=3 {
            if let Some(x) = heap.pop() {
                ret += x;
            }
        }

        return Ok(ret.to_string());
    } else {
        Err("File was not opened!".to_string())
    }
    
}
