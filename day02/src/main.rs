use std::fs;

const OPPONENT_ROCK: &str = "A";
const OPPONENT_PAPER: &str = "B";
const OPPONENT_SCISSORS: &str = "C";

const MY_ROCK: &str = "X";
const MY_PAPER: &str = "Y";
const MY_SCISSORS: &str = "Z";

const LOSE: &str = "X";
const DRAW: &str = "Y";
const WIN: &str = "Z";

const SCORE_WIN: i32 = 6;
const SCORE_DRAW: i32 = 3;
const SCORE_LOSE: i32 = 0;

const SCORE_ROCK: i32 = 1;
const SCORE_PAPER: i32 = 2;
const SCORE_SCISSORS: i32 = 3;

fn calculate_score_cheating(instructions: &[(&str, &str)]) -> i32 {
    // Rock = 1
    // Paper = 2
    // Scissors = 3
    instructions
        .iter()
        .map(|instruction| match instruction {
            (OPPONENT_ROCK, LOSE) => SCORE_LOSE + SCORE_SCISSORS,
            (OPPONENT_ROCK, DRAW) => SCORE_DRAW + SCORE_ROCK,
            (OPPONENT_ROCK, WIN) => SCORE_WIN + SCORE_PAPER,
            (OPPONENT_PAPER, LOSE) => SCORE_LOSE + SCORE_ROCK,
            (OPPONENT_PAPER, DRAW) => SCORE_DRAW + SCORE_PAPER,
            (OPPONENT_PAPER, WIN) => SCORE_WIN + SCORE_SCISSORS,
            (OPPONENT_SCISSORS, LOSE) => SCORE_LOSE + SCORE_PAPER,
            (OPPONENT_SCISSORS, DRAW) => SCORE_DRAW + SCORE_SCISSORS,
            (OPPONENT_SCISSORS, WIN) => SCORE_WIN + SCORE_ROCK,
            (_, _) => panic!("Invalid combination"),
        })
        .sum()
}

fn calculate_score(instructions: &[(&str, &str)]) -> i32 {
    instructions
        .iter()
        .map(|instruction| match instruction {
            (OPPONENT_ROCK, MY_PAPER) => SCORE_WIN + SCORE_PAPER,
            (OPPONENT_ROCK, MY_ROCK) => SCORE_DRAW + SCORE_ROCK,
            (OPPONENT_ROCK, MY_SCISSORS) => SCORE_LOSE + SCORE_SCISSORS,
            (OPPONENT_PAPER, MY_ROCK) => SCORE_LOSE + SCORE_ROCK,
            (OPPONENT_PAPER, MY_SCISSORS) => SCORE_WIN + SCORE_SCISSORS,
            (OPPONENT_PAPER, MY_PAPER) => SCORE_WIN + SCORE_PAPER,
            (OPPONENT_SCISSORS, MY_SCISSORS) => SCORE_DRAW + SCORE_SCISSORS,
            (OPPONENT_SCISSORS, MY_PAPER) => SCORE_LOSE + SCORE_PAPER,
            (OPPONENT_SCISSORS, MY_ROCK) => SCORE_WIN + SCORE_ROCK,
            (_, _) => panic!("Invalid combination"),
        })
        .sum()
}

fn main() {
    let input_data = fs::read_to_string("data/input.txt").unwrap();
    let lines = input_data.lines();

    let instructions: Vec<(&str, &str)> = lines
        .map(|line| {
            let moves: Vec<&str> = line.split(' ').collect();
            (moves[0], moves[1])
        })
        .collect();

    let total_score: i32 = calculate_score(&instructions);
    let total_score_cheated: i32 = calculate_score_cheating(&instructions);

    println!("Total score: {}", total_score);
    println!("Total score cheated: {}", total_score_cheated);
}

#[cfg(test)]
mod tests {
    use crate::calculate_score_cheating;

    #[test]
    fn calculate_score_cheating_works() {
        let instructions = vec![("A", "Y"), ("B", "X"), ("C", "Z")];
        let score = calculate_score_cheating(&instructions);

        assert_eq!(score, 12);
    }
}

