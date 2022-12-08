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
    let mut visible = 0;
    for line in input.lines() {
        let row: Vec<i32> = line.chars().map(|x| x.to_string().parse::<i32>().unwrap()).collect::<Vec<i32>>();
        grid.push_row(row);
    }
    for (index, item) in grid.iter().enumerate() {
        current_x = index % x_max;
        current_y = index / y_max;
        if is_visible(current_x, current_y, &grid) {
            visible += 1;
        }
    }

    println!("{}", visible);
}

fn is_visible(x: usize, y: usize, grid: &Grid<i32>) -> bool {
    let mut visible = false;
    //up down left right order of tuples
    let mut direction_visible = (true, true, true, true);
    if x == 0 || y == 0 || x == grid.cols() - 1 || y == grid.rows() {
        return true;
    }
    //check up
    for row in (0..y).rev() {
        if grid.get(row, x) >= grid.get(y, x){
            direction_visible.0 = false;
        }
    }
    //check down
    for row in y + 1..grid.rows() {
        if grid.get(row, x) >= grid.get(y, x) {
            direction_visible.1 = false;
        }
    }
    //check left
    for col in (0..x).rev() {
        if grid.get(y, col) >= grid.get(y, x) {
            direction_visible.2 = false;
        }
    }
    //check down
    for col in x + 1..grid.cols() {
        if grid.get(y, col) >= grid.get(y, x) {
            direction_visible.3 = false;
        }
    }

    if direction_visible.0 || direction_visible.1 || direction_visible.2 || direction_visible.3 {
        visible = true;
    }
    return visible;
}