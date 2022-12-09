use std::fs;

fn parse_input() -> (Vec<Vec<i32>>, usize, usize) {
    let input_data = fs::read_to_string("data/input.txt").expect("Can't find the input file");

    let grid: Vec<Vec<i32>> = input_data
        .lines()
        .map(|line| line.chars().map(|c| c as i32 - 0x30).collect())
        .collect();

    let width = grid.first().unwrap().len();
    let height = grid.len();

    (grid, width, height)
}

fn main() {
    let (grid, width, height) = parse_input();
}
