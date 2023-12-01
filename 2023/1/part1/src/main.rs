use std::fs;

fn main() {
    let input = fs::read_to_string("input.txt").unwrap();
    let mut sum = 0;
    for line in input.lines() {
        let mut number_string = String::new();
        let mut number = 0;
        for c in line.chars() {
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
