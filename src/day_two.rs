use std::{cmp::Ordering, io::{BufReader, BufRead}, fs};

#[derive(Debug, Clone)]
enum RPS {
    Rock,
    Paper,
    Scisor,
}

#[derive(Debug, Clone)]
enum Outcome {
    Loss,
    Draw,
    Win,
}

impl Outcome {
    fn build(c: &char) -> Self{
        match c {
            'X' => Self::Loss,
            'Y' => Self::Draw,
            'Z' => Self::Win,
            _ => panic!("Invalid character provided!")
        }
    }
}

impl RPS {
    fn value(&self) -> i32 {
        match self {
            RPS::Rock => 1,
            RPS::Paper => 2,
            RPS::Scisor => 3,
        }
    }

    fn beats(&self, other: &Self) -> bool {
        matches!(
            (self, other), 
                (Self::Rock, Self::Scisor) 
                | (Self::Paper, Self::Rock) 
                | (Self::Scisor, Self::Paper)
        )
    }

    fn compare(&self, other: &Self) -> Ordering {
        if self.beats(other) {
            return Ordering::Greater
        } else if other.beats(self) {
            return Ordering::Less
        } 

        Ordering::Equal
    }

    fn build(c: &char) -> Self {
        match c {
            'A' => RPS::Rock,
            'B' => RPS::Paper,
            'C' => RPS::Scisor,
            _ => panic!("Invalid character provided!")
        }
    }

    fn build_for_outcome(outcome: &Outcome, opponent: &Self) -> Self {
        let options = [RPS::Rock, RPS::Paper, RPS::Scisor];

        match outcome {
            Outcome::Loss => options.into_iter().find(|option| opponent.beats(option)),
            Outcome::Draw => Some(opponent.clone()),
            Outcome::Win => options.into_iter().find(|option| option.beats(opponent)),
        }
        .unwrap()
    }

    fn play_round(&self, opponent: &RPS) -> i32 {
        let mut result = self.value();
        result += match self.compare(opponent) {
            Ordering::Less => 0,
            Ordering::Equal => 3,
            Ordering::Greater => 6,
        };

        result
    }

}

pub fn solve() {
    let file = fs::File::open("inputs/day_two.txt").expect("file should exist");
    let buffer = BufReader::new(file);
    let mut result = 0;

    for line in buffer.lines() {
        let play = build_play(line.expect("line should build"));

        if let Some((opp, you)) = play {
            result += you.play_round(&opp);
        }
    }

    println!("DAY 2 ANSWER: {result}")
}

fn build_play(line: String) -> Option<(RPS, RPS)> {
    let chars: Vec<char> = line.chars().collect();
    if chars.len() < 3 {
        return None;
    }
    let opponent = RPS::build(chars.first().unwrap());
    let outcome = Outcome::build(chars.last().unwrap());
    let you = RPS::build_for_outcome(&outcome, &opponent);
    Some((opponent, you))
}