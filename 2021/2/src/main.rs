use std::fs;

fn main() {
    let input = fs::read_to_string("input.txt").expect("expected input file");
    let mut horizontal = 0;
    let mut vertical = 0;
    let mut current_num = 0;
    let mut ans = 0;
    for line in input.lines() {
        if line.contains("forward") {
            current_num = line[8..9].trim().parse().expect("Expected an interger");
            horizontal += current_num;
        } else if line.contains("down") {
            current_num = line[5..6].trim().parse().expect("Expected an interger");
            vertical += current_num;
        } else if line.contains("up") {
            current_num = line[3..4].trim().parse().expect("Expected an interger");
            vertical -= current_num;
        }
    }
    ans = horizontal * vertical;
    println!("{ans}");
}
