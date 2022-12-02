use std::fs;

fn main() {
    let input = fs::read_to_string("input.txt").expect("Expected input");
    let mut opponent = 0;
    let mut you = 0;
    let mut score = 0;
    for line in input.lines() {
        for character in line.split(" ") {
            match character.trim() {
                "A" => opponent = 1,
                "B" => opponent = 2,
                "C" => opponent = 3,
                "X" => you = 1,
                "Y" => you = 2,
                "Z" => you = 3,
                &_ => todo!(),
            }
        }
        if opponent == 3 && you == 1 || opponent == 1 && you == 2 || opponent == 2 && you == 3 {
            score += 6;
            score += you;
        } else if opponent == you {
            score += 3;
            score += you;
        } else {
            score += you;
        }
    }
    println!("The answer to part 1 is {}", score);
}
