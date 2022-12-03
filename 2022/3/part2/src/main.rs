use std::fs;
fn main() {
    let input = fs::read_to_string("input.txt").unwrap();
    let alphabet_array: [char; 52] = ['a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm', 'n', 'o', 'p', 'q', 'r', 's', 't', 'u', 'v', 'w', 'x', 'y', 'z', 'A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'I', 'J', 'K', 'L', 'M', 'N', 'O', 'P', 'Q', 'R', 'S', 'T', 'U', 'V', 'W', 'X', 'Y', 'Z'];
    let mut total_priority: i32 = 0;
    let mut group: Vec<String> = Vec::new();
    for lines in input.trim().lines() {
        group.push(lines.trim().to_string());
    }
    for chunk in group.chunks(3) {
        for i in alphabet_array.iter() {
            if chunk[0].trim().contains(*i) && chunk[1].trim().contains(*i) && chunk[2].trim().contains(*i) {
                let mut index = alphabet_array.iter().position(|&x| x == *i).unwrap();
                index += 1;
                total_priority += index.to_string().trim().parse::<i32>().unwrap();
            }
        }
    }
    println!("The answer is {}", total_priority);
}