use std::{fs};

fn main() {
    let input = fs::read_to_string("input.txt").unwrap();
    let mut num_to_move = 0;
    let mut from_to: Vec<i32> = Vec::new();
    let mut crates: [Vec<char>; 9] = [
        "FLMW".chars().rev().collect::<Vec<char>>(),
        "FMVZB".chars().rev().collect::<Vec<char>>(),
        "QLSRVH".chars().rev().collect::<Vec<char>>(),
        "JTMPQVSF".chars().rev().collect::<Vec<char>>(),
        "WSL".chars().rev().collect::<Vec<char>>(),
        "WJRMPVF".chars().rev().collect::<Vec<char>>(),
        "FRNPCQJ".chars().rev().collect::<Vec<char>>(),
        "BRWZSPHV".chars().rev().collect::<Vec<char>>(),
        "WZHGCJMB".chars().rev().collect::<Vec<char>>(),
    ];
    let mut answer = String::new();
    for part in input.split("/n/n") {
        for line in part.lines(){
            if line.contains("move") {
                for directions_text1 in line.trim().split("move ") {
                    if directions_text1.contains(" from ") {
                        for directions_text2 in directions_text1.trim().split(" from ") {
                            if directions_text2.contains(" to ") {
                                for directions_text3 in directions_text2.trim().split(" to ") {
                                    from_to.push(directions_text3.trim().parse().unwrap());
                                }
                            } else {
                                num_to_move = directions_text2.trim().parse().unwrap();
                            }
                        }
                    }
                }
            }
            
            for i in 1..=num_to_move {
                let from: usize = (from_to.get(0).unwrap() - 1).try_into().unwrap();
                let to: usize = (from_to.get(1).unwrap() - 1).try_into().unwrap();
                crates[to].push(*crates[from].last().unwrap());
                crates[from].truncate(crates[from].len() - 1);
            }

            num_to_move = 0;
            from_to.clear();
        }
    }

    for i in crates.iter() {
        answer.push(*i.last().unwrap());
    }

    println!("{}", answer);
}
