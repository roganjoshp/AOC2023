use std::collections::{HashMap, HashSet};
use std::{fs, str::FromStr};

#[derive(Debug)]
enum Error {
    CardParseError,
}

#[derive(Debug)]
struct Card {
    id: u32,
    winning_nos: HashSet<u32>,
    numbers: HashSet<u32>,
}

impl FromStr for Card {
    type Err = Error;

    fn from_str(string: &str) -> Result<Self, Self::Err> {
        let (name, game) = string.split_once(":").unwrap();
        let number = name
            .split_whitespace()
            .last()
            .unwrap()
            .parse::<u32>()
            .unwrap();
        let (winners, draw) = game.split_once("|").unwrap();

        return Ok(Card {
            id: number,
            winning_nos: winners
                .split_whitespace()
                .map(|n| n.parse::<u32>().unwrap())
                .collect::<HashSet<u32>>(),
            numbers: draw
                .split_whitespace()
                .map(|n| n.parse::<u32>().unwrap())
                .collect::<HashSet<u32>>(),
        });
    }
}

impl Card {
    fn get_wins(&self) -> u32 {
        self.numbers
            .intersection(&self.winning_nos)
            .collect::<Vec<&u32>>()
            .len() as u32
    }

    fn get_score(&self) -> u32 {
        let matching: u32 = self.get_wins();
        if matching == 0 {
            return 0 as u32;
        }
        u32::pow(2, matching as u32 - 1)
    }
}

fn build_cards(data: Vec<String>) -> Vec<Card> {
    data.iter()
        .map(|line| Card::from_str(line).unwrap())
        .collect()
}

fn process_deck(cards: &Vec<Card>) -> u32 {
    let mut all_cards: HashMap<u32, u32> = HashMap::new();
    let total_cards = cards.len() as u32;

    for card in cards {
        let qty_of_this_card = *all_cards.entry(card.id).or_insert(1);
        let card_wins = card.get_wins();
        // iterate for as many of this card we have
        for _ in 0..qty_of_this_card {
            for j in card.id..card.id + card_wins {
                if j < total_cards {
                    let next_card = &cards[j as usize];
                    all_cards
                        .entry(next_card.id)
                        .and_modify(|e| *e += 1)
                        .or_insert(2);
                }
            }
        }
    }
    all_cards.iter().map(|c| c.1).sum::<u32>()
}

fn read_input(filename: &str) -> Vec<String> {
    let file = fs::read_to_string(filename).expect("Cannot find file");
    let lines: Vec<String> = file.split("\n").map(|c| c.to_owned()).collect();
    lines
}

fn main() {
    let data = read_input("part_1.txt");
    let cards = build_cards(data);
    let part_1: u32 = cards.iter().map(|c| c.get_score()).sum();
    println!("{}", part_1);
    let part_2 = process_deck(&cards);
    println!("{}", part_2);
}
