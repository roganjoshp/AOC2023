use std::collections::{HashMap, HashSet};
use std::fs;

const NEIGHBOURHOOD: [[i32; 2]; 8] = [
    [-1, -1],
    [-1, 0],
    [-1, 1],
    [0, -1],
    [0, 1],
    [1, -1],
    [1, 0],
    [1, 1],
];

#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
struct Digit {
    i: i32,
    j: i32,
}

impl Digit {
    fn new(i: i32, j: i32) -> Self {
        Self { i: i, j: j }
    }

    fn has_neighbour(&self, symbol_indices: &HashSet<(i32, i32)>) -> bool {
        for coord in NEIGHBOURHOOD {
            if symbol_indices.contains(&(self.i + coord[0], self.j + coord[1])) {
                return true;
            }
        }
        false
    }
}

#[derive(Debug, Default, Hash, PartialEq, Eq, Clone)]
struct Number {
    id: i32,
    value: i32,
    digit_coords: Vec<Digit>,
}

impl Number {
    fn is_valid(&self, symbol_indices: &HashSet<(i32, i32)>) -> bool {
        self.digit_coords
            .iter()
            .any(|&d| d.has_neighbour(&symbol_indices))
    }
}

fn build_maps(data: Vec<Vec<char>>) -> (Vec<Number>, HashSet<(i32, i32)>, HashSet<(i32, i32)>) {
    let mut seen_chars: HashSet<(i32, i32)> = HashSet::new();
    let mut numbers: Vec<Number> = Vec::new();
    let mut star_map: HashSet<(i32, i32)> = HashSet::new();

    let mut number_started: bool = false;
    let mut str_number = String::new();
    let mut digits: Vec<Digit> = Vec::new();
    let mut seen_numbers = 0;

    for (i, row) in data.iter().enumerate() {
        if number_started {
            numbers.push(Number {
                id: seen_numbers,
                value: str_number.parse::<i32>().unwrap(),
                digit_coords: digits.clone(),
            });
            number_started = false;
            str_number.clear();
            digits.clear();
            seen_numbers += 1;
        }
        for (j, col_value) in row.iter().enumerate() {
            if col_value.is_ascii_digit() {
                number_started = true;
                str_number.push(*col_value);
                digits.push(Digit::new(i as i32, j as i32));
            } else {
                if number_started {
                    numbers.push(Number {
                        id: seen_numbers,
                        value: str_number.parse::<i32>().unwrap(),
                        digit_coords: digits.clone(),
                    });
                    number_started = false;
                    str_number.clear();
                    digits.clear();
                    seen_numbers += 1;
                }
                if *col_value != '.' {
                    seen_chars.insert((i as i32, j as i32));
                }
                if *col_value == '*' {
                    star_map.insert((i as i32, j as i32));
                }
            }
        }
    }
    (numbers, seen_chars, star_map)
}

fn build_inverted_number_map(numbers: &Vec<Number>) -> HashMap<(i32, i32), Number> {
    let mut inverted_map: HashMap<(i32, i32), Number> = HashMap::new();
    for number in numbers {
        for digit in &number.digit_coords {
            inverted_map.insert((digit.i, digit.j), number.clone());
        }
    }
    inverted_map
}

fn find_gears(star_map: HashSet<(i32, i32)>, number_map: HashMap<(i32, i32), Number>) -> i32 {
    let mut seen_numbers: HashSet<&Number> = HashSet::new();
    let mut total = 0;

    for star in star_map {
        for neighbour in NEIGHBOURHOOD {
            if number_map.contains_key(&(star.0 + neighbour[0], star.1 + neighbour[1])) {
                let number = number_map
                    .get(&(star.0 + neighbour[0], star.1 + neighbour[1]))
                    .unwrap();
                seen_numbers.insert(number);
            }
        }
        if seen_numbers.len() == 2 {
            let mut inner_total = 1;
            for &number in &seen_numbers {
                inner_total = inner_total * number.value;
            }
            total += inner_total
        }
        seen_numbers.clear();
    }
    total
}

fn read_input(filename: &str) -> Vec<Vec<char>> {
    let file = fs::read_to_string(filename).expect("Cannot find file");
    let lines: Vec<Vec<char>> = file.split("\n").map(|c| c.chars().collect()).collect();
    lines
}

fn main() {
    let input = read_input("test_file.txt");

    let (numbers, symbol_indices, star_map) = build_maps(input);

    let part_1: i32 = numbers
        .iter()
        .filter(|&x| x.is_valid(&symbol_indices))
        .map(|x| x.value)
        .sum();
    println!("{}", part_1);

    let inverted_map = build_inverted_number_map(&numbers);
    let part_2 = find_gears(star_map, inverted_map);
    println!("{}", part_2);
}
