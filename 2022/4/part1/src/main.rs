use std::fs;

fn main() {
    let input = fs::read_to_string("input.txt").unwrap();
    let mut input_vector: Vec<i32> = Vec::new();
    for line in input.lines() {
        for group in line.trim().split(",") {
            for i in group.trim().split("-") {
                input_vector.push(i.trim().parse().unwrap());
            }
        }
    }
    let mut overlaps = 0;
    for i in input_vector.chunks(4) {
        let a1 = i[0];
        let a2 = i[1];
        let b1 = i[2];
        let b2 = i[3];
        if (a1 <= b1 && a2 >= b2) || (b1 <= a1 && b2 >= a2) {
            overlaps += 1;
        }
    }

    println!("The answer is {}", overlaps);
}
