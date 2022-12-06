use std::collections::{HashMap, HashSet};
use std::fs;

fn create_item_priority_scores() -> HashMap<char, i32> {
    let item_priorities = ('a'..='z').chain('A'..='Z');

    HashMap::from_iter(item_priorities.enumerate().map(|(index, item)| {
        let score: i32 = i32::try_from(index).unwrap() + 1;
        (item, score)
    }))
}

fn get_shared_item(items: &[&str]) -> Option<char> {
    let sets = items
        .iter()
        .map(|item| item.chars().collect::<HashSet<_>>());
    let overlaps = sets.reduce(|set1, set2| set1.intersection(&set2).copied().collect());

    match overlaps {
        Some(o) => Some(*o.iter().next().unwrap()),
        None => None,
    }
}

fn split_line(line: &str) -> (&str, &str) {
    let line_length = line.len();

    line.split_at(line_length / 2)
}

fn calculate_part_one(lines: &std::str::Lines, item_scores: &HashMap<char, i32>) -> i32 {
    lines
        .clone()
        .map(split_line)
        .map(|(left, right)| item_scores[&get_shared_item(&vec![left, right]).unwrap()])
        .sum()
}

fn calculate_part_two(lines: &std::str::Lines, item_scores: &HashMap<char, i32>) -> i32 {
    let mut i = 0;
    let mut badges: Vec<char> = Vec::new();

    let inputs = lines.clone().map(|line| line.trim()).collect::<Vec<_>>();

    while i < inputs.len() {
        let group_lines = &inputs[i..i + 3];
        let group_badge = get_shared_item(group_lines);

        badges.push(group_badge.unwrap());

        i += 3;
    }

    badges.iter().map(|badge| item_scores[&badge]).sum()
}

fn main() {
    let input_data = fs::read_to_string("data/input.txt").unwrap();
    let lines = input_data.lines();

    let item_scores = create_item_priority_scores();

    let score_part_1 = calculate_part_one(&lines, &item_scores);
    let score_part_2 = calculate_part_two(&lines, &item_scores);

    println!("Score part 1: {}", score_part_1);
    println!("Score part 2: {}", score_part_2);
}

#[cfg(test)]
mod tests {
    use crate::{create_item_priority_scores, get_shared_item};

    use super::split_line;

    #[test]
    fn split_line_returns_two_halves() {
        let line = "aabbccdd";
        let (left, right) = split_line(line);

        assert_eq!(left, "aabb");
        assert_eq!(right, "ccdd");
    }

    #[test]
    fn get_shared_item_returns_the_right_item() {
        let left = "vJrwpWtwJgWr";
        let right = "hcsFMMfFFhFp";

        let shared_item = get_shared_item(&vec![left, right]).unwrap();

        assert_eq!(shared_item, 'p');
    }

    #[test]
    fn score_items_return_the_right_score() {
        let scores = create_item_priority_scores();

        let score_a = scores[&'a'];
        let score_b = scores[&'A'];

        assert_eq!(score_a, 1);
        assert_eq!(score_b, 27);
    }

    #[test]
    fn create_item_priority_scores_returns_the_right_items() {
        let scores = create_item_priority_scores();

        assert_eq!(scores.keys().len(), 52);
    }
}
