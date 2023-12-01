use std::{fs, vec};
use pathfinding::prelude::Grid;

fn main() {
    let input = fs::read_to_string("input.txt").unwrap();
    let alphabet_array: [char; 26] = ['a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm', 'n', 'o', 'p', 'q', 'r', 's', 't', 'u', 'v', 'w', 'x', 'y', 'z'];
    let mut input_vec: Vec<Vec<char>> = Vec::new();
    let mut input_height: Vec<Vec<i32>> = Vec::new();
    let mut able_to_see_cords: Vec<(i32 ,i32)> = Vec::new();
    for line in input.lines() {
        let mut vector: Vec<char> = Vec::new();
        for character in line.chars() {
            vector.push(character);
        }
        input_vec.push(vector);
    }
    let grid = Grid::new(input_vec[0].len(), input_vec.len());
    for (row_num, row) in input_vec.iter().enumerate() {
        let mut height_vec: Vec<i32> = Vec::new();
        for (item_num, item) in row.iter().enumerate() {
            let mut current_level = 0;
            for (alphabet_num, alphabet) in alphabet_array.iter().enumerate() {
                if item == alphabet {
                    current_level = alphabet_num;
                } else if item == &'S' {
                    current_level = 0;
                } else if item == &'E' {
                    current_level = 0;
                }
            }
            height_vec.push(current_level as i32);
            
        }
        input_height.push(height_vec);
    }
    println!("{:#?}", input_height);
}