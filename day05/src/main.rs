use regex::Regex;
use std::fs;

struct Move {
    amount: i32,
    from: i32,
    to: i32,
}

impl Move {
    fn new(amount: i32, from: i32, to: i32) -> Move {
        Move { amount, from, to }
    }
}

fn define_state() -> Vec<Vec<&'static str>> {
    /*
     [N]             [R]             [C]
     [T] [J]         [S] [J]         [N]
     [B] [Z]     [H] [M] [Z]         [D]
     [S] [P]     [G] [L] [H] [Z]     [T]
     [Q] [D]     [F] [D] [V] [L] [S] [M]
     [H] [F] [V] [J] [C] [W] [P] [W] [L]
     [G] [S] [H] [Z] [Z] [T] [F] [V] [H]
     [R] [H] [Z] [M] [T] [M] [T] [Q] [W]
      1   2   3   4   5   6   7   8   9
    */

    vec![
        vec!["R", "G", "H", "Q", "S", "B", "T", "N"],
        vec!["H", "S", "F", "D", "P", "Z", "J"],
        vec!["Z", "H", "V"],
        vec!["M", "Z", "J", "F", "G", "H"],
        vec!["T", "Z", "C", "D", "L", "M", "S", "R"],
        vec!["M", "T", "W", "V", "H", "Z", "J"],
        vec!["T", "F", "P", "F", "L", "Z"],
        vec!["Q", "V", "W", "S"],
        vec!["W", "H", "L", "M", "T", "D", "N", "C"],
    ]
}

fn parse_moves() -> Vec<Move> {
    let move_pattern = Regex::new(r"^move (\d+) from (\d+) to (\d+)$").unwrap();
    let input_data = fs::read_to_string("data/input.txt").unwrap();
    let input_lines = input_data.lines();

    input_lines
        .map(|line| {
            let captures = move_pattern.captures(line).unwrap();
            Move {
                amount: captures[1].parse().unwrap(),
                from: captures[2].parse().unwrap(),
                to: captures[3].parse().unwrap(),
            }
        })
        .collect()
}

fn process_crates_9000(state: &mut Vec<Vec<&str>>, moves: &Vec<Move>) {
    for mv in moves {
        let mut moves_to_make = mv.amount;

        while moves_to_make > 0 {
            let from_stack = &mut state[(mv.from - 1) as usize];

            let moved_item = from_stack.pop().unwrap();

            let to_stack = &mut state[(mv.to - 1) as usize];
            to_stack.push(moved_item);

            moves_to_make -= 1;
        }
    }
}

fn process_crates_9001(state: &mut Vec<Vec<&str>>, moves: &Vec<Move>) {
    for mv in moves {
        let from_stack = &mut state[(mv.from - 1) as usize];
        let moved_items = from_stack.split_off(from_stack.len() - mv.amount as usize);

        let to_stack = &mut state[(mv.to - 1) as usize];
        to_stack.append(&mut moved_items.clone());
    }
}

fn print_code(state_name: &str, state: &Vec<Vec<&str>>) {
    let codes = state
        .iter()
        .map(|stack| stack[stack.len() - 1])
        .collect::<Vec<_>>()
        .join("\0");

    println!("Code for {}: {}", state_name, codes);
}

fn main() {
    let moves = parse_moves();
    let mut shipping_state_9000 = define_state();
    let mut shipping_state_9001 = define_state();

    process_crates_9000(&mut shipping_state_9000, &moves);
    process_crates_9001(&mut shipping_state_9001, &moves);

    print_code("Crate mover 9000", &shipping_state_9000);
    print_code("Crate mover 9001", &shipping_state_9001);
}

#[cfg(test)]
mod tests {
    use super::Move;
    use super::{process_crates_9000, process_crates_9001};

    fn is_state_equal(left: Vec<Vec<&str>>, right: Vec<Vec<&str>>) -> bool {
        for (left_stack, right_stack) in left.iter().zip(right.iter()) {
            if left_stack != right_stack {
                return false;
            }
        }

        true
    }

    #[test]
    fn process_crates_9000_works_correctly() {
        let mut state = vec![vec!["Z", "N"], vec!["M", "C", "D"], vec!["P"]];
        let moves = vec![Move::new(2, 2, 1)];

        let expected_state = vec![vec!["Z", "N", "D", "C"], vec!["M"], vec!["P"]];

        process_crates_9000(&mut state, &moves);

        assert!(is_state_equal(expected_state, state));
    }

    #[test]
    fn process_crates_9001_works_correctly() {
        let mut state = vec![vec!["Z", "N"], vec!["M", "C", "D"], vec!["P"]];
        let moves = vec![Move::new(2, 2, 1)];

        let expected_state = vec![vec!["Z", "N", "C", "D"], vec!["M"], vec!["P"]];

        process_crates_9001(&mut state, &moves);

        assert!(is_state_equal(expected_state, state));
    }
}
