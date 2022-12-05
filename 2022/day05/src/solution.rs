use crate::configuration::Configuration;
use std::{
    collections::VecDeque,
    fs::File,
    io::{BufRead, BufReader},
};

#[allow(dead_code)]
enum InstructionMethod {
    FiFo,
    FiLo,
}

pub fn run(config: Configuration) -> Result<String, String> {
    if config.input_file_buffer.is_some() {
        // Part 1
        //let im: InstructionMethod = InstructionMethod::FiFo;
        // Part 2
        let im: InstructionMethod = InstructionMethod::FiLo;
        let result_stacks = execute_instructions(parse_file(config.input_file_buffer.unwrap()), im);

        Ok(get_readout(result_stacks))
    } else {
        panic!("Could not open file");
    }
}

pub fn parse_file(b: BufReader<File>) -> (Vec<Vec<char>>, VecDeque<(i32, i32, i32)>) {
    let mut buffer = b.lines().map(|s| s.unwrap()).peekable();

    // Accounting for the missing ifnal whitespace, each stack is 4 chars long
    let stack_count: i32 = buffer.peek().unwrap().chars().count() as i32 + 1 / 4;

    let mut next_part: bool = false;
    let mut crates: VecDeque<String> = VecDeque::new();
    let mut instructions: VecDeque<(i32, i32, i32)> = VecDeque::new();
    let mut stacks: Vec<Vec<char>> = Vec::with_capacity(stack_count as usize);

    for line in buffer {
        if line != "" {
            if line.get(0..2).unwrap() == " 1" {
                next_part = true;
                continue;
            };
            if !next_part {
                let mut temp: String = String::new();
                let mut s = line.chars();
                for _ in 0..stack_count {
                    if let Some(c) = s.nth(1) {
                        temp.push(c);
                    }
                }
                crates.push_front(temp);
            } else {
                let items: Vec<&str> = line.split_whitespace().collect::<Vec<_>>();
                instructions.push_back((
                    items.get(1).unwrap().parse::<i32>().unwrap(),
                    items.get(3).unwrap().parse::<i32>().unwrap(),
                    items.get(5).unwrap().parse::<i32>().unwrap(),
                ));
            }
        }
    }

    for _ in 0..stack_count {
        stacks.push(Vec::new())
    }

    for level in &crates {
        let iter = level
            .chars()
            .enumerate()
            .filter(|&(i, _)| i % 2 == 0)
            .map(|(_, c)| c)
            .enumerate();
        for (i, c) in iter {
            match c {
                ' ' => continue,
                _ => stacks.get_mut(i).unwrap().push(c),
            };
        }
    }

    (stacks, instructions)
}

/*
 * Here, as_chunk indicates if crates are taken from the stacks one-by-one or as a as_chunk.
 * In practice, the temp_deque should change between FiFo (one-by-one) or FiLo (as a chunk)*/
fn execute_instructions(
    (mut stacks, instructions): (Vec<Vec<char>>, VecDeque<(i32, i32, i32)>),
    as_chunk: InstructionMethod,
) -> Vec<Vec<char>> {
    for (amnt, src, dst) in instructions.iter() {
        let mut temp_deque: VecDeque<char> = VecDeque::with_capacity(*amnt as usize);
        {
            let src_stack = stacks.get_mut(*src as usize - 1).unwrap();
            for _ in 0..*amnt {
                if let Some(c) = src_stack.pop() {
                    temp_deque.push_back(c);
                }
            }
        }
        {
            let dst_stack = stacks.get_mut(*dst as usize - 1).unwrap();
            for _ in 0..temp_deque.len() {
                let c = match as_chunk {
                    InstructionMethod::FiFo => temp_deque.pop_front().unwrap(),
                    InstructionMethod::FiLo => temp_deque.pop_back().unwrap(),
                };
                dst_stack.push(c.clone());
            }
        }
    }

    stacks
}

fn get_readout(stacks: Vec<Vec<char>>) -> String {
    let mut s = String::new();

    for mut stack in stacks {
        if let Some(c) = stack.pop() {
            s.push(c);
        }
    }

    s
}
