use std::{str::Chars, iter::Peekable, fmt::Debug};
use itertools::Itertools;

mod helper;

enum Packet {
    Value(u32),
    List(Vec<Packet>),
}

impl Packet {
    fn parse(line: &str) -> Self{
        let mut interior = line[1..line.len()-1].chars().peekable();
        let mut data = vec![];

        
        loop {
            let Some(c) = interior.peek() else {
                break;
            };

            match c {
                '[' => data.push(Self::parse_list(&mut interior)),
                c if c.is_digit(10) => data.push(Self::parse_number(&mut interior)),
                _ => { interior.next(); }
            }
        }

        Self::List(data)
    }

    fn parse_list(chars: &mut Peekable<Chars>) -> Self{
        let mut list = vec![];
        chars.next(); // remove list start character

        loop {
            match chars.peek() {
                Some(c) if c.is_digit(10) => list.push(Self::parse_number(chars)),
                Some(c) if *c == ']' => { 
                    chars.next();
                    break;
                },
                Some(c) if *c == '[' => { 
                    list.push(Self::parse_list(chars));
                },
                Some(_) => {chars.next();},
                None => panic!("List never closed!")
            }
        }
        
        Self::List(list)
    }

    fn parse_number(chars: &mut Peekable<Chars>) -> Self {

        let n: String = chars.peeking_take_while(|c| *c != ',' && *c != ']').collect();
        Self::Value(n.parse().unwrap())
    }
}

impl Debug for Packet {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Value(arg0) => f.write_fmt(format_args!("{}", arg0)),
            Self::List(arg0) => f.debug_list().entries(arg0).finish(),
        }
    }
}

pub fn solve() {
    let lines: Vec<String> = helper::get_file_lines_iter("inputs/input.txt")
        .map(|s| s.unwrap())
        .filter(|s| !s.is_empty())
        .collect();

    for line in lines {
        println!("{:?}", Packet::parse(&line));
    }
}