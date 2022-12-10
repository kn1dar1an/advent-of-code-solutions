use crate::configuration::Configuration;
use std::{
    cell::RefCell,
    cmp::{Ordering, Reverse},
    collections::BinaryHeap,
    fs,
    io::{BufRead, BufReader},
    rc::{Rc, Weak},
};

type NodeLink = Rc<RefCell<Directory>>;
type ParentNodeLink = Weak<RefCell<Directory>>;

const TOTAL_SPACE: i32 = 70000000;
const TARGET_FREE_SPACE: i32 = 30000000;

#[derive(Debug)]
struct Tree {
    root: NodeLink,
    total_used_space: i32,
}

impl Tree {
    fn new(root_name: String) -> Self {
        Self {
            root: Directory::new(root_name),
            total_used_space: 0,
        }
    }

    fn add_subdir(parent_link: &NodeLink, name: String) {
        let mut parent = parent_link.borrow_mut();
        let subdir = Directory::new_with_parent(name, parent_link);

        parent.sub_directories.push(subdir);
    }

    pub fn sum_dir_size_if_up_to(&self, accumulator: &mut i32, parent: &NodeLink, threshold: i32) {
        for subdir in parent.borrow_mut().sub_directories.iter() {
            let size = subdir.borrow().size;
            if size <= threshold {
                *accumulator += size;
            }
            self.sum_dir_size_if_up_to(accumulator, subdir, threshold);
        }
    }

    pub fn find_suitable_dirs_for_deletion(
        &self,
        collection: &mut BinaryHeap<Reverse<i32>>,
        parent: &NodeLink,
        free_space: i32,
    ) {
        for subdir in parent.borrow_mut().sub_directories.iter() {
            let size = subdir.borrow().size;
            if free_space + size >= TARGET_FREE_SPACE {
                collection.push(Reverse(size));
            }
            self.find_suitable_dirs_for_deletion(collection, subdir, free_space);
        }
    }
}

#[derive(Debug)]
struct Directory {
    name: String,
    files: BinaryHeap<File>,
    parent_directory: Option<ParentNodeLink>,
    sub_directories: BinaryHeap<NodeLink>,
    size: i32,
}

impl Directory {
    fn new(name: String) -> NodeLink {
        Rc::new(RefCell::new(Self {
            name,
            files: BinaryHeap::new(),
            parent_directory: None,
            sub_directories: BinaryHeap::new(),
            size: 0,
        }))
    }

    fn new_with_parent(name: String, parent_directory: &NodeLink) -> NodeLink {
        Rc::new(RefCell::new(Self {
            name,
            files: BinaryHeap::new(),
            parent_directory: Some(Rc::downgrade(parent_directory)),
            sub_directories: BinaryHeap::new(),
            size: 0,
        }))
    }

    fn add_file(&mut self, name: String, size: i32) {
        self.files.push(File::new(name, size));
        self.mod_size(size);
    }

    fn mod_size(&mut self, by: i32) {
        self.size += by;
        if let Some(weak_parent) = &self.parent_directory {
            if let Some(parent) = Weak::upgrade(weak_parent) {
                parent.borrow_mut().mod_size(by);
            }
        };
    }
}

impl PartialEq for Directory {
    fn eq(&self, other: &Self) -> bool {
        self.name == other.name && self.size == other.size
    }
}

impl Eq for Directory {}

impl Ord for Directory {
    fn cmp(&self, other: &Self) -> Ordering {
        self.size.cmp(&other.size)
    }
}

impl PartialOrd for Directory {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        match self.name.partial_cmp(&other.name) {
            Some(core::cmp::Ordering::Equal) => {}
            ord => return ord,
        }
        self.size.partial_cmp(&other.size)
    }
}

#[derive(Debug, PartialEq, Eq, PartialOrd)]
struct File {
    name: String,
    file_size: i32,
}

impl File {
    fn new(name: String, file_size: i32) -> Self {
        File { name, file_size }
    }
}

impl Ord for File {
    fn cmp(&self, other: &Self) -> Ordering {
        self.file_size.cmp(&other.file_size)
    }
}

pub fn run(config: Configuration) -> Result<String, String> {
    if config.input_file_buffer.is_some() {
        let tree = parse_file(config.input_file_buffer.unwrap());

        let mut accumulator: i32 = 0;
        let mut size_heap: BinaryHeap<Reverse<i32>> = BinaryHeap::new();
        tree.sum_dir_size_if_up_to(&mut accumulator, &tree.root, 100000);

        tree.find_suitable_dirs_for_deletion(
            &mut size_heap,
            &tree.root,
            TOTAL_SPACE - tree.total_used_space,
        );

        Ok(format!(
            "\nPart 1: {}\nPart 2: {}",
            accumulator.to_string(),
            size_heap.pop().unwrap().0
        ))
    } else {
        panic!("No input file given!");
    }
}

fn parse_file(mut buf: BufReader<fs::File>) -> Tree {
    let mut tree: Tree = Tree::new("/".to_string());

    let root_ref: Rc<RefCell<Directory>> = tree.root.clone();

    let mut current_dir = root_ref;

    let mut line: String = String::new();
    while let Ok(bytes_read) = buf.read_line(&mut line) {
        if bytes_read > 0 {
            let command_args = line.split_whitespace().map(|s| s).collect::<Vec<_>>();
            if let Some(&s) = command_args.get(0) {
                if s == "$" {
                    if let Some(&command) = command_args.get(1) {
                        match command {
                            "cd" => {
                                if let Some(&name) = command_args.get(2) {
                                    match name {
                                        "/" => {
                                            current_dir = tree.root.clone();
                                        }
                                        ".." => {
                                            if let Some(dir) =
                                                &current_dir.clone().borrow().parent_directory
                                            {
                                                if let Some(parent_ref) = Weak::upgrade(dir) {
                                                    current_dir = parent_ref.clone();
                                                }
                                            }
                                        }
                                        name @ _ => {
                                            if let Some(dir) = current_dir
                                                .clone()
                                                .borrow()
                                                .sub_directories
                                                .iter()
                                                .find(|&dir| dir.borrow().name == name.to_string())
                                            {
                                                current_dir = dir.clone();
                                            };
                                        }
                                    }
                                }
                            }
                            "ls" => loop {
                                let mut ls_buf = String::new();
                                if let Ok(bytes_read) = buf.read_line(&mut ls_buf) {
                                    if bytes_read > 0 {
                                        let args = ls_buf.split_whitespace().collect::<Vec<_>>();
                                        if let Some(&s) = args.get(0) {
                                            match s {
                                                "$" => {
                                                    buf.seek_relative(
                                                        -(ls_buf.bytes().len() as i64),
                                                    )
                                                    .unwrap();
                                                    break;
                                                }
                                                "dir" => {
                                                    Tree::add_subdir(
                                                        &current_dir,
                                                        args.get(1).unwrap().to_string(),
                                                    );
                                                }
                                                _ => {
                                                    current_dir.borrow_mut().add_file(
                                                        args.get(1).unwrap().to_string(),
                                                        args.get(0)
                                                            .unwrap()
                                                            .parse::<i32>()
                                                            .unwrap(),
                                                    );
                                                }
                                            }
                                        }
                                    } else {
                                        break;
                                    }
                                }
                            },
                            c @ _ => {
                                panic!("Unkown command: {}", c)
                            }
                        };
                    }
                }
            }
            line.clear();
        } else {
            break;
        }
    }

    tree.total_used_space = tree.root.clone().borrow().size;

    tree
}
