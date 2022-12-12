//WARNING!!! THIS CODE DOES NOT WORK

use std::fs;

#[derive(Debug)]
#[derive(PartialEq)]
#[derive(Clone, Copy)]
enum Operand {
    Plus,
    Minus,
    Multiply,
    Divide
}
#[derive(Debug)]
struct Monkey {
    num: i32,
    items: Vec<i32>,
    operand: Operand,
    operand_num: i32,
    test_num: i32,
    if_true: i32,
    if_false: i32,
    inspect_times: i32,
}
fn main() {
    let input = fs::read_to_string("input.txt").unwrap();
    let mut monkey_list: Vec<Monkey> = Vec::new();
    for monkey in input.split("\n\n") {
        let lines: Vec<&str> = monkey.lines().collect::<Vec<&str>>();
        let monkey_num: i32 = lines[0].split(" ").last().unwrap().replace(":", "").parse().unwrap();
        let items: Vec<i32> = lines[1].split(": ").last().unwrap().split(", ").map(|x| x.parse::<i32>().unwrap()).collect();
        let operand = return_operand(lines[2]);
        let operand_num: i32 = lines[2].split(" ").last().unwrap().parse().unwrap();
        let test_num: i32 = lines[3].split(" ").last().unwrap().parse().unwrap();
        let if_true: i32 = lines[4].split(" ").last().unwrap().parse().unwrap();
        let if_false: i32 = lines[5].split(" ").last().unwrap().parse().unwrap();
        let monkey = Monkey {
            num: monkey_num,
            items: items,
            operand: operand,
            operand_num: operand_num,
            test_num: test_num,
            if_true: if_true,
            if_false: if_false,
            inspect_times: 0,
        };
        monkey_list.push(monkey);
    }
    for _times in 1..=20 {
        for monkey_no in 0..=monkey_list.len() - 1 {
            for item_no in 0..=monkey_list[monkey_no].items.len() - 1 {
                println!("{}", monkey_list[monkey_no].items[item_no]);
            }
        }
    }
}

fn return_operand(operand_string: &str) -> Operand {
    if operand_string.contains("+") {
        return Operand::Plus;
    } else if operand_string.contains("-") {
        return Operand::Minus;
    } else if operand_string.contains("*") {
        return Operand::Multiply;
    } else {
        return Operand::Divide;
    }
}

fn calc_operation(operand: Operand, num: i32, old_worry: i32) -> i32 {
    let mut result = 0;
    if operand == Operand::Plus {
        result = old_worry + num;
        return result;
    } else if operand == Operand::Minus {
        result = old_worry - num;
        return result;
    } else if operand == Operand::Multiply {
        result = old_worry * num;
        return result;
    } else {
        result = old_worry / num;
        return result;
    }
}

fn remove_item(monkey_no: usize, monkey: &mut Monkey, i: usize) {
    monkey.items.remove(i);
}