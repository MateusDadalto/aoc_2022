use std::{
    fs,
    io::{BufRead, BufReader}, collections::HashSet,
};

#[derive(Debug)]
struct Ruckgroup(Rucksack, Rucksack, Rucksack);

#[derive(Debug)]
struct Rucksack {
    chars: HashSet<char>
}

impl Rucksack {
    fn try_build(input: &str) -> Self {
        Rucksack {
            chars: HashSet::from_iter(input.chars()),
        }
    }
}

impl Ruckgroup {
    fn from_vec(v: &mut Vec<Rucksack>) -> Self {
        if v.len() != 3 {
            panic!("Ruckgroup size must be 3!")
        }

        Ruckgroup(v.pop().unwrap(), v.pop().unwrap(), v.pop().unwrap())
    }

    fn find_badge(&self) -> &char {
        self.0.chars.iter()
            .find(|&c| self.1.chars.contains(c) && self.2.chars.contains(c))
            .unwrap()
    }

    fn calculate_badge_priority(&self) -> u32 {
        let c = self.find_badge();

        if c.is_uppercase() {
            return (*c as u32) - 38;
        }

        (*c as u32) - 96
    }
}

pub fn solve() {
    let file = fs::File::open("inputs/day_three.txt").expect("file should exist");
    let mut lines = BufReader::new(file).lines();

    let mut counter = 0;
    let mut badges = vec![];
    let mut group = vec![];

    while let Some(line) = lines.next() {
        counter += 1;
        let line = line.unwrap();

        group.push(Rucksack::try_build(&line));

        if counter % 3 == 0 {
            let ruckgroup = Ruckgroup::from_vec(&mut group);

            badges.push(ruckgroup.calculate_badge_priority());
            group.clear();
        }
    }

    println!("DAY 3 PART TWO: {}", badges.into_iter().sum::<u32>())
}
