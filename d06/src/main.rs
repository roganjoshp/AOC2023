use std::fs;

#[derive(Debug, Default)]
struct RaceRecord {
    time: u64,
    distance: u64,
}

fn multiply_all_wins(wins: &[u64]) -> u64 {
    wins.iter().fold(1, |res, a| res * a)
}

impl RaceRecord {
    fn new(time: &str, distance: &str) -> Self {
        Self {
            time: time.parse::<u64>().unwrap(),
            distance: distance.parse::<u64>().unwrap(),
            ..Default::default()
        }
    }

    fn evaluate_charge_times(&self) -> u64 {
        let mut wins: u64 = 0;
        for charge_time in 0..self.time {
            let remaining_time = self.time - charge_time;
            if remaining_time * charge_time > self.distance {
                wins += 1
            }
        }
        wins
    }
}

fn read_input(filename: &str) -> Vec<RaceRecord> {
    let file = fs::read_to_string(filename).expect("Cannot find file");
    let rows: Vec<_> = file
        .split("\n")
        .map(|row| row.split_ascii_whitespace().skip(1).collect::<Vec<&str>>())
        .collect();
    let mut records: Vec<RaceRecord> = Vec::new();
    for (i, time) in rows[0].iter().enumerate() {
        records.push(RaceRecord::new(&time, &rows[1][i]))
    }
    records
}

fn fix_kerning(filename: &str) -> RaceRecord {
    let file = fs::read_to_string(filename).expect("Cannot find file");
    let rows: Vec<_> = file
        .split("\n")
        .map(|row| row.split_ascii_whitespace().skip(1).collect::<Vec<&str>>())
        .collect();
    let joined: Vec<_> = rows
        .iter()
        .map(|line| line.iter().fold(String::new(), |acc, entry| acc + entry))
        .collect();
    RaceRecord::new(&joined[0], &joined[1])
}

fn main() {
    let records = read_input("part_1.txt");
    let wins: Vec<u64> = records
        .iter()
        .map(|record| record.evaluate_charge_times())
        .collect();
    let mult_wins = multiply_all_wins(&wins);
    println!("Part 1: {mult_wins}");
    let single_race = fix_kerning("part_1.txt");
    let num_wins = single_race.evaluate_charge_times();
    println!("Part 2: {num_wins}");
}
