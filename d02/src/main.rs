use std::{fs, str::FromStr};

#[derive(Debug, Default)]
struct Round {
    red: i32,
    green: i32,
    blue: i32,
}

#[derive(Debug)]
enum Error {
    NoColourFound,
}

const MAX_ROUND: Round = Round {
    red: 12,
    green: 13,
    blue: 14,
};

impl FromStr for Round {
    type Err = Error;

    fn from_str(data: &str) -> Result<Self, Self::Err> {
        let mut obj = Round::default();

        let split_parts: Vec<&str> = data.split(",").map(|s| s.trim()).collect();

        for part in split_parts {
            let (number, colour) = part.split_once(" ").unwrap();
            let qty = number.parse::<i32>().unwrap();

            match colour {
                "red" => obj.red = qty,
                "green" => obj.green = qty,
                "blue" => obj.blue = qty,
                _ => return Err(Error::NoColourFound),
            };
        }
        Ok(obj)
    }
}

impl Round {
    fn within_max(&self) -> bool {
        self.red <= MAX_ROUND.red && self.green <= MAX_ROUND.green && self.blue <= MAX_ROUND.blue
    }
}

#[derive(Debug)]
struct Game {
    score: i32,
    rounds: Vec<Round>,
}

impl Game {
    fn new(game_row: &String) -> Self {
        let (name, all_rounds) = game_row.split_once(":").unwrap();

        let (_, score) = name.split_once(" ").unwrap();
        let rounds: Vec<Round> = all_rounds
            .split(";")
            .map(|n| Round::from_str(n).unwrap())
            .collect();

        Self {
            score: score.parse::<i32>().unwrap(),
            rounds: rounds,
        }
    }

    fn check_rounds(&self) -> bool {
        self.rounds.iter().all(|round| round.within_max())
    }

    fn get_power(&self) -> i32 {
        let mut max_red = 0;
        let mut max_green = 0;
        let mut max_blue = 0;

        for round in &self.rounds {
            if round.red > max_red {
                max_red = round.red
            }
            if round.green > max_green {
                max_green = round.green
            }
            if round.blue > max_blue {
                max_blue = round.blue
            }
        }
        max_red * max_green * max_blue
    }
}

fn sum_winning_games(games: &Vec<Game>) -> i32 {
    games
        .iter()
        .filter(|&game| game.check_rounds())
        .map(|game| game.score)
        .sum()
}

fn read_input(filename: &str) -> Vec<String> {
    let file = fs::read_to_string(filename).expect("Cannot find file");
    let lines: Vec<String> = file.split("\n").map(|c| c.to_owned()).collect();
    lines
}

fn main() {
    let data = read_input("part_1.txt");
    let games: Vec<Game> = data.iter().map(|n| Game::new(&n)).collect();
    let part_1 = sum_winning_games(&games);
    println!("{:?}", part_1);
    let part_2: i32 = games.iter().map(|game| game.get_power()).sum();
    println!("{:?}", part_2);
}
