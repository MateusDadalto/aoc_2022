use std::{
    fs,
    io::{BufRead, BufReader},
};

#[derive(Debug)]
struct Rucksack {
    pocket_a: Vec<char>,
    pocket_b: Vec<char>,
}

impl Rucksack {
    fn try_build(input: &str) -> Self {
        let half = input.len() / 2;

        Rucksack {
            pocket_a: (&input[0..half]).chars().collect(),
            pocket_b: (&input[half..]).chars().collect(),
        }
    }

    fn find_repeated_item(&self) -> &char {
        self.pocket_a.iter().find(|c| self.pocket_b.contains(*c)).unwrap()
    }

    fn calculate_priority(&self) -> u32 {
        let c = self.find_repeated_item();

        if c.is_uppercase() {
            return (*c as u32) - 38;
        }

        (*c as u32) - 96
    }
}

pub fn solve() {
    let file = fs::File::open("inputs/day_three.txt").expect("file should exist");
    let buffer = BufReader::new(file);

    let total_priority: u32 = buffer.lines()
        .map(|line| {
            let rucksack = Rucksack::try_build(line.unwrap().as_str());

            rucksack.calculate_priority()
        }).sum();

    println!("DAY 3 ANSWER: {total_priority}");
}
