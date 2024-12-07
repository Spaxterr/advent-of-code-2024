use std::fs::read_to_string;

fn main() {
    let content = read_to_string("./input.txt").expect("Could not read file");
    let lines = content.split("\n").filter(|line| !line.is_empty());

    let mut word_search: Vec<Vec<char>> = Vec::new();
    for line in lines {
        word_search.push(line.chars().collect());
    }

    let mut xmas_total = 0;
    xmas_total += horizontal_search(&word_search);
    xmas_total += vertical_search(&word_search);
    xmas_total += diagonal_search(&word_search);

    let mas_total = x_search(&word_search);
    println!("XMAS Total: {xmas_total}\nMAS Total: {mas_total}");
}

fn horizontal_search(word_search: &Vec<Vec<char>>) -> u16 {
    let mut count = 0;
    for line in word_search {
        for i in 0..line.len() {
            if i >= line.len() - 3 {
                break;
            }

            let mut word = String::new();
            for n in 0..4 {
                word.push(line[n + i]);
            }
            if is_xmas(&word) {
                count += 1;
            }
        }
    }
    return count;
}

fn vertical_search(word_search: &Vec<Vec<char>>) -> u16 {
    let mut count = 0;

    let columns = word_search[0].len();
    let rows = word_search.len() - 3;

    for row in 0..rows {
        for i in 0..columns {
            let mut word = String::new();
            for n in 0..4 {
                word.push(word_search[row + n][i]);
            }
            if is_xmas(&word) {
                count += 1;
            }
        }
    }

    return count;
}

fn diagonal_search(word_search: &Vec<Vec<char>>) -> u16 {
    let mut count = 0;

    let columns = word_search[0].len();
    let rows = word_search.len() - 3;

    for row in 0..rows {
        for col in 0..columns {
            let mut first_word = String::new();
            let mut second_word = String::new();

            // Stop at 4th last column
            for n in 0..4 {
                if col + n >= columns {
                    break;
                }
                first_word.push(word_search[row + n][col + n]);
            }

            // Ignore first 4 columns
            for n in 0..4 {
                if n > col {
                    break;
                }
                second_word.push(word_search[row + n][col - n]);
            }

            if is_xmas(&first_word) {
                count += 1;
            }
            if is_xmas(&second_word) {
                count += 1;
            }
        }
    }

    return count;
}

fn x_search(word_search: &Vec<Vec<char>>) -> u16 {
    let mut count = 0;

    let columns = word_search[0].len();
    let rows = word_search.len() - 2;
    for row in 0..rows {
        for col in 0..columns {
            let mut first_word = String::new();
            let mut second_word = String::new();

            for n in 0..3 {
                if col + n >= columns {
                    break;
                }
                first_word.push(word_search[row + n][col + n]);
            }

            for n in 0..3 {
                if (col + 2) - n >= columns {
                    break;
                }
                second_word.push(word_search[row + n][(col + 2) - n]);
            }

            if is_mas(&first_word) && is_mas(&second_word) {
                count += 1;
            }
        }
    }

    return count;
}

fn is_xmas(value: &str) -> bool {
    return value == "XMAS" || value == "SAMX";
}

fn is_mas(value: &str) -> bool {
    return value == "MAS" || value == "SAM";
}
