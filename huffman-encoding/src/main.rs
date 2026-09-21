use std::{collections::HashMap, fmt::Debug, hash, iter};

#[derive(Debug)]
struct Node {
    byte: Option<u8>,
    frequency: u32,
    left: Option<Box<Node>>,
    right: Option<Box<Node>>,
}
impl Node {
    pub fn new(
        byte: Option<u8>,
        frequency: u32,
        left: Option<Box<Node>>,
        right: Option<Box<Node>>,
    ) -> Self {
        Node {
            byte,
            frequency,
            left,
            right,
        }
    }
}
// Zeichen : (Länge des Codes, Code),
type CodeTable = HashMap<u8, (u8, u32)>;

fn build_codes(node: &Node, code: u32, code_len: u8, table: &mut CodeTable) {
    if let Some(byte) = node.byte {
        // Blatt: Speichern
        table.insert(byte, (code_len, code));
    } else {
        // Links = 0
        if let Some(ref left) = node.left {
            build_codes(left, (code << 1) | 0, code_len + 1, table);
        }
        // Rechts = 1
        if let Some(ref right) = node.right {
            build_codes(right, (code << 1) | 1, code_len + 1, table);
        }
    }
}
fn get_codes(nodes: Vec<Node>) -> CodeTable {
    let mut table = CodeTable::new();
    if nodes.is_empty() {
        return table;
    }

    let root = &nodes[0];
    build_codes(root, 0, 0, &mut table);
    table
}

fn huffman_encoding(data: &Vec<u8>) -> Vec<usize> {
    let mut encoded = Vec::new();

    let mut freq_map: HashMap<u8, u32> = HashMap::new();

    for byte in data {
        freq_map.insert(*byte, freq_map.get(byte).unwrap_or(&0) + 1);
    }

    let mut nodes = Vec::new();

    for (byte, freq) in freq_map {
        let node = Node::new(Some(byte), freq, None, None);
        nodes.push(node);
    }

    while nodes.len() > 1 {
        nodes.sort_by(|a, b| a.frequency.cmp(&b.frequency));
        let small1 = nodes.remove(0);
        let small2 = nodes.remove(0); // rückt ein nach links also einfach nochmal 0
        let new_node = Node::new(
            None,
            small1.frequency + small2.frequency,
            Some(Box::new(small1)),
            Some(Box::new(small2)),
        );
        nodes.push(new_node);
    }
    println!("{:?}", nodes);

    let coding_table = get_codes(nodes);

    println!("table: {:?}", coding_table);

    for byte in data {
        let (code_len, whole_code) = coding_table.get(byte).unwrap(); // muss da sein, sonst ist was schief gelaufen!

        let code: usize = get_last_n_bits(whole_code, code_len);
        encoded.push(code);
    }

    encoded
}
fn get_last_n_bits(value: &u32, n: &u8) -> usize {
    (value & ((1 << n) - 1)) as usize
}
fn huffman_decoding(data: &Vec<u8>) -> Vec<u8> {
    let decoded = Vec::new();

    decoded
}

fn main() {
    let data =
        "hallo welt, was geht. Fufu ist sehr cooler dieser kleine aal, waalomat hehehaha siuuuuuuu";

    let hfm = huffman_encoding(&data.into());

    println!("Huffman encoding: {:?}", hfm);
}
