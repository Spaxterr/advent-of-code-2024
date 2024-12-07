use std::fs::read_to_string;

fn main() {
    let numbers: Vec<Vec<i32>> = read_numbers("./input.txt");

    let safe_count = numbers.iter()
        .filter(|list| list.len() > 0)
        .filter(|list| is_safe(list))
        .count();

    let dampened_safe_count = numbers.iter()
        .filter(|list| list.len() > 0)
        .filter(|list| is_safe(list) || can_be_safe(list))
        .count();

    println!("Safe count: {safe_count}\nDampened safe count: {dampened_safe_count}");
}

fn can_be_safe(numbers: &Vec<i32>) -> bool {
    for i in 0..=numbers.len() - 1 {
        let mut dampened = numbers.clone();
        dampened.remove(i);
        if is_safe(&dampened) {
            return true;
        }
    }
    return false;
}

fn is_safe(numbers: &Vec<i32>) -> bool {
    if numbers.len() > 0 && has_consistent_direction(numbers) {
        let mut prev_number: Option<i32> = None;
        for &number in numbers {
            if let Some(num) = prev_number {
                let diff = number.abs_diff(num);
                if diff < 1 || diff > 3 {
                    return false;
                }
            }
            prev_number = Some(number);
        }
        return true
    } else {
        return false
    }
}

fn has_consistent_direction(numbers: &Vec<i32>) -> bool {
    return numbers.is_sorted() || numbers.is_sorted_by(|a, b| a >= b);
}

fn read_numbers(file_path: &str) -> Vec<Vec<i32>> {
    let numbers: Vec<Vec<i32>> = read_to_string(file_path)
        .expect("Could read file")
        .split("\n")
        .map(|line: &str| {
            return line.split_whitespace().map(|number| number.parse().unwrap_or(0)).collect();
        }).collect();

    return numbers;
}
