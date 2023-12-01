use std::fs;

fn main() {
    let input = fs::read_to_string("input.txt").unwrap();
    let possible_num_as_str = ["one", "two", "three", "four", "five", "six", "seven", "eight", "nine"];
    let mut sum = 0;
    for line in input.lines() {
        let mut number_string = String::new();
        let mut num_as_string = String::new();
        let mut number = 0;
        for c in line.chars() {
            if c.is_alphabetic() {
                num_as_string.push(c);
            }
            if check_is_string_number(&num_as_string, possible_num_as_str) != 0 {
                println!("Num as String and is Number: {}", num_as_string);
                number_string.push(char::from_digit(check_is_string_number(&num_as_string, possible_num_as_str), 10).unwrap());
                num_as_string = num_as_string[num_as_string.len() - 1 .. num_as_string.len()].to_string()
            }
            if c.is_digit(10) {
                number_string.push(c);
            }
        }
        if number_string.len() == 1 {
            number_string.push(number_string[0..1].parse::<char>().unwrap());
            number = number_string.parse::<i32>().unwrap()
        } else {
            let length = number_string.len();
            let last = number_string[length - 1..length].parse::<char>().unwrap();
            let mut extracted_num = String::from(&number_string[0..1]);
            extracted_num.push(last);
            number = extracted_num.parse::<i32>().unwrap()
        }
        println!("Number String: {} Parsed Number: {}", number_string, number);
        sum += number;
    }
    println!("{}", sum);
}

fn check_is_string_number(num_string: &str, list: [&str; 9]) -> u32 {
    for (index, item) in list.iter().enumerate() {
        if num_string.contains(*item) {
            return index as u32 + 1;
        }
    }
    return 0;
}