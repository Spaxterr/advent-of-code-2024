use std::fs::read_to_string;
use regex::Regex;

fn main() {
    let muls: Vec<(i32, i32)> = find_muls("./input.txt", false);
    let instructed_muls: Vec<(i32, i32)> = find_muls("./input.txt", true);

    let mut answer = 0;
    for mul in muls {
        answer += mul.0 * mul.1;
    }

    println!("Total mul: {answer}");

    answer = 0;
    for mul in instructed_muls {
        answer += mul.0 * mul.1;
    }

    println!("Total instructed muls: {answer}");
}

fn find_muls(file_path: &str, use_instructions: bool) -> Vec<(i32, i32)> {
    let content = read_to_string(file_path).expect("Could not read file {file_path}");
    let re = Regex::new(r"mul\(\d+,\s*\d+\)|do(?:n't)?").unwrap();

    let result: Vec<&str> = re.find_iter(&content).map(|m| m.as_str()).collect();

    let mut enabled = true;
    let mut muls: Vec<(i32, i32)> = Vec::<(i32, i32)>::new();

    for res in result {
        if res == "do" {
            enabled = true;
            continue;
        } else if res == "don't" {
            enabled = false;
            continue;
        } else {
            if enabled || !use_instructions {
                let num_str = res.replace("mul(", "").replace(")", "");
                let numbers: Vec<i32> = num_str.split(",").take(2)
                    .map(|n| {
                        n.parse().expect("Could not parse number ")
                    }).collect();
                muls.push((numbers[0], numbers[1]));
            }
        }
    }

    return muls;
}
