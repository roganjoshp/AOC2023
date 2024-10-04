use rayon::prelude::*;
use std::collections::HashMap;
use std::fs;
use std::thread::current;

#[derive(Debug)]
struct Seed {
    id: u64,
}

impl Seed {
    fn from_id(id: &str) -> Self {
        Self {
            id: id.parse::<u64>().unwrap(),
        }
    }
}

#[derive(Debug, Default)]
struct MappedRange {
    start: u64,
    end: u64,
    destination_start: u64,
    size: u64,
}

impl MappedRange {
    fn new(start: u64, end: u64, destination_start: u64, size: u64) -> Self {
        Self {
            start: start,
            end: end,
            destination_start: destination_start,
            size: size,
        }
    }

    fn in_range(&self, source: u64) -> bool {
        source >= self.start && source <= self.end
    }

    fn get_destination(&self, source: u64) -> u64 {
        self.destination_start + (source - self.start)
    }
}

#[derive(Debug, Default)]
struct Mapper {
    seeds: Vec<Seed>,
    map_order: Vec<String>,
    maps: HashMap<String, Vec<MappedRange>>,
    seed_scores: HashMap<Seed, u64>,
}

impl Mapper {
    fn new() -> Self {
        let maps = vec![
            "seed-to-soil",
            "soil-to-fertilizer",
            "fertilizer-to-water",
            "water-to-light",
            "light-to-temperature",
            "temperature-to-humidity",
            "humidity-to-location",
        ];

        let mut collected_maps: HashMap<String, Vec<MappedRange>> = HashMap::new();

        Self {
            map_order: maps.iter().map(|&s| s.to_owned()).collect(),
            maps: collected_maps,
            ..Default::default()
        }
    }

    fn parse_data(&mut self, data: &Vec<String>) -> () {
        // The first row is just a list of the seeds
        let seed_data: Vec<&str> = data[0].split_ascii_whitespace().collect();
        let seeds: Vec<Seed> = seed_data[1..]
            .iter()
            .map(|&id| Seed::from_id(&id))
            .collect();
        self.seeds = seeds;

        let mut active_map = "";
        for row in data[1..].iter() {
            if row.is_empty() {
                continue;
            } else if !row.chars().next().unwrap().is_numeric() {
                active_map = row.split_ascii_whitespace().next().unwrap();
            } else {
                self.populate_map(&active_map, row);
            }
        }
    }

    fn populate_map(&mut self, map_name: &str, values: &str) -> () {
        let data: Vec<u64> = values
            .split_ascii_whitespace()
            .map(|val| val.parse::<u64>().unwrap())
            .collect();

        self.maps
            .entry(map_name.to_owned())
            .or_insert(Vec::new())
            .push(MappedRange::new(
                data[1],
                data[1] + data[2] - 1,
                data[0],
                data[2],
            ));
    }

    fn find_destination(&self, map_name: &str, source: u64) -> u64 {
        let map = self.maps.get(map_name).unwrap();
        for map_range in map.iter() {
            let contains = map_range.in_range(source);
            if contains {
                return map_range.get_destination(source);
            }
        }
        return source;
    }

    fn score_seed(&self, seed_id: u64) -> u64 {
        let mut current_source = seed_id;
        for map in &self.map_order {
            let destination = self.find_destination(map, current_source);
            current_source = destination;
        }
        current_source
    }

    fn find_min_individual_seed_score(&self) -> u64 {
        let mut lowest_score = u64::MAX;

        for seed in &self.seeds {
            let seed_score = self.score_seed(seed.id);
            if seed_score < lowest_score {
                lowest_score = seed_score;
            }
        }
        lowest_score
    }

    fn find_min_seed_pair_score_brute(&self) -> u64 {
        let mut seed_scores: Vec<u64> = Vec::with_capacity(self.seeds.len() / 2);
        let mut count: i32 = 1;
        for pair in self.seeds.chunks(2) {
            let start = pair[0].id;
            let end = pair[0].id + pair[1].id - 1;
            let mut min_score: u64 = (start..end)
                .into_par_iter()
                .map(|seed_id| self.score_seed(seed_id))
                .min()
                .unwrap();
            seed_scores.push(min_score);
            println!("Done {:?}", count);
            count += 1;
        }
        println!("{:?}", seed_scores);
        *seed_scores.iter().min().unwrap()
    }
}

fn read_input(filename: &str) -> Vec<String> {
    let file = fs::read_to_string(filename).expect("Cannot find file");
    let lines: Vec<String> = file.split("\n").map(|c| c.to_owned()).collect();
    lines
}

fn main() {
    let mut maps = Mapper::new();
    let data = read_input("part_1.txt");
    maps.parse_data(&data);
    let min_score = maps.find_min_seed_pair_score_brute();
    println!("MIN: {:?}", min_score);
}
