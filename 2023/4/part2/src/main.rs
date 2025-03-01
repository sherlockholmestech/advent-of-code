use std::fs;

fn main() {
    let input = fs::read_to_string("input.txt").unwrap();
    let mut cards_to_wins: Vec<i32> = vec![0; input.lines().count()];
    for  card in input.lines() {
        let current_card_no = card.split(":").next().unwrap().split(" ").last().unwrap().parse::<i32>().unwrap();
        let number_section = card.split(":").last().unwrap();
        let mut winning: Vec<&str> = number_section.split("|").next().unwrap().trim().split(" ").map(|x| x.trim()).collect();
        let mut current_numbers: Vec<&str> = number_section.split("|").last().unwrap().trim().split(" ").map(|x| x.trim()).collect();
        winning.retain(|x| !x.is_empty());
        current_numbers.retain(|x| !x.is_empty());
        // println!("Card: {}, Winning Numbers: {:?}, Current Numbers: {:?}", current_card_no, winning, current_numbers);
        for number in current_numbers {
            if winning.contains(&number) {
                cards_to_wins[(current_card_no - 1) as usize] += 1;
            }
        }
    }
    // println!("Cards to Wins: {:?}", cards_to_wins);
    let mut cards_to_number: Vec<i32> = vec![1; input.lines().count()];
    for (i, card_wins) in cards_to_wins.iter().enumerate() {
        if card_wins > &0 {
            for card in 1..=cards_to_number[i] {
                for count in 1..=*card_wins {
                    cards_to_number[i + count as usize] += 1;
                    // println!("Wins: {}", card_wins);
                    // println!("Cards to Number: {:?}", cards_to_number);
                }
            }
        }
    }
    // println!("Cards to Number: {:?}", cards_to_number);
    println!("Sum: {}", cards_to_number.iter().sum::<i32>());
}
