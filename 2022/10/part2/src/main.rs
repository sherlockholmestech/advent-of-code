use std::{fs, collections::HashMap};

fn main() {
    let input = fs::read_to_string("input.txt").unwrap();
    let mut cycles = 0;
    let mut x = 1;
    let mut ans = String::new();
    let cycle_index: [i32; 6] = [20, 60, 100, 140, 180, 220];
    let mut draw: [[char; 40]; 6] = [['.'; 40]; 6];
    for line in input.lines() {
       if line.contains("addx") {
            let split = line.split(" ");
            let mod_x: i32 = split.last().unwrap().trim().parse().unwrap();
            cycles += 1;
            draw_pixel(&mut draw, cycles, x);
            cycles += 1;
            draw_pixel(&mut draw, cycles, x);
            x += mod_x;
        } else {
            cycles += 1;
            draw_pixel(&mut draw, cycles, x);
        }
    }
    for line in draw.iter() {
        for char in line.iter() {
            ans.push(*char);
        }
        ans.push_str("\n");
    }

    println!("{}", ans);
}

fn draw_pixel(draw: &mut [[char; 40]; 6] , cycles: i32, x: i32) {
    let crt_pos = (cycles - 1) % 40;
    let crt_line = (cycles - 1) / 40;
    if (crt_pos - x).abs() <= 1 {
        draw[crt_line as usize][crt_pos as usize] = '#';
    } else {
        draw[crt_line as usize][crt_pos as usize] = '.';
    }
}