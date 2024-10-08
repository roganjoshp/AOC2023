use std::cmp::Ordering;
use std::collections::HashMap;
use std::fs;

fn get_card_value(card: char) -> Option<u8> {
    match card {
        'A' => Some(14),
        'K' => Some(13),
        'Q' => Some(12),
        'J' => Some(11),
        'T' => Some(10),
        '9' => Some(9),
        '8' => Some(8),
        '7' => Some(7),
        '6' => Some(6),
        '5' => Some(5),
        '4' => Some(4),
        '3' => Some(3),
        '2' => Some(2),
        '1' => Some(1),
        _ => None,
    }
}

#[derive(Debug, Ord, Eq, PartialEq, PartialOrd)]
struct Card {
    name: char,
    value: u8,
}

impl Card {
    fn new(name: char, value: u8) -> Self {
        Self {
            name: name,
            value: value,
        }
    }
}

#[derive(Debug, Ord, Eq, PartialEq)]
struct Cards {
    cards: Vec<Card>,
    bid: u64,
    ordering: u8,
}

#[derive(Debug, Eq, PartialEq, Ord, PartialOrd)]
enum Hand {
    FiveOfKind(Cards), // 7
    FourOfKind(Cards), // 6
    FullHouse(Cards),
    ThreeOfKind(Cards),
    TwoPair(Cards),
    OnePair(Cards),
    HighCard(Cards),
}

impl PartialOrd for Cards {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        if self.ordering > other.ordering {
            return Some(Ordering::Greater);
        } else if self.ordering == other.ordering {
            for (i, card) in self.cards.iter().enumerate() {
                if card.value > other.cards[i].value {
                    return Some(Ordering::Less);
                } else if card.value == other.cards[i].value {
                    return Some(Ordering::Equal);
                } else {
                    return Some(Ordering::Greater);
                }
            }
        } else {
            return Some(Ordering::Less);
        }
        None
    }
}

fn determine_hand(deck: Vec<Card>, bid: u64) -> Option<Hand> {
    let mut counter: HashMap<char, u32> = HashMap::new();
    for card in &deck {
        *counter.entry(card.name).or_default() += 1
    }
    let max_count = *counter.values().max().unwrap();
    let mut seen_pairs = 0;
    let mut distinct_cards = 0;

    for counts in counter.values() {
        if *counts == 2 {
            seen_pairs += 1
        }
        distinct_cards += 1
    }
    if max_count == 5 {
        return Some(Hand::FiveOfKind(Cards {
            cards: deck,
            bid: bid,
            ordering: 7,
        }));
    } else if max_count == 4 {
        return Some(Hand::FourOfKind(Cards {
            cards: deck,
            bid: bid,
            ordering: 6,
        }));
    } else if max_count == 3 && distinct_cards == 2 {
        return Some(Hand::FullHouse(Cards {
            cards: deck,
            bid: bid,
            ordering: 5,
        }));
    } else if max_count == 3 && distinct_cards == 3 {
        return Some(Hand::ThreeOfKind(Cards {
            cards: deck,
            bid: bid,
            ordering: 4,
        }));
    } else if seen_pairs == 2 {
        return Some(Hand::TwoPair(Cards {
            cards: deck,
            bid: bid,
            ordering: 3,
        }));
    } else if seen_pairs == 1 && distinct_cards == 4 {
        return Some(Hand::OnePair(Cards {
            cards: deck,
            bid: bid,
            ordering: 2,
        }));
    } else if distinct_cards == 5 {
        return Some(Hand::HighCard(Cards {
            cards: deck,
            bid: bid,
            ordering: 1,
        }));
    }
    None
}

fn create_hand(hand: &[&str]) -> Hand {
    let cards = hand[0];
    let bid = hand[1].parse::<u64>().unwrap();
    let mut deck: Vec<Card> = Vec::new();
    for card in cards.chars() {
        let value = get_card_value(card).unwrap();
        deck.push(Card::new(card, value));
    }
    let hand = determine_hand(deck, bid).unwrap();
    hand
}

fn read_input(filename: &str) -> Vec<Hand> {
    let file = fs::read_to_string(filename).expect("Cannot find file");
    let rows: Vec<_> = file
        .split("\n")
        .map(|row| row.split_ascii_whitespace().collect::<Vec<&str>>())
        .collect();

    let hands = rows.iter().map(|i| create_hand(i)).collect::<Vec<_>>();
    hands
}

fn play_game_1(hands: &mut Vec<Hand>) -> () {
    hands.sort();
    let mut score = 0;
    for (rank, hand) in hands.iter().enumerate() {
        println!("{:?}\n\n", hand);
        // score += rank * hand.?????;
    }
    println!("Part 1: {score}");
}

fn main() {
    let mut hands = read_input("test.txt");
    play_game_1(&mut hands);
}
