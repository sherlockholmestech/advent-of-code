use std::fs;
fn main() {
    let input = fs::read_to_string("input.txt").unwrap();
    let alphabet_array: [char; 52] = ['a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm', 'n', 'o', 'p', 'q', 'r', 's', 't', 'u', 'v', 'w', 'x', 'y', 'z', 'A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'I', 'J', 'K', 'L', 'M', 'N', 'O', 'P', 'Q', 'R', 'S', 'T', 'U', 'V', 'W', 'X', 'Y', 'Z'];
    let mut total_priority: i32 = 0;
    for lines in input.trim().lines() {
        let compartment_len = lines.len() / 2;
        for i in alphabet_array.iter() {
            if lines[0..compartment_len].contains(&i.to_string()) && lines[compartment_len..lines.len()].contains(&i.to_string()) {
                let mut index = alphabet_array.iter().position(|&x| x == *i).unwrap();
                index += 1;
                total_priority += index.to_string().trim().parse::<i32>().unwrap();
            }
        }
    }

    println!("The answer is {}", total_priority);
}
