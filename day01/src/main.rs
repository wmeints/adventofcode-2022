use std::fs;

fn main() {
    let input_data =  fs::read_to_string("data/input.txt").unwrap();
    let lines = input_data.lines();

    let mut current_count = 0;
    let mut calory_count = Vec::new();

    for line in lines {
        if line.is_empty() {
            calory_count.push(current_count);
            current_count = 0;
        } else {
            let calory_value: i32 = line.parse().unwrap();
            current_count += calory_value;
        }
    }

    let max_calory_count = calory_count.iter().max().unwrap();

    println!("Calory count: {}", max_calory_count);

    calory_count.sort();
    calory_count.reverse();

    let top3_calory_count: i32 = calory_count.iter().take(3).sum();

    println!("Top 3 calory count: {}", top3_calory_count);
}
