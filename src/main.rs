use std::{
    fs::{self, File},
    io::{self, BufWriter, Read, Write},
    iter,
};

use frequency_table::create_frequency_table;
use huffman_coding::{decode::decoder, encode::encoder, huffman_tree::create_huffman_tree};

mod frequency_table;
mod huffman_tree;

fn main() -> io::Result<()> {
    let text = fs::read_to_string("sample_text.txt").expect("Can't get file");

    let lines = text.split(" ").map(|x| x).collect();

    let freq_table = create_frequency_table(&lines);

    let tree = create_huffman_tree(&freq_table);

    let encoded_bits = encoder(&tree, &lines);

    // let decoded_text = decoder(&tree, &encoded_bits);

    let data: Vec<u8> = rmp_serde::encode::to_vec(&encoded_bits).unwrap();

    let mut compressed_data = File::create("output.bin").expect("Couldn't create a file");
    compressed_data.write(&data)?;

    // write_encoded_data("output.bin", encoded_bits);

    // let mut decompressed_file = File::create("output.txt")?;
    // decompressed_file.write_all(normal_text.as_bytes())?;

    Ok(())
}
