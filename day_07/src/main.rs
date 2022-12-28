use std::io::{BufReader, BufRead}; 
use std::fs::File;
use std::rc::{Weak, Rc};
use std::cell::RefCell;
use std::collections::HashMap;
use crate::NodeType::*;

#[derive(Debug, Clone, Copy)]
enum NodeType {
    File,
    Directory
}

#[derive(Debug)]
struct FileNode {
    node_name: String,
    node_type: NodeType,
    node_size: i64,
    entries: RefCell<HashMap<String, Rc<FileNode>>>,
    parent: Weak<FileNode>
}

/* Solution for the first step
fn get_size(node: &Rc<FileNode>, total_size_below_treshold: &mut i64, treshold: i64) -> i64 {
    match node.node_type {
        NodeType::File => node.node_size,
        Directory => {
            let mut total_dir_size = 0;

            for child_node in node.entries.borrow().values() {
                total_dir_size += get_size(child_node, total_size_below_treshold, treshold);
            }

            if total_dir_size <= treshold {
                *total_size_below_treshold += total_dir_size;
            }

            return total_dir_size;
        }
    }
}
*/

fn get_all_dirs_sizes(node: &Rc<FileNode>, dirs_sizes: &mut Vec<i64>) -> i64 {
    match node.node_type {
        NodeType::File => node.node_size,
        Directory => {
            let mut total_dir_size = 0;

            for child_node in node.entries.borrow().values() {
                total_dir_size += get_all_dirs_sizes(child_node, dirs_sizes);
            }

            dirs_sizes.push(total_dir_size);

            return total_dir_size;
        }
    }
}

fn main() {
    let mut buffer = String::new();
    let file = File::open("input.txt").unwrap();
    let mut reader = BufReader::new(file);

    let root_node = Rc::new(FileNode {
        node_name: String::from("/"),
        node_type: Directory,
        node_size: 0,
        entries: RefCell::new(HashMap::new()),
        parent: Weak::new()
    });

    // ommit cd / - I did it above
    let _ = reader.read_line(&mut buffer);
    buffer.clear();

    let mut current_node = Rc::clone(&root_node);

    while let Ok(bytes_read) = reader.read_line(&mut buffer) {
        if bytes_read == 0 {
            break;
        }

        let parts: Vec<_> = buffer.trim_end().split(" ").collect();

        match parts[0] {
            "$" => {
                match parts[1] {
                    "cd" => {
                        let dir_name = parts[2];

                        if dir_name == ".." {
                            if let Some(rc_parent) = current_node.parent.upgrade() {
                                current_node = rc_parent;
                            } else {
                                panic!("Can't go beyond root node!");
                            }
                        } else {
                            let current_copy = Rc::clone(&current_node);
                            let mut current_entries = current_copy.entries.borrow_mut();

                            if let Some(next_file_node) = current_entries.get(dir_name) {
                                current_node = Rc::clone(next_file_node);
                            } else {
                                let node_to_go = Rc::new(FileNode {
                                    node_name: dir_name.to_string(),
                                    node_type: Directory,
                                    node_size: 0,
                                    entries: RefCell::default(),
                                    parent: Rc::downgrade(&current_node)
                                });

                                current_node = Rc::clone(&node_to_go);
                                current_entries.insert(dir_name.to_string(), node_to_go);
                            }
                        }
                    },
                    "ls" => { },
                    _ => {
                        panic!("Unknown command");
                    }
                }
            },
            "dir" => {
                let mut mut_entries = current_node.entries.borrow_mut();
                let dir_name = parts[1].to_string();

                mut_entries.insert(dir_name.clone(), Rc::new(FileNode {
                    node_name: dir_name,
                    node_type: Directory,
                    node_size: 0,
                    entries: RefCell::default(),
                    parent: Rc::downgrade(&current_node)
                }));
            },
            _ => {
                let file_size = parts[0].parse().unwrap();
                let file_name = parts[1].to_string();
                let mut mut_entires = current_node.entries.borrow_mut();

                mut_entires.insert(file_name.clone(), Rc::new(FileNode {
                    node_name: file_name,
                    node_type: File,
                    node_size: file_size,
                    entries: RefCell::default(),
                    parent: Rc::downgrade(&current_node)
                }));
            }
        }

        buffer.clear();
    }

    const SIZE_OF_FILE_SYSTEM: i64 = 70_000_000;
    const REQUIRED_FREE_SPACE : i64 = 30_000_000;
    let mut accumulator = Vec::new();
    let size_of_root = get_all_dirs_sizes(&root_node, &mut accumulator);

    if SIZE_OF_FILE_SYSTEM - size_of_root >= REQUIRED_FREE_SPACE {
        println!("No need to delete anything");
    } else {
        let space_required_to_free = REQUIRED_FREE_SPACE - (SIZE_OF_FILE_SYSTEM - size_of_root);
        let size_of_dir_to_remove = accumulator.iter()
            .filter(|dir_size| **dir_size >= space_required_to_free)
            .min()
            .unwrap();
        println!("{}", size_of_dir_to_remove);
    }
}
