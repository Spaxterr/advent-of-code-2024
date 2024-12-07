use std::{collections::{HashMap, HashSet}, fs::read_to_string, ops::Index};

fn main() {
    let (mut pages, mapped_rules) = parse_input("./input.txt");
    let valid_pages = get_valid_pages(&mut pages, &mapped_rules);
    let invalid_pages: Vec<Vec<u8>> = pages
        .into_iter()
        .filter(|page| !valid_pages.contains(page))
        .collect();

    let sum: u16 = valid_pages
        .into_iter()
        .map(|numbers| numbers[(numbers.len() - 1) / 2] as u16)
        .sum();

    println!("Sum of middle in valid pages: {sum}");

    create_rule_sorting(&mapped_rules);
}

fn fix_page(page: &mut Vec<u8>, rules: &HashMap<u8, Vec<u8>>) {
}

fn parse_input(file_path: &str) -> (Vec<Vec<u8>>, HashMap<u8, Vec<u8>>) {
    let content = read_to_string(file_path).expect("Could not read file");

    // Input is split into two parts, separated by an empty line
    let parts: Vec<&str> = content
        .split("\n\n")
        .filter(|s| !s.is_empty())
        .take(2)
        .collect();

    let rules = parts[0];
    let pages = parts[1];

    // Get pages by getting each line and splitting on commas
    let mut page_lines: Vec<Vec<u8>> = Vec::new();
    for line in pages.split("\n") {
        page_lines.push(
            line.split(",")
                .filter(|n| !n.is_empty())
                .map(|n| n.parse().unwrap())
                .collect(),
        );
    }

    // Remove any lines that didn't contain any numbers
    page_lines = page_lines
        .into_iter()
        .filter(|line| line.len() != 0)
        .collect();

    return (page_lines, map_rules(rules));
}

fn get_valid_pages(pages: &mut Vec<Vec<u8>>, rules: &HashMap<u8, Vec<u8>>) -> Vec<Vec<u8>> {
    let mut valid_page_lines: Vec<Vec<u8>> = Vec::new();
    for line in pages {
        // Reverse to scan from end to start of vector
        line.reverse();

        if is_valid(line, &rules) {
            // Reverse again to reset order
            line.reverse();
            valid_page_lines.push(line.clone());
        }
    }
    return valid_page_lines;
}

fn is_valid(page: &mut Vec<u8>, rules: &HashMap<u8, Vec<u8>>) -> bool {
    let mut previous_numbers: Vec<u8> = Vec::new();
    for &number in page.iter() {
        if rules.contains_key(&number)
            && rules
                .get(&number)
                .unwrap()
                .iter()
                .any(|&n| previous_numbers.contains(&n))
        {
            return false;
        }
        previous_numbers.push(number);
    }
    return true;
}

fn map_rules(rules: &str) -> HashMap<u8, Vec<u8>> {
    // Map rules into a map
    // The map will contain numbers as keys with the value being a vector of numbers that should
    // appear before itself
    let mut map = HashMap::<u8, Vec<u8>>::new();
    for rule in rules.split("\n") {
        let mut parts = rule.split("|");
        let before: u8 = parts
            .next()
            .expect("Could not get first part")
            .parse()
            .unwrap();
        let after: u8 = parts
            .next()
            .expect("Could not get last part")
            .parse()
            .unwrap();

        map.entry(after).or_insert_with(Vec::new).push(before);
    }
    return map;
}

fn create_rule_sorting(rules: &HashMap<u8, Vec<u8>>) -> Vec<u8> {
    let mut all: HashSet<u8> = rules.keys().cloned().collect();
    for page in rules.values().cloned() {
        for num in page {
            all.insert(num);
        }
    }
    let numbers: Vec<u8> = all.into_iter().collect();
    let priority: HashMap<u8, u16> = HashMap::new();

    for number in numbers.iter() {
        let occurences = rules.values().filter(|n| n.contains(number)).count();
        println!("Count {occurences}");
        let count = rules.get(&number).unwrap().len();
    }

    return numbers;
}
