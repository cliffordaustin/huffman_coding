#![allow(dead_code)]
use std::collections::HashMap;

pub fn create_frequency_table<T: Eq + Copy + std::hash::Hash>(data: &Vec<T>) -> HashMap<T, usize> {
    let mut table = HashMap::new();

    for &token in data {
        let count = table.entry(token).or_insert(0);
        *count += 1;
    }

    return table;
}
