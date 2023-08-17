use std::{fmt::Display, collections::VecDeque};

use crate::helper;

#[derive(Debug)]
struct MoveCmd {
    crates_to_move: u8,
    from: char,
    to: char,
}

impl MoveCmd {
    // line pattern: move X from Y to Z
    fn parse(line: String) -> Self {
        let line_pattern: Vec<String> = line.split_whitespace().map(|s| s.to_string()).collect();

        MoveCmd {
            crates_to_move: line_pattern[1].parse::<u8>().unwrap(),
            from: line_pattern[3].parse::<char>().unwrap(),
            to: line_pattern[5].parse::<char>().unwrap(),
        }
    }
}

#[derive(Debug)]
struct Crate {
    label: char,
    original_index: usize,
}

impl Crate {
    fn new(index: usize, label: char) -> Self {
        Crate {
            label,
            original_index: index,
        }
    }
}

#[derive(Debug)]
struct CrateStack {
    label: char,
    crates: Vec<Crate>,
    original_index: usize,
}

impl CrateStack {
    fn new(index: usize, label: char) -> Self {
        CrateStack {
            crates: vec![],
            original_index: index,
            label,
        }
    }
}

impl Display for Crate {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "[{}]", self.label)
    }
}

pub fn solve() {
    let mut lines_iter = helper::get_file_lines_iter("inputs/day_five.txt");

    let mut crate_stacks = vec![];
    let mut crates = vec![];
    // block to parse crate drawing
    while let Some(line) = lines_iter.next() {
        let line = line.unwrap();
        if line == "" {
            break;
        }

        let mut line_crates = parse_crates(line.as_str());

        if line_crates.is_empty() {
            crate_stacks.append(&mut parse_stacks(line));
        } else {
            crates.append(&mut line_crates);
        }
    }

    crate_stacks = fill_stacks(crate_stacks, crates);

    for line in lines_iter {
        let line = line.unwrap();
        let move_cmd = MoveCmd::parse(line);

        move_crates(move_cmd, &mut crate_stacks);
    }

    draw_board(&crate_stacks);
}

fn fill_stacks(mut stacks: Vec<CrateStack>, crates: Vec<Crate>) -> Vec<CrateStack> {
    crates.into_iter().rev().for_each(|c| {
        let own_stack = stacks
            .iter_mut()
            .find(|stack| stack.original_index == c.original_index)
            .unwrap();

        own_stack.crates.push(c);
    });

    stacks
}

fn parse_stacks(line: String) -> Vec<CrateStack> {
    line.match_indices(|c: char| c.is_digit(10))
        .map(|(index, char)| CrateStack::new(index, char.chars().next().unwrap()))
        .collect()
}

fn parse_crates(line: &str) -> Vec<Crate> {
    let crates: Vec<_> = line
        .match_indices("[")
        .map(|(crate_index, _)| {
            let index = crate_index + 1;
            let label = line.as_bytes()[crate_index + 1] as char;
            Crate::new(index, label)
        })
        .collect();

    crates
}

fn move_crates(move_cmd: MoveCmd, stacks: &mut Vec<CrateStack>) {
    let from = stacks.iter_mut().find(|stack| stack.label == move_cmd.from).unwrap();
    let mut crates_to_move = VecDeque::new();
    for _ in 0..move_cmd.crates_to_move {
        crates_to_move.push_front(from.crates.pop().unwrap());
    }

    let to = stacks.iter_mut().find(|stack| stack.label == move_cmd.to).unwrap();
    to.crates.append(&mut crates_to_move.into_iter().collect());
}

fn draw_board(stacks: &Vec<CrateStack>) {
    let height = stacks.iter().map(|s| s.crates.len()).max().unwrap();
    
    for line_number in (0..height).rev() {
        let mut line = String::new();
        for stack in stacks {
            match stack.crates.get(line_number) {
                Some(crt) => line = format!("{line} {crt}"),
                None => line = format!("{line}    "),
            }
        }
        println!("{}", &line[1..])
    }
    let mut crate_labels = String::new();
    for stack in stacks {
        crate_labels = format!("{crate_labels}  {} ", stack.label)
    }
    println!("{}", &crate_labels[1..])
}