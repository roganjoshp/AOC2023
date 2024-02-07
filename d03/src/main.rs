use std::collections::HashSet;
use std::fs;

const NEIGHBOURHOOD: [[i32; 2]; 9] = [
    [-1, -1],
    [-1, 0],
    [-1, 1],
    [0, -1],
    [0, 0], // pointlesss
    [0, 1],
    [1, -1],
    [1, 0],
    [1, 1],
];

#[derive(Debug, Copy, Clone)]
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

#[derive(Debug, Default)]
struct Number {
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

fn build_maps(data: Vec<Vec<char>>) -> (Vec<Number>, HashSet<(i32, i32)>) {
    let mut seen_chars: HashSet<(i32, i32)> = HashSet::new();
    let mut numbers: Vec<Number> = Vec::new();

    let mut number_started: bool = false;
    let mut str_number = String::new();
    let mut digits: Vec<Digit> = Vec::new();

    for (i, row) in data.iter().enumerate() {
        if number_started {
            numbers.push(Number {
                value: str_number.parse::<i32>().unwrap(),
                digit_coords: digits.clone(),
            });
            number_started = false;
            str_number.clear();
            digits.clear();
        }
        for (j, col_value) in row.iter().enumerate() {
            if col_value.is_ascii_digit() {
                number_started = true;
                str_number.push(*col_value);
                digits.push(Digit::new(i as i32, j as i32));
            } else {
                if number_started {
                    numbers.push(Number {
                        value: str_number.parse::<i32>().unwrap(),
                        digit_coords: digits.clone(),
                    });
                    number_started = false;
                    str_number.clear();
                    digits.clear();
                }
                if *col_value != '.' {
                    seen_chars.insert((i as i32, j as i32));
                }
            }
        }
    }
    (numbers, seen_chars)
}

fn read_input(filename: &str) -> Vec<Vec<char>> {
    let file = fs::read_to_string(filename).expect("Cannot find file");
    let lines: Vec<Vec<char>> = file.split("\n").map(|c| c.chars().collect()).collect();
    lines
}

fn main() {
    let input = read_input("part_1.txt");

    let (numbers, symbol_indices) = build_maps(input);
    println!("{}", numbers.len());

    let part_1: i32 = numbers
        .iter()
        .filter(|&x| x.is_valid(&symbol_indices))
        .map(|x| x.value)
        .sum();
    println!("{}", part_1);
}
