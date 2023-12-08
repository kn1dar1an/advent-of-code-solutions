use crate::configuration::Configuration;
use std::{
    cell::RefCell,
    collections::HashMap,
    io::{self, BufRead, ErrorKind},
    rc::Rc,
};

pub fn run(config: Configuration) -> io::Result<String> {
    if let Some(input_buf) = config.input_file_buffer {
        let mut input_iterator = input_buf.lines();

        let instruction = input_iterator.next().unwrap()?;
        let mut nodes_map = HashMap::<String, Rc<RefCell<Node>>>::new();

        // skip empty line
        input_iterator.next();

        // create rest
        for line in input_iterator {
            if let Ok(s) = line {
                Node::new_from_string(s, &mut nodes_map);
            }
        }

        let root_node = nodes_map.get("AAA").unwrap().clone();
        let part_1 = simulate_instructions(&root_node, "ZZZ", &instruction);

        let mut part_2_root_nodes = vec![];
        nodes_map.iter().for_each(|(k, v)| {
            if k.ends_with('A') {
                part_2_root_nodes.push(v.clone())
            }
        });

        let part_2_counts = simulate_multiple(&part_2_root_nodes, "Z", &instruction);

        let part_2 = part_2_counts.iter().cloned().fold(1, lcm);

        Ok(format!("part1: {}, part2: {}", part_1, part_2))
    } else {
        Err(io::Error::new(ErrorKind::Other, "Input file required"))
    }
}

fn simulate_instructions(
    root_node: &Rc<RefCell<Node>>,
    ending_in: &str,
    instruction: &str,
) -> usize {
    let mut count = 0usize;
    let mut current_node = root_node.clone();
    let mut curent_iterator = instruction.as_bytes().iter();

    loop {
        if current_node.borrow().name.ends_with(ending_in) {
            break;
        }

        if let Some(char) = curent_iterator.next() {
            count += 1;
            current_node = if *char as char == 'L' {
                if let Some(left_node) = &current_node.borrow().left {
                    left_node.clone()
                } else {
                    Rc::default()
                }
            } else if let Some(right_node) = &current_node.borrow().right {
                right_node.clone()
            } else {
                Rc::default()
            }
        } else {
            curent_iterator = instruction.as_bytes().iter();
        }
    }

    count
}

// Until reaching a node ending in Z
fn simulate_multiple(
    root_nodes: &Vec<Rc<RefCell<Node>>>,
    ending_in: &str,
    instruction: &str,
) -> Vec<usize> {
    let mut counts: Vec<usize> = vec![];

    for root_node in root_nodes {
        counts.push(simulate_instructions(root_node, ending_in, instruction));
    }

    counts
}

fn gcd(a: usize, b: usize) -> usize {
    // Euclidean division
    if b == 0 {
        a
    } else {
        gcd(b, a % b)
    }
}

fn lcm(a: usize, b: usize) -> usize {
    b * (a / gcd(a, b))
}

#[derive(Default, Eq)]
struct Node {
    name: String,
    left: Option<Rc<RefCell<Node>>>,
    right: Option<Rc<RefCell<Node>>>,
}

impl Node {
    fn new_with_name(name: &String) -> Self {
        Self {
            name: name.to_string(),
            left: None,
            right: None,
        }
    }

    pub fn new_from_string(
        node_string: String,
        nodes_map: &mut HashMap<String, Rc<RefCell<Node>>>,
    ) -> Rc<RefCell<Node>> {
        let node_string = node_string.replace([' ', '(', ')'], "");

        let parts = node_string.split('=').collect::<Vec<_>>();
        let node_name = parts[0].to_string();
        let branches = parts[1].split(',').collect::<Vec<_>>();

        let self_node = nodes_map
            .entry(node_name)
            .or_insert_with_key(|key| Rc::new(RefCell::new(Node::new_with_name(key))))
            .clone();

        let left_node_name = branches[0].to_string();
        let left = nodes_map
            .entry(left_node_name)
            .or_insert_with_key(|key| Rc::new(RefCell::new(Node::new_with_name(key))))
            .clone();

        let right_node_name = branches[1].to_string();
        let right = nodes_map
            .entry(right_node_name)
            .or_insert_with_key(|key| Rc::new(RefCell::new(Node::new_with_name(key))))
            .clone();

        let mut self_node_ref = self_node.borrow_mut();
        if self_node_ref.left.is_none() {
            self_node_ref.left = Some(left);
        };

        if self_node_ref.right.is_none() {
            self_node_ref.right = Some(right);
        };

        self_node.clone()
    }
}

impl PartialEq for Node {
    fn eq(&self, other: &Self) -> bool {
        self.name == other.name
    }
}
