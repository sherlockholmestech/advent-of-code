use std::fs;

const MAX_RED: i32 = 12;
const MAX_GREEN: i32 = 13;
const MAX_BLUE: i32 = 14;
fn main() {
    let input = fs::read_to_string("input.txt").unwrap();
    let mut sum = 0;
    for line in input.lines() {
        let game_number = line.split(":").next().unwrap().split(" ").last().unwrap().parse::<i32>().unwrap();
        let mut within_limit = true;
        for round in line.split(":").last().unwrap().split(';') {
            println!("Round: {}", round);
            for cubes_shown in round.split(",") {
                let cube_num = cubes_shown.trim().split(" ").next().unwrap().parse::<i32>().unwrap();
                let cube_shown = cubes_shown.trim().split(" ").last().unwrap();
                match cube_shown.trim() {
                    "red" => {
                        if cube_num > MAX_RED {
                            within_limit = false;
                        }
                    },
                    "blue" => {
                        if cube_num > MAX_BLUE {
                            within_limit = false;
                        }
                    },
                    "green" => {
                        if cube_num > MAX_GREEN {
                            within_limit = false;
                        }
                    },
                    &_ => {
                        todo!();
                    }
                }
                println!("Cube Number: {}, Cube Type: {}, Within Limit? {}", cube_num, cube_shown, within_limit);
            }
        }
        if within_limit {
            sum += game_number;
        }
    }
    println!("Sum: {}", sum);
}
