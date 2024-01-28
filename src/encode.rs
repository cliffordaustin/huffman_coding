use std::{collections::HashMap, hash::Hash};

use bitvec::vec::BitVec;

use crate::huffman_tree::HuffmanTree;

pub fn encoder_bits<T: Eq + Clone + Hash>(
    word: &Vec<T>,
    encoded_data: HashMap<T, BitVec>,
) -> BitVec {
    let mut result = BitVec::new();

    for token in word {
        if let Some(bitvec) = encoded_data.get(&token) {
            result.extend_from_bitslice(bitvec);
        } else {
            // Handle the case where the token is not present in the encoded_data.
            // You might want to log an error, return a default value, or handle it as needed.
            eprintln!("Token not found in encoded_data");
        }
    }

    return result;
}

pub fn encoder<T: Eq + Clone + Hash>(tree: &HuffmanTree<T>, word: &Vec<T>) -> BitVec {
    let mut encoder = HashMap::new();

    let mut stack = vec![(tree, BitVec::new())];

    while !stack.is_empty() {
        let (node, path) = stack.pop().unwrap();

        match node {
            HuffmanTree::Leaf { token, .. } => {
                encoder.insert(token.clone(), path.clone());
            }

            HuffmanTree::Internal { left, right, .. } => {
                let mut left_path = path.clone();
                left_path.push(false);
                stack.push((left, left_path));

                let mut right_path = path.clone();
                right_path.push(true);
                stack.push((right, right_path));
            }
        }
    }

    let encoded_bits = encoder_bits(word, encoder);

    return encoded_bits;
}
