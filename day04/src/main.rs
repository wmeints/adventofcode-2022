use std::fs;

struct SectionRange {
    start: i32,
    end: i32,
}

impl SectionRange {
    fn contains(&self, other: &SectionRange) -> bool {
        self.start <= other.start && self.end >= other.end
    }

    fn overlaps(&self, other: &SectionRange) -> bool {
        (self.start >= other.start && self.start <= other.end) ||
        (self.end >= other.start && self.end <= other.end)
    }
}


impl From<&str> for SectionRange {
    fn from(text: &str) -> Self {
        let mut parts = text.split('-');
        let start = parts.next().unwrap().parse().unwrap();
        let end = parts.next().unwrap().parse().unwrap();
        SectionRange { start, end }
    }
}


fn parse_input() -> Vec<(SectionRange, SectionRange)> {
    let input_data = fs::read_to_string("data/input.txt").unwrap();
    let input_lines = input_data.lines();

    input_lines.map(|line| {
        let mut split_data = line.split(',');
        (split_data.next().unwrap(), split_data.next().unwrap())
    }).map(|(left, right)| {
        (SectionRange::from(left), SectionRange::from(right))
    }).collect::<Vec<(SectionRange, SectionRange)>>()
}

fn get_contained_sections(sections: &Vec<(SectionRange, SectionRange)>) -> Vec<&(SectionRange, SectionRange)> {
    sections.iter().filter(|(left, right)| {
        left.contains(right) || right.contains(left)
    }).collect::<Vec<_>>()
}

fn get_overlapping_sections(sections: &Vec<(SectionRange, SectionRange)>) -> Vec<&(SectionRange, SectionRange)> {
    sections.iter().filter(|(left, right)| {
        left.overlaps(right) || right.overlaps(left)
    }).collect::<Vec<_>>()
}

fn main() {
    let input_data = parse_input();
    let contained_sections = get_contained_sections(&input_data);
    let overlapping_sections = get_overlapping_sections(&input_data);

    println!("Number of sections: {}", contained_sections.len());
    println!("Number of overlapping sections: {}", overlapping_sections.len());
}

#[cfg(test)]
mod tests{ 
    use super::SectionRange;

    #[test]
    fn contains_checks_correctly_for_contained_sections() {
        let left = SectionRange { start: 1, end: 10 };
        let right = SectionRange { start: 2, end: 9 };
        assert!(left.contains(&right));
    }

    #[test]
    fn contains_checks_correctly_for_non_contained_sections() {
        let left = SectionRange { start: 1, end: 10 };
        let right = SectionRange { start: 2, end: 11 };
        assert!(!left.contains(&right));
    }

    #[test]
    fn overlaps_checks_correctly_for_overlapping_sections() {
        let left = SectionRange { start: 1, end: 3 };
        let right = SectionRange { start: 2, end: 4 };
        assert!(left.overlaps(&right));
        assert!(right.overlaps(&left));
    }

    #[test]
    fn overlaps_checks_correctly_for_touching_sections() {
        let left = SectionRange { start: 1, end: 3 };
        let right = SectionRange { start: 3, end: 4 };
        assert!(left.overlaps(&right));
        assert!(right.overlaps(&left));
    }

    #[test]
    fn overlaps_checks_correctly_for_non_overlapping_sections() {
        let left = SectionRange { start: 1, end: 2 };
        let right = SectionRange { start: 3, end: 4 };
        assert!(!left.overlaps(&right));
        assert!(!right.overlaps(&left));
    }
}