use std::{fs, process::exit};
use itertools::Itertools;

fn main() {
    let input = fs::read_to_string("input.txt").unwrap();
    let mut start: usize = 0;
    let mut end: usize = 14;
    for i in 1..input.len() - 12 {
        let original_group = input[start..end].chars().collect::<Vec<char>>();
        let original_length = original_group.len();
        let filtered_group = original_group.into_iter().unique().collect::<Vec<char>>();
        if filtered_group.len() == original_length {
            println!("{}", i + 13);
            exit(0);
        }
        start += 1;
        end += 1;
    }
}