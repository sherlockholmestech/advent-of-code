use core::num;
use std::fs;

fn main() {
    let input = fs::read_to_string("input.txt").unwrap();
    let mut input_vec: Vec<Vec<char>> = Default::default();
    let mut sum = 0;
    //format input
    for (index, line) in input.trim().lines().enumerate() {
        input_vec.push(vec![]);
        for char in line.chars() {
            input_vec[index].push(char);
        }
    }
    // real shit
    for (y, line) in input_vec.iter().enumerate() {
        for (x, char) in line.iter().enumerate() {
            if char != &".".chars().next().unwrap() && !char.is_digit(10){
                println!("Character: {} Position: {}, {}", char, x, y);
                for result in neighbours(input_vec.to_vec(), x, y).iter() {
                    sum += result;
                }
            }
        }
    }
    println!("Sum: {}", sum);
}

fn neighbours(mut vector: Vec<Vec<char>>, x: usize, y: usize) -> Vec<i32> {
    let mut numbers: Vec<i32> = Vec::new();
    // check left
    if vector[y][x - 1].is_digit(10) {
        let mut x_pointer = x - 1;
        println!("Current x_pointer: {}", x_pointer);
        while if x_pointer >= 1 {vector[y][x_pointer - 1].is_digit(10)} else {false} {
            x_pointer -= 1;
        }
        println!("x_pointer: {}", x_pointer);
        let mut num_string = String::new();
        while if x_pointer <= vector[y].len() - 1 {vector[y][x_pointer].is_digit(10)} else {false} {
            num_string.push(vector[y][x_pointer]);
            vector[y][x_pointer] = ".".chars().next().unwrap();
            x_pointer += 1;
        }
        println!("Number: {}", num_string);
        numbers.push(num_string.parse::<i32>().unwrap());
    }
    // check right
    if vector[y][x + 1].is_digit(10) {
        let mut x_pointer = x + 1;
        println!("Current x_pointer: {}", x_pointer);
        while if x_pointer >= 1 {vector[y][x_pointer - 1].is_digit(10)} else {false} {
            x_pointer -= 1;
        }
        println!("x_pointer: {}", x_pointer);
        let mut num_string = String::new();
        while if x_pointer <= vector[y].len() - 1 {vector[y][x_pointer].is_digit(10)} else {false} {
            num_string.push(vector[y][x_pointer]);
            vector[y][x_pointer] = ".".chars().next().unwrap();
            x_pointer += 1;
        }
        println!("Number: {}", num_string);
        numbers.push(num_string.parse::<i32>().unwrap());
    }
    // check down
    if y <= vector.len() - 1 {
        if vector[y + 1][x].is_digit(10) {
            let mut x_pointer = x;
            println!("Current x_pointer: {}", x_pointer);
            while if x_pointer >= 1 {vector[y + 1][x_pointer - 1].is_digit(10)} else {false} {
                x_pointer -= 1;
            }
            println!("x_pointer: {}", x_pointer);
            let mut num_string = String::new();
            while if x_pointer <= vector[y+1].len() - 1 {vector[y + 1][x_pointer].is_digit(10)} else {false} {
                num_string.push(vector[y + 1][x_pointer]);
                vector[y + 1][x_pointer] = ".".chars().next().unwrap();
                x_pointer += 1;
            }
            println!("Number: {}", num_string);
            numbers.push(num_string.parse::<i32>().unwrap());
        }
    }
    // check up
    if y >= 1 {
        if vector[y - 1][x].is_digit(10) {
            let mut x_pointer = x;
            println!("Current x_pointer: {}", x_pointer);
            while if x_pointer >= 1 {vector[y - 1][x_pointer - 1].is_digit(10)} else {false} {
                x_pointer -= 1;
            }
            println!("x_pointer: {}", x_pointer);
            let mut num_string = String::new();
            while if x_pointer <= vector[y-1].len() - 1 {vector[y - 1][x_pointer].is_digit(10)} else {false} {
                num_string.push(vector[y - 1][x_pointer]);
                vector[y - 1][x_pointer] = ".".chars().next().unwrap();
                x_pointer += 1;
            }
            println!("Number: {}", num_string);
            numbers.push(num_string.parse::<i32>().unwrap());
        }
    }
    // check up left
    if y >= 1 {
        if vector[y - 1][x - 1].is_digit(10) {
            let mut x_pointer = x - 1;
            println!("Current x_pointer: {}", x_pointer);
            while if x_pointer >= 1 {vector[y - 1][x_pointer - 1].is_digit(10)} else {false} {
                x_pointer -= 1;
            }
            println!("x_pointer: {}", x_pointer);
            let mut num_string = String::new();
            while if x_pointer <= vector[y-1].len() - 1 {vector[y - 1][x_pointer].is_digit(10)} else {false} {
                num_string.push(vector[y - 1][x_pointer]);
                vector[y - 1][x_pointer] = ".".chars().next().unwrap();
                x_pointer += 1;
            }
            println!("Number: {}", num_string);
            numbers.push(num_string.parse::<i32>().unwrap());
        }
    }
    // check up right
    if y >= 1 {
        if vector[y - 1][x + 1].is_digit(10) {
            let mut x_pointer = x + 1;
            println!("Current x_pointer: {}", x_pointer);
            while if x_pointer >= 1 {vector[y - 1][x_pointer - 1].is_digit(10)} else {false} {
                x_pointer -= 1;
            }
            println!("x_pointer: {}", x_pointer);
            let mut num_string = String::new();
            while if x_pointer <= vector[y-1].len() - 1 {vector[y - 1][x_pointer].is_digit(10)} else {false} {
                num_string.push(vector[y - 1][x_pointer]);
                vector[y - 1][x_pointer] = ".".chars().next().unwrap();
                x_pointer += 1;
            }
            println!("Number: {}", num_string);
            numbers.push(num_string.parse::<i32>().unwrap());
        }
    }
    // check down left
    if y <= vector.len() - 1 {
        if vector[y + 1][x - 1].is_digit(10) {
            let mut x_pointer = x - 1;
            println!("Current x_pointer: {}", x_pointer);
            while if x_pointer >= 1 {vector[y + 1][x_pointer - 1].is_digit(10)} else {false} {
                x_pointer -= 1;
            }
            println!("x_pointer: {}", x_pointer);
            let mut num_string = String::new();
            while if x_pointer <= vector[y+1].len() - 1 {vector[y + 1][x_pointer].is_digit(10)} else {false} {
                num_string.push(vector[y + 1][x_pointer]);
                vector[y + 1][x_pointer] = ".".chars().next().unwrap();
                x_pointer += 1;
            }
            println!("Number: {}", num_string);
            numbers.push(num_string.parse::<i32>().unwrap());
        }
    }
    // check down right
    if y <= vector.len() - 1 {
        if vector[y + 1][x + 1].is_digit(10) {
            let mut x_pointer = x + 1;
            println!("Current x_pointer: {}", x_pointer);
            while if x_pointer >= 1 {vector[y + 1][x_pointer - 1].is_digit(10)} else {false} {
                x_pointer -= 1;
            }
            println!("x_pointer: {}", x_pointer);
            let mut num_string = String::new();
            while if x_pointer <= vector[y+1].len() - 1 {vector[y + 1][x_pointer].is_digit(10)} else {false} {
                num_string.push(vector[y + 1][x_pointer]);
                vector[y + 1][x_pointer] = ".".chars().next().unwrap();
                x_pointer += 1;
            }
            println!("Number: {}", num_string);
            numbers.push(num_string.parse::<i32>().unwrap());
        }
    }
    return numbers;
}