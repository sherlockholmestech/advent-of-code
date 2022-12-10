use std::{fs};
use itertools::Itertools;
//ALl tuples regarding positions are in the format (x, y)
fn main() {
    let input = fs::read_to_string("input.txt").expect("Expected input");
    let mut head_position = (0, 0);
    let mut tail_position = (0, 0);
    let mut tail_pos_vec: Vec<(i32, i32)> = vec![(0, 0)];
    for line in input.lines() {
        let mut split = line.split(" ");
        let direction = split.next().unwrap();
        let num: i32 = split.next().unwrap().parse().unwrap();
        match direction {
            "L" => {
                for _i in 0..num {
                    head_position.0 -= 1;
                    update_tail(head_position, &mut tail_position);
                    tail_pos_vec.push(tail_position);
                }
            },
            "R" => {
                for _i in 0..num {
                    head_position.0 += 1;
                    update_tail(head_position, &mut tail_position);
                    tail_pos_vec.push(tail_position);
                }
            },
            "D" => {
                for _i in 0..num {
                    head_position.1 += 1;
                    update_tail(head_position, &mut tail_position);
                    tail_pos_vec.push(tail_position);
                }
            },
            "U" => {
                for _i in 0..num {
                    head_position.1 -= 1;
                    update_tail(head_position, &mut tail_position);
                    tail_pos_vec.push(tail_position);
                }
            },
            _ => {},
        }
    }
    let unique = tail_pos_vec.iter().unique();
    let ans = unique.into_iter().count();
    println!("{}", ans);
}

fn update_tail(head_position: (i32, i32), tail_position: &mut(i32, i32)) {
    if head_position.0 - tail_position.0 < -1 {
        tail_position.1 = head_position.1;
        tail_position.0 = head_position.0 + 1;
    } else if head_position.0 - tail_position.0 > 1 {
        tail_position.1 = head_position.1;
        tail_position.0 = head_position.0 - 1;
    } else if head_position.1 - tail_position. 1 < -1 {
        tail_position.0 = head_position.0;
        tail_position.1 = head_position.1 + 1;
    } else if head_position.1 - tail_position.1 > 1 {
        tail_position.0 = head_position.0;
        tail_position.1 = head_position.1 - 1;
    }
}