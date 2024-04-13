use std::{collections::HashMap, fs, io::Write, vec};

use crate::reverse_summation_node::{deserialize, ReverseSummationNode};

pub mod reverse_summation_node;

fn main() {
    println!("Hello, world!");
    let mut tree = ReverseSummationNode {
        id: 0,
        count: 0,
        children: HashMap::new(),
    };

    tree.add_reverse_ngram(&vec![1, 2, 3, 4]);
    tree.add_reverse_ngram(&vec![1, 2, 2, 4]);
    tree.add_reverse_ngram(&vec![1, 3, 2, 4]);

    if let Some(node) = tree.query_reverse_ngram(&vec![1, 2]) {
        println!("{}", node.count);
    }

    println!("{}", tree.to_string());

    let mut file = fs::OpenOptions::new()
        .write(true)
        .open("bin/tree.rst")
        .unwrap();
    let _ = file.write_all(&tree.serialize());

    let data = fs::read("bin/tree.rst").unwrap();
    if let Ok(read_node) = deserialize(data) {
        println!("{}", read_node.to_string());
    }
}
