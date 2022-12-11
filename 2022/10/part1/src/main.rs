use std::{fs, collections::HashMap};

fn main() {
    let input = fs::read_to_string("input.txt").unwrap();
    let mut cycles = 0;
    let mut x = 1;
    let cycle_index: [i32; 6] = [20, 60, 100, 140, 180, 220];
    let mut signal_strength: HashMap<i32, i32> = HashMap::new();
    for line in input.lines() {
       if line.contains("addx") {
            let split = line.split(" ");
            let mod_x: i32 = split.last().unwrap().trim().parse().unwrap();
            cycles += 1;
            signal_strength.insert(cycles, cycles * x);
            cycles += 1;
            signal_strength.insert(cycles, cycles * x);
            x += mod_x;
        } else {
            cycles += 1;
            signal_strength.insert(cycles, cycles * x);
        }
    }
    
    let sum: i32 = cycle_index.map(|x| signal_strength.get(&x).unwrap().clone()).iter().sum();
    println!("{}", sum);
}
