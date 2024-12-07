use std::collections::HashMap;
use std::collections::HashSet;
use std::fs;

fn main() {
    let (mut left, mut right) = get_numbers("./input.txt");
    let distance = get_total_distance(&mut left, &mut right);
    let similarity = get_similarity_score(left, right);

    println!("Distance: {distance}\nSimilarity: {similarity}");
}

fn get_total_distance(left_numbers: &mut Vec<i32>, right_numbers: &mut Vec<i32>) -> i32 {
    left_numbers.sort_unstable();
    right_numbers.sort_unstable();

    let result: i32 = left_numbers.into_iter().zip(right_numbers).map(|(left, right)| (*left - *right).abs()).sum();

    return result;
}

fn get_similarity_score(left_numbers: Vec<i32>, right_numbers: Vec<i32>) -> i32 {
    let unique_left_numbers: HashSet<i32> = left_numbers.into_iter().collect();
    let mut right_number_occurences: HashMap<i32, i32> = HashMap::new();

    for number in right_numbers {
        if unique_left_numbers.contains(&number) {
            *right_number_occurences.entry(number).or_insert(0) += 1;
        }
    }

    let result: i32 = right_number_occurences.into_iter().map(|(number, occurences)| {
        return number * occurences;
    }).sum();

    return result;
}

fn get_numbers(file_path: &str) -> (Vec<i32>, Vec<i32>) {
    let file_contents = fs::read_to_string(file_path).expect("Could read file");

    let (left_numbers, right_numbers): (Vec<i32>, Vec<i32>) = file_contents
        .lines().map(|line| {
            let mut numbers = line.trim().split_whitespace();
            let first = numbers.next().unwrap_or("0").parse().unwrap_or(0);
            let second = numbers.next().unwrap_or("0").parse().unwrap_or(0);
            return (first, second)
        }).unzip();

    return (left_numbers, right_numbers);
}

