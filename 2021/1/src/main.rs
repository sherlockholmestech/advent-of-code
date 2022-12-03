use std::fs;
fn main() {
    let input = fs::read_to_string("input.txt").expect("Unable to read input");
    let mut numbers: [i32; 2000] = [0; 2000];
    let mut index = 0;
    let mut previous_index = 0;
    let mut ans = 1;
    for line in input.lines() {
        numbers[index] = line.trim().parse().expect("expected a number");
        index += 1;
    }
    for i in 1..1999 {
        if numbers[i] > numbers[previous_index] {
            ans += 1;
            previous_index += 1;
        } else {
            previous_index += 1;
        }
    }
    println!("The answer is {}", ans);
}
