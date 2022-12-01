use std::{fs};

fn main() {
    let input = fs::read_to_string("input.txt").expect("Expected input");
    let mut result: Vec<i64> = Vec::new();
    let mut current: i64 = 0;
    for line in input.lines() {
        if line.is_empty() {
            result.push(current);
            current = 0;
        } else {
            current += line.trim().parse::<i64>().unwrap();
        }
    }
    result.sort();
    println!("The answer is {}", result.last().unwrap() + result.get(result.len() - 2).unwrap() + result.get(result.len() - 3).unwrap());
}
