use std::fs;

fn main() {
    let input = fs::read_to_string("input.txt").unwrap();
    let mut points = 0;
    for card in input.lines() {
        let number_section = card.split(":").last().unwrap();
        let mut winning: Vec<&str> = number_section.split("|").next().unwrap().trim().split(" ").map(|x| x.trim()).collect();
        let mut current_numbers: Vec<&str> = number_section.split("|").last().unwrap().trim().split(" ").map(|x| x.trim()).collect();
        winning.retain(|x| !x.is_empty());
        current_numbers.retain(|x| !x.is_empty());
        println!("Winning Numbers: {:?}, Current Numbers: {:?}", winning, current_numbers);
        let mut win_sum =0;
        for number in current_numbers {
            if winning.contains(&number) {
                win_sum += 1;
            }
        }
        points += 1 * (if win_sum == 0 {0} else {2_i32.pow(win_sum - 1)});
        println!("Points as of now: {}", points);
    }
    println!("Points: {}", points);
}
