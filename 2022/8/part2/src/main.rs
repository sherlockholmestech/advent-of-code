use std::fs;

use grid::Grid;
fn main() {
    let input = fs::read_to_string("input.txt").expect("Expected input");
    let first_line = input.lines().next().unwrap();
    let x_max = first_line.chars().count();
    let y_max = input.lines().count();
    let mut grid: Grid<i32> = Grid::new(0, 0);
    let mut current_x = 0;
    let mut current_y: usize = 0;
    let mut score_vec: Vec<i32> = Vec::new();
    for line in input.lines() {
        let row: Vec<i32> = line.chars().map(|x| x.to_string().parse::<i32>().unwrap()).collect::<Vec<i32>>();
        grid.push_row(row);
    }
    for (index, item) in grid.iter().enumerate() {
        current_x = index % x_max;
        current_y = index / y_max;
        score_vec.push(calculate_score(current_x, current_y, &grid));
    }
    score_vec.sort();
    let ans = score_vec.last().unwrap();
    println!("{}", ans);
}

fn calculate_score(x: usize, y: usize, grid: &Grid<i32>) -> i32 {
    let mut visible = false;
    //up down left right
    let mut score = (0, 0, 0, 0);
    //up down left right order of tuples
    let mut direction_visible = (true, true, true, true);
    if x == 0 || y == 0 || x == grid.cols() - 1 || y == grid.rows() {
        return 0;
    }
    //check up
    for row in (0..y).rev() {
        score.0 += 1;
        if grid.get(row, x) >= grid.get(y, x){
            direction_visible.0 = false;
            break;
        }
    }
    //check down
    for row in y + 1..grid.rows() {
        score.1 += 1;
        if grid.get(row, x) >= grid.get(y, x) {
            direction_visible.1 = false;
            break;
        }
    }
    //check left
    for col in (0..x).rev() {
        score.2 += 1;
        if grid.get(y, col) >= grid.get(y, x) {
            direction_visible.2 = false;
            break;
        }
    }
    //check down
    for col in x + 1..grid.cols() {
        score.3 += 1;
        if grid.get(y, col) >= grid.get(y, x) {
            direction_visible.3 = false;
            break;
        }
    }

    if direction_visible.0 || direction_visible.1 || direction_visible.2 || direction_visible.3 {
        visible = true;
    }
    let return_value = score.0 * score.1 * score.2 * score.3;
    return return_value;
}