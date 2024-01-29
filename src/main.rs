use std::{
    fs::{self, File},
    io::{self, Write},
};

use frequency_table::create_frequency_table;
use huffman_coding::{encode::encoder, huffman_tree::create_huffman_tree};

mod frequency_table;
mod huffman_tree;

fn main() -> io::Result<()> {
    let text = fs::read_to_string("sample_text.txt").expect("Can't get file");

    let lines = text.split(" ").map(|x| x).collect();

    let freq_table = create_frequency_table(&lines);

    let tree = create_huffman_tree(&freq_table);

    let encoded_bits = encoder(&tree, &lines);

    let data: Vec<u8> = rmp_serde::encode::to_vec(&encoded_bits).unwrap();

    let mut compressed_data = File::create("output.bin").expect("Couldn't create a file");
    compressed_data.write(&data)?;

    Ok(())
}
