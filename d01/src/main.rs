use std::collections::HashMap;
use std::fs;

fn read_input(filename: &str) -> Vec<String> {
    let file = fs::read_to_string(filename).expect("Cannot find file");
    let lines: Vec<String> = file.split("\n").map(|c| c.to_owned()).collect();
    lines
}

fn get_digits(instructions: &Vec<String>) -> Vec<String> {
    instructions
        .iter()
        .map(|row| row.chars().filter(|c| c.is_digit(10)).collect())
        .collect()
}

fn sum_instructions(instructions: Vec<String>) -> u32 {
    instructions
        .iter()
        .map(|a| match a.len() {
            0 => 0,
            1 => (a.parse::<u32>().unwrap() * 10) + a.parse::<u32>().unwrap(),
            2 => a.parse::<u32>().unwrap(),
            _ => {
                a.chars().next().unwrap().to_digit(10).unwrap() * 10
                    + a.chars().nth_back(0).unwrap().to_digit(10).unwrap()
            }
        })
        .sum()
}

fn search_for_string_no(
    letters: &Vec<String>,
    candidates: &HashMap<&str, Vec<&str>>,
    word_map: &HashMap<&str, u32>,
    eager: bool,
) -> Option<u32> {
    let mut number: Option<u32> = None;
    for (i, letter) in letters.iter().enumerate() {
        if candidates.contains_key(&letter.as_ref()) {
            let words = candidates.get(&letter.as_ref()).unwrap();
            for &word in words.iter() {
                if letters[i..].join("").starts_with(&word) {
                    if eager {
                        return Some(*word_map.get(word).unwrap());
                    }
                    number = Some(*word_map.get(word).unwrap());
                }
            }
        }
    }
    number
}

fn find_numbers(instructions: &Vec<String>) -> u32 {
    let candidates: HashMap<&str, Vec<&str>> = HashMap::from([
        ("o", vec!["one"]),
        ("t", vec!["two", "three"]),
        ("f", vec!["four", "five"]),
        ("s", vec!["six", "seven"]),
        ("e", vec!["eight"]),
        ("n", vec!["nine"]),
    ]);
    let word_map: HashMap<&str, u32> = HashMap::from([
        ("one", 1),
        ("two", 2),
        ("three", 3),
        ("four", 4),
        ("five", 5),
        ("six", 6),
        ("seven", 7),
        ("eight", 8),
        ("nine", 9),
    ]);
    let mut total_sum: u32 = 0;

    for (x, inst) in instructions.iter().enumerate() {
        let mut first_value: u32 = 0;
        let mut last_value: u32 = 0;

        let first_to_digit: Vec<String> = inst
            .chars()
            .take_while(|c| !c.is_digit(10))
            .map(|c| c.to_string())
            .collect();

        if first_to_digit.len() < 3 {
            // We definitely can't have a string number
            first_value = inst
                .chars()
                .nth(first_to_digit.len())
                .unwrap()
                .to_digit(10)
                .unwrap();
        } else {
            let string_search = search_for_string_no(&first_to_digit, &candidates, &word_map, true);
            match string_search {
                Some(_i) => first_value = string_search.unwrap(),
                _ => {
                    first_value = inst
                        .chars()
                        .nth(first_to_digit.len())
                        .unwrap()
                        .to_digit(10)
                        .unwrap()
                }
            }
        }

        let last_to_digit: Vec<String> = inst
            .chars()
            .rev()
            .take_while(|c| !c.is_digit(10))
            .map(|c| c.to_string())
            .collect();

        if last_to_digit.len() < 3 {
            last_value = inst
                .chars()
                .nth_back(last_to_digit.len())
                .unwrap()
                .to_digit(10)
                .unwrap();
        } else {
            let ending = last_to_digit.into_iter().rev().collect();
            let rev_search = search_for_string_no(&ending, &candidates, &word_map, false);
            match rev_search {
                Some(i) => last_value = rev_search.unwrap(),
                _ => {
                    last_value = inst
                        .chars()
                        .nth_back(ending.len())
                        .unwrap()
                        .to_digit(10)
                        .unwrap()
                }
            }
        }
        total_sum = total_sum + (first_value * 10) + last_value;
    }
    total_sum
}

fn main() {
    let data = read_input("part_1.txt");
    let digits = get_digits(&data);
    // println!("{:?}", digits);
    let part_1 = sum_instructions(digits);
    println!("{:?}", part_1);
    let part_2 = find_numbers(&data);
    println!("{:?}", part_2);
}
