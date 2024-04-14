pub fn str_to_reverse_ngram(str: String) -> Vec<u64> {
    str.chars()
        .map(|c: char| -> u64 { u64::from(c).into() })
        .rev()
        .collect::<Vec<u64>>()
}

pub fn unicode_str_subsection(str: String, start: usize, end: usize) -> String {
    let mut c = str.chars();
    for _ in 0..start {
        c.next();
    }
    let mut out = String::new();
    for _ in start..end {
        if let Some(ch) = c.next() {
            out += &ch.to_string();
        }
    }
    return out;
}
