use std::{io::{BufReader, BufRead}, fs};
use ndarray::{Array, Ix2, Array2, ArrayView};

use crate::configuration::Configuration;

pub fn run(config: Configuration) -> Result<String, String>{

    if config.input_file_buffer.is_some() {
        let forest = parse_file(config.input_file_buffer.unwrap());
        let part_1 = count_visible_trees(&forest);
        let part_2 = count_highest_scenic_score(&forest);

        Ok(format!("\nPart 1: {}\nPart 2: {}", part_1, part_2))
    } else {
        panic!("No file provided!");
    }
}

fn count_visible_trees(forest: &Array<u8, Ix2>) -> i32 {
    let mut visible_trees = Array::<u8, _>::zeros(forest.raw_dim());

    {
        // E -> W
        for i in 0..forest.nrows() {
            let mut max: Option<u8> = None;
            for k in 0..forest.ncols() {
                if max.is_none() || forest[[i, k]] > max.unwrap() {
                    max = Some(forest[[i, k]]);
                    visible_trees[[i, k]] = 1;
                }
            }
        }
    }

    {
        // W -> E
        let mut i = forest.nrows();
        while i > 0 as usize{
            let mut k = forest.ncols();
            let mut max: Option<u8> = None;
             while k > 0 as usize {
                let ix_i = i - 1 as usize;
                let ix_k = k - 1 as usize;
                if max.is_none() || forest[[ix_i, ix_k]] > max.unwrap() {
                    max = Some(forest[[ix_i, ix_k]]);
                    visible_trees[[ix_i, ix_k]] = 1;
                }
                k -= 1 as usize;
             }
            i -= 1 as usize;
        }
    }

    {
        // N -> S
        for i in 0..forest.ncols() {
            let mut max: Option<u8> = None;
            for k in 0..forest.nrows() {
                if max.is_none() || forest[[k, i]] > max.unwrap() {
                    max = Some(forest[[k, i]]);
                    visible_trees[[k, i]] = 1;
                }
            }
        }
    }

    {
        // S -> N
    let mut i = forest.ncols();
        while i > 0 as usize {
            let mut k = forest.nrows();
            let mut max: Option<u8> = None;
            while k > 0 as usize {
                let ix_i = i - 1;
                let ix_k = k - 1;
                if max.is_none() || forest[[ix_k, ix_i]] > max.unwrap() {
                    max = Some(forest[[ix_k, ix_i]]);
                    visible_trees[[ix_k, ix_i]] = 1;
                }
                k -= 1;
            }
            i -= 1;
        }
    }
    visible_trees.fold(0, |acc, &t| { acc + t as i32})
}

fn count_highest_scenic_score(forest: &Array<u8, Ix2>) -> i32 {
    let mut max: i32 = 0;
    for i in 0..forest.nrows() {
        for k in 0..forest.ncols() {
            let current_tree = forest[[i, k]]; 
            
            let mut left_count: i32 = 0;
            let mut right_count: i32 = 0;
            let mut up_count: i32 = 0;
            let mut down_count: i32 = 0;

            
            //to left
            let mut ix_k = k.clone();
            while ix_k > 0 {
                ix_k -= 1;
                if forest[[i, ix_k]] < current_tree { 
                    left_count += 1;
                } else {
                    left_count += 1;
                    break;
                }
            }

            //to right
            let mut ix_k = k.clone();
            while ix_k < forest.ncols() - 1 {
                ix_k += 1;
                if forest[[i, ix_k]] < current_tree { 
                    right_count += 1;
                } else {
                    right_count += 1;
                    break;
                }
            }

            //up
            let mut ix_i = i.clone();
            while ix_i > 0 {
                ix_i -= 1;
                if forest[[ix_i, k]] < current_tree { 
                    up_count += 1;
                } else {
                    up_count += 1;
                    break;
                }
            }

            //down
            let mut ix_i = i.clone();
            while ix_i < forest.nrows() - 1 {
                ix_i += 1;
                if forest[[ix_i, k]] < current_tree { 
                    down_count += 1;
                } else {
                    down_count += 1;
                    break;
                }
            }

            
            let current_count = left_count * right_count * up_count * down_count;

            if current_count > max { max = current_count }
        }
    }

    max
}

fn parse_file(buf: BufReader<fs::File>) -> Array<u8, Ix2> {
    let mut iterator = buf.lines().peekable();

    let width: usize;
    if let Some(Ok(line)) = iterator.peek() {
        width = line.len();
    } else {
        panic!("Couldn't parse line");
    }

    let mut array: Array<u8, Ix2> = Array2::zeros((0,width));
    
    for result in iterator {
        if let Ok(line) = result {
            let chars = line.chars().map(|c| c.to_string().parse::<u8>().unwrap()).collect::<Vec<_>>();
            match array.push_row(ArrayView::from(chars.as_slice())) {
                Ok(()) => (),
                Err(error) => panic!("Error adding row: {}", error),
            }
        }
    }

    array
}