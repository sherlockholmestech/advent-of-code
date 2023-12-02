use std::fs;
fn main() {
    let input = fs::read_to_string("input.txt").unwrap();
    let mut power_sum = 0;
    for line in input.lines() {
        let game_number = line.split(":").next().unwrap().split(" ").last().unwrap().parse::<i32>().unwrap();
        let mut required_cubes = [0;3];
        println!("Game: {}", game_number);
        for round in line.split(":").last().unwrap().split(';') {
            println!("Round: {}", round);
            for cubes_shown in round.split(",") {
                let cube_num = cubes_shown.trim().split(" ").next().unwrap().parse::<i32>().unwrap();
                let cube_shown = cubes_shown.trim().split(" ").last().unwrap();
                match cube_shown.trim() {
                    "red" => {
                        if cube_num > required_cubes[0] {
                            required_cubes[0] = cube_num;
                        }
                    },
                    "green" => {
                        if cube_num > required_cubes[1] {
                            required_cubes[1] = cube_num;
                        }
                    },
                    "blue" => {
                        if cube_num > required_cubes[2] {
                            required_cubes[2] = cube_num;
                        }
                    },
                    &_ => {
                        todo!();
                    }
                }
                println!("Cube Number: {}, Cube Type: {}, Current Minimum: {:?}", cube_num, cube_shown, required_cubes);
            }
        }
        power_sum += required_cubes[0] * required_cubes[1] * required_cubes[2];
        println!("Current power: {}, Current Power Sum: {}", required_cubes[0] * required_cubes[1] * required_cubes[2], power_sum)
    }
    println!("Sum: {}", power_sum);
}
