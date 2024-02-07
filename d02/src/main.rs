use std::fs;

struct Round {
    red: i32,
    green: i32,
    blue: i32,
}

impl Round {
    fn new(round_str: &str) -> Self {
        let mut green: i32 = 0;
        let mut red: i32 = 0;
        let mut blue: i32 = 0;
        round_str
            .split(",")
            .into_iter()
            .map(|command| match command {
                command if command.ends_with("red") => {
                    red = command
                        .split_whitespace()
                        .next()
                        .unwrap()
                        .trim()
                        .parse::<i32>()
                        .expect("Number not found")
                }
                command if command.ends_with("blue") => {
                    blue = command
                        .split_whitespace()
                        .next()
                        .unwrap()
                        .trim()
                        .parse::<i32>()
                        .expect("Number not found")
                }
                command if command.ends_with("green") => {
                    green = command
                        .split_whitespace()
                        .next()
                        .unwrap()
                        .trim()
                        .parse::<i32>()
                        .expect("Number not found")
                }
                _ => (),
            });

        Self {
            red: red,
            blue: blue,
            green: green,
        }
    }
}

struct Game {
    score: i32,
    rounds: Vec<i32>,
}

impl Game {
    fn new(game_row: &String) -> Self {
        let (name, all_rounds) = game_row.split_once(":").unwrap();

        let (_, score) = name.split_once(" ").unwrap();
        let rounds: Vec<&str> = all_rounds.split(";").map(|n| n.trim()).collect();

        println!("{:?}", rounds);
        Self {
            score: 10,
            rounds: vec![1, 2, 3],
        }
    }
}

fn read_input(filename: &str) -> Vec<String> {
    let file = fs::read_to_string(filename).expect("Cannot find file");
    let lines: Vec<String> = file.split("\n").map(|c| c.to_owned()).collect();
    lines
}

fn main() {
    let data = read_input("part_1.txt");
    let new_data: Vec<Game> = data.iter().map(|n| Game::new(&n)).collect();
}
