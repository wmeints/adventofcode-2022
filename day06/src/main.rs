use std::fs;
use std::ops::{Bound, RangeBounds};

// Rust supports unicode string, so you'll need a special set of utilities to
// correctly map between character positions and byte positions. That's what the
// mumbo jumbo below is for.

pub trait StringUtils {
    fn substring(&self, start: usize, len: usize) -> &str;
    fn slice(&self, range: impl RangeBounds<usize>) -> &str;
}

impl StringUtils for str {
    fn substring(&self, start: usize, len: usize) -> &str {
        let mut char_pos = 0;
        let mut byte_start = 0;
        let mut it = self.chars();
        loop {
            if char_pos == start {
                break;
            }
            if let Some(c) = it.next() {
                char_pos += 1;
                byte_start += c.len_utf8();
            } else {
                break;
            }
        }
        char_pos = 0;
        let mut byte_end = byte_start;
        loop {
            if char_pos == len {
                break;
            }
            if let Some(c) = it.next() {
                char_pos += 1;
                byte_end += c.len_utf8();
            } else {
                break;
            }
        }
        &self[byte_start..byte_end]
    }
    fn slice(&self, range: impl RangeBounds<usize>) -> &str {
        let start = match range.start_bound() {
            Bound::Included(bound) | Bound::Excluded(bound) => *bound,
            Bound::Unbounded => 0,
        };
        let len = match range.end_bound() {
            Bound::Included(bound) => *bound + 1,
            Bound::Excluded(bound) => *bound,
            Bound::Unbounded => self.len(),
        } - start;
        self.substring(start, len)
    }
}

// This is where the puzzle specific code starts.

fn is_unique_sequence(sequence: &str) -> bool {
    let characters: Vec<char> = sequence.chars().collect();
    let mut i = 0;

    while i < characters.len() {
        let left = &characters[..i];

        if left.iter().find(|o| **o == characters[i]).is_some() {
            return false;
        }

        i += 1;
    }

    true
}

fn find_marker(input_data: &str, marker_size: usize) -> Option<usize> {
    let mut i = marker_size;

    while i < input_data.len() {
        let sequence = input_data.slice(i - marker_size..i);

        if is_unique_sequence(sequence) {
            return Some(i);
        }

        i += 1;
    }

    return None;
}

fn main() {
    let input_data = fs::read_to_string("data/input.txt").expect("Unable to read input file");
    let start_of_packet_marker = find_marker(input_data.as_str(), 4).expect("Unable to find marker");
    let start_of_message_marker = find_marker(input_data.as_str(), 14).expect("Unable to find marker");

    println!("Start of packet marker found at position {}", start_of_packet_marker);
    println!("Start of message marker found at position {}", start_of_message_marker);
}

#[cfg(test)]
mod tests {
    use super::{find_marker, is_unique_sequence};

    #[test]
    fn unique_sequence_correctly_detected() {
        assert!(is_unique_sequence(&"abcd"));
        assert!(!is_unique_sequence(&"cdee"));
    }

    #[test]
    fn find_marker_works_correctly() {
        let input_data = vec![
            ("bvwbjplbgvbhsrlpgdmjqwftvncz", 5),
            ("nppdvjthqldpwncqszvftbrmjlhg", 6),
        ];

        for (input_string, expected_marker) in input_data {
            let actual_marker = find_marker(input_string, 4).expect("Unable to find marker");
            assert_eq!(actual_marker, expected_marker);
        }
    }
}
