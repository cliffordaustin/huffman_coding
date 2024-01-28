use bitvec::vec::BitVec;

use crate::huffman_tree::HuffmanTree;
pub fn decoder<'a, T: Eq + Clone>(root: &'a HuffmanTree<T>, encoded_bits: &BitVec) -> Vec<&'a T> {
    let mut decoded_text = Vec::new();
    let mut stack = vec![root];

    let mut counter = 0;

    let mut message_size = 0;

    for bit in encoded_bits {
        let node = stack.pop().unwrap();

        match node {
            HuffmanTree::Leaf { token, .. } => {
                decoded_text.push(token);
            }
            HuffmanTree::Internal { left, right, .. } => {
                if bit == true {
                    match &**right {
                        HuffmanTree::Leaf { token, frequency } => {
                            decoded_text.push(token);
                            stack.push(root);
                            message_size += (counter + 1) * frequency;
                            counter = 0;
                        }
                        HuffmanTree::Internal { .. } => {
                            stack.push(right);
                            counter += 1;
                        }
                    }
                } else {
                    match &**left {
                        HuffmanTree::Leaf { token, frequency } => {
                            decoded_text.push(token);
                            stack.push(root);
                            message_size += (counter + 1) * frequency;
                            counter = 0;
                        }
                        HuffmanTree::Internal { .. } => {
                            stack.push(left);
                            counter += 1;
                        }
                    }
                }
            }
        }
    }

    println!("size of the message is {}", message_size);

    return decoded_text;
}
