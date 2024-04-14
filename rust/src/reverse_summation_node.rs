use std::collections::HashMap;

// These store n-gram data in reverse order to allow for easy querying of (n-x)-grams
pub struct ReverseSummationNode {
    // This is the ID of the token which is attached to the node
    pub id: u64,
    // This is the number of the time this node appears, or, how many times the n-gram which starts with the value in this node
    // and continues with the values in its parent appears
    pub count: u64,
    // These are the children of the node, it is in a hashmap to be easily queriable
    pub children: HashMap<u64, ReverseSummationNode>,
}

impl ReverseSummationNode {
    // This adds a reverse n-gram to the tree structure
    pub fn add_reverse_ngram(&mut self, reverse_ngram: &[u64]) {
        self.count += 1;
        if reverse_ngram.len() == 0 {
            return;
        }
        if let Some(child) = self.children.get_mut(&reverse_ngram[0]) {
            child.add_reverse_ngram(&reverse_ngram[1..]);
        } else {
            let mut new_child = ReverseSummationNode {
                id: reverse_ngram[0],
                count: 0,
                children: HashMap::new(),
            };
            new_child.add_reverse_ngram(&reverse_ngram[1..]);
            self.children.insert(reverse_ngram[0], new_child);
        }
    }

    // This querys the number of a (reverse) n-gram in the tree structure
    pub fn query_reverse_ngram(&self, reverse_ngram: &[u64]) -> Option<&ReverseSummationNode> {
        if reverse_ngram.len() == 0 {
            return Some(self);
        }
        if let Some(child) = self.children.get(&reverse_ngram[0]) {
            return child.query_reverse_ngram(&reverse_ngram[1..]);
        }
        None
    }

    // This allows the tree to be stored efficiently
    pub fn serialize(&self) -> Vec<u8> {
        let serial_data = &mut self.id.to_le_bytes().to_vec();
        serial_data.append(&mut self.count.to_le_bytes().to_vec());
        serial_data.append(&mut (self.children.keys().len() as u64).to_le_bytes().to_vec());
        for child in self.children.values() {
            serial_data.append(&mut child.serialize());
        }
        serial_data.to_vec()
    }

    pub fn _to_string(&self, indent: usize, last_child: bool, before_line: Vec<usize>) -> String {
        let mut out = String::new();
        let mut new_before_line = before_line.clone();
        // If this is the first node then no indentation
        if indent == 0 {
            out = format!("{} {}", self.id, self.count);
        } else {
            for i in 0..indent - 1 {
                if before_line.contains(&i) {
                    out.push_str("│");
                }
                out.push_str("\t");
            }
            out = format!(
                "{}{}───────{} {}",
                out,
                if last_child { "└" } else { "├" },
                self.id,
                self.count
            );

            if !last_child {
                new_before_line.push(indent - 1);
            }
        }
        for child in self.children.values().enumerate() {
            out += &format!(
                "\n{}",
                child.1._to_string(
                    indent + 1,
                    child.0 == self.children.keys().len() - 1,
                    new_before_line.clone(),
                )
            );
        }
        return out;
    }

    // This allows the node to be displayed (kinda) as a string
    pub fn to_string(&self) -> String {
        self._to_string(0, false, vec![])
    }
}

fn _deserialize(data: &[u8]) -> Result<(ReverseSummationNode, usize), u8> {
    // Check to see if the data is wrong
    if data.len() < 24 {
        return Err(data.len() as u8);
    }

    let mut index_out: usize = 24; // This is because the first 24 bytes are standard
    let mut out = ReverseSummationNode {
        id: u64::from_le_bytes(data[0..8].try_into().expect("Slice with incorrect length")),
        count: u64::from_le_bytes(data[8..16].try_into().expect("Slice with incorrect length")),
        children: HashMap::new(),
    };

    let child_count = u64::from_le_bytes(
        data[16..24]
            .try_into()
            .expect("Slice with incorrect length"),
    );
    for _ in 0..child_count {
        let deserialization_result = _deserialize(&data[index_out..]);
        if let Ok(data_tuple) = deserialization_result {
            out.children.insert(data_tuple.0.id, data_tuple.0);
            index_out += data_tuple.1;
        } else {
            return deserialization_result;
        }
    }

    return Ok((out, index_out));
}

// This is a wrapper for _deserialize which removes some other uneeded data
// The u8 in the error code is the number of bytes left in the vector when it reads too little data to deserialize
pub fn deserialize(data: Vec<u8>) -> Result<ReverseSummationNode, u8> {
    match _deserialize(&data) {
        Ok(tuple_data) => return Ok(tuple_data.0),
        Err(err_code) => return Err(err_code),
    }
}
