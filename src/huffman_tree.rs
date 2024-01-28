#![allow(dead_code, unused_variables)]

use std::{
    cmp::{Ordering, Reverse},
    collections::{BinaryHeap, HashMap},
};
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum HuffmanTree<T> {
    Internal {
        frequency: usize,
        left: Box<HuffmanTree<T>>,
        right: Box<HuffmanTree<T>>,
    },
    Leaf {
        token: T,
        frequency: usize,
    },
}

impl<T: Clone> HuffmanTree<T> {
    pub fn frequency(&self) -> usize {
        match self {
            HuffmanTree::Internal { frequency, .. } => *frequency,
            HuffmanTree::Leaf { frequency, .. } => *frequency,
        }
    }
}

impl<T: Clone + Eq> Ord for HuffmanTree<T> {
    fn cmp(&self, other: &Self) -> Ordering {
        self.frequency().cmp(&other.frequency())
    }
}

impl<T: Clone + Eq> PartialOrd for HuffmanTree<T> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

pub fn create_huffman_tree<T: Clone + Eq>(freq_table: &HashMap<T, usize>) -> HuffmanTree<T> {
    let mut heap = BinaryHeap::new();

    for (character, freq) in freq_table {
        heap.push(Reverse(HuffmanTree::Leaf {
            token: character.clone(),
            frequency: *freq,
        }));
    }

    while heap.len() > 1 {
        let (node1, node2) = (heap.pop().unwrap().0, heap.pop().unwrap().0);

        let merged_node = HuffmanTree::Internal {
            frequency: node1.frequency() + node2.frequency(),
            left: Box::new(node1),
            right: Box::new(node2),
        };

        heap.push(Reverse(merged_node));
    }

    return heap.pop().unwrap().0;
}
