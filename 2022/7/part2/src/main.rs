use std::{fs, collections::HashMap, process::exit};
fn main() {
    let input = fs::read_to_string("input.txt").expect("Expected input");
    let mut current_dir: Vec<String> = Vec::new();
    let mut dir_lists : Vec<String> = Vec::new();
    let mut file_list: HashMap<String, i32> = HashMap::new();
    let mut current_sum = 0;
    let mut dir_size_list: Vec<i32> = Vec::new();
    let mut sum = 0;
    for line in input.lines() {
        let words = line.split(" ").collect::<Vec<&str>>();
        match (words[0], words[1]) {
            ("$", "ls") => {},
            ("$", "cd") => {
                match words[2] {
                    "/" => {
                        current_dir.push(words[2].to_string());
                        let mut dir_string = String::new();
                        for i in current_dir.iter() {
                            dir_string.push_str(i);
                        }
                        dir_lists.push(dir_string);
                    },
                    ".." => {
                        current_dir.remove(current_dir.len() - 1);
                    },
                    &_ => {
                        if current_dir.last().unwrap() == "/" {
                            let mut dir_push = String::new();
                            dir_push.push_str(words[2]);
                            current_dir.push(dir_push);
                            let mut dir_string = String::new();
                            for i in current_dir.iter() {
                                dir_string.push_str(i);
                            }
                            dir_lists.push(dir_string);
                        } else {
                            let mut dir_push = String::from("/");
                            dir_push.push_str(words[2]);
                            current_dir.push(dir_push);
                            let mut dir_string = String::new();
                            for i in current_dir.iter() {
                                dir_string.push_str(i);
                            }
                            dir_lists.push(dir_string);
                        }
                    }
                }
            },
            ("dir", _) => {
            },
            (&_, _) => {
                let mut dir_string = String::new();
                        for i in current_dir.iter() {
                            dir_string.push_str(i);
                        }
                dir_string.push_str(words[1]);
                file_list.insert(dir_string, words[0].parse::<i32>().unwrap());
            },
        }
    }
    for dir in dir_lists.iter() {
        for i in file_list.iter() {
            if i.0.contains(dir) {
                current_sum += i.1;
            }
        }
        dir_size_list.push(current_sum);
        current_sum = 0;
    }
    dir_size_list.sort();
    sum = *dir_size_list.last().unwrap();
    let required_space = 30000000 - (70000000 - sum);
    for i in dir_size_list.iter() {
        if i >= &required_space {
            println!("{}", i);
            exit(0);
        }
    }
}