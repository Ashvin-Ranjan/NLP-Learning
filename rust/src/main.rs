use std::{cmp, collections::HashMap, env, fs, io::Write, vec};

use helpers::unicode_str_subsection;
use stupid_backoff_letter::generate_text;

use crate::{
    helpers::str_to_reverse_ngram,
    reverse_summation_node::{deserialize, ReverseSummationNode},
};

pub mod helpers;
pub mod reverse_summation_node;
mod stupid_backoff_letter;

const CONTEXT: usize = 12;

fn main() {
    let args: Vec<_> = env::args().collect();

    println!(
        "{}",
        unicode_str_subsection("Hello привет".to_string(), 2, 10)
    );

    println!("Hello, world!");
    let mut tree = ReverseSummationNode {
        id: 0,
        count: 0,
        children: HashMap::new(),
    };

    // tree.add_reverse_ngram(&vec![1, 2, 3, 4]);
    // tree.add_reverse_ngram(&vec![1, 2, 2, 4]);
    // tree.add_reverse_ngram(&vec![1, 3, 2, 4]);

    // if let Some(node) = tree.query_reverse_ngram(&vec![1, 2]) {
    //     println!("{}", node.count);
    // }

    // println!("{}", tree.to_string());

    if args.len() > 1 && args[1] == "read" {
        println!("Reading tree from file");
        let data = fs::read("bin/tree.rst").unwrap();
        if let Ok(read_node) = deserialize(data) {
            tree = read_node;
        } else {
            println!("Unable to read tree from file")
        }
    } else {
        println!("Compiling tree");
        let contents = fs::read_to_string("bin/data/cleaned_tweets.txt").unwrap();
        let mut index = 0;
        for tweet in contents.replace('\r', "").split("\n") {
            for i in 0..tweet.len() + CONTEXT {
                tree.add_reverse_ngram(&str_to_reverse_ngram(unicode_str_subsection(
                    tweet.to_string(),
                    cmp::max(i as isize - CONTEXT as isize, 0) as usize,
                    cmp::min(i, tweet.len()),
                )));
            }
            index += 1;
            if index == 10000 {
                break;
            }
        }

        let mut file = fs::OpenOptions::new()
            .write(true)
            .open("bin/tree.rst")
            .unwrap();
        let _ = file.write_all(&tree.serialize());
    }
    println!("Generating Text");
    println!("{}", generate_text("t".to_string(), 280, tree, CONTEXT));
}
