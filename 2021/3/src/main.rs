use std::{fs};

fn main() {
    let input = fs::read_to_string("input.txt").expect("Expected input");
    let mut gamma = String::new();
    let mut epsilon = String::new();
    let mut gamma_decimal = 0;
    let mut epsilon_decimal = 0;
    let mut input_array: [i32; 12] = [0; 12];
    for line in input.lines() {
        for iterator in 1..13 {
            if line[iterator - 1 .. iterator].trim().parse::<i32>().unwrap() == 1 {
                input_array[iterator - 1] += 1;
            } else if line[iterator - 1 .. iterator].trim().parse::<i32>().unwrap() == 0 {
                input_array[iterator - 1] -= 1;
            }
        }
    }
    for i in input_array.iter() {
        if i < &0 {
            gamma.push_str("0");
            epsilon.push_str("1");
        } else if i > &0 {
            gamma.push_str("1");
            epsilon.push_str("0");
        }
    }
    gamma_decimal = isize::from_str_radix(&gamma, 2).unwrap();
    epsilon_decimal = isize::from_str_radix(&epsilon, 2).unwrap();
    println!("{}" ,gamma_decimal * epsilon_decimal);
}
