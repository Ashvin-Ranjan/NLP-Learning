use crate::{
    helpers::{str_to_reverse_ngram, unicode_str_subsection},
    reverse_summation_node::ReverseSummationNode,
};
use rand::Rng;

fn generate_next_character(text: &String, tree: &ReverseSummationNode, context: usize) -> char {
    let reverse_ngram = &str_to_reverse_ngram(unicode_str_subsection(
        text.clone(),
        if context > text.len() {
            0
        } else {
            text.len() - context
        },
        text.len(),
    ));
    let potential_node = tree.query_reverse_ngram(&reverse_ngram);
    if let Some(node) = potential_node {
        let mut rng = rand::thread_rng();
        let mut index = rng.gen_range(1..node.count + 1) as isize;
        for c in tree.children.values() {
            let potential_child = c.query_reverse_ngram(&reverse_ngram);
            if let Some(child) = potential_child {
                index -= child.count as isize;
                if index <= 0 {
                    return char::from_u32(c.id as u32).unwrap_or('〇');
                }
            }
        }
    }
    return generate_next_character(text, tree, context - 1);
}

pub fn generate_text(
    text: String,
    end: usize,
    tree: ReverseSummationNode,
    context: usize,
) -> String {
    let mut text_out = text.clone();
    for _ in 0..end {
        text_out += &generate_next_character(&text_out, &tree, context).to_string();
    }
    return text_out;
}
