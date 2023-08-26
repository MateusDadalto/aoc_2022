use itertools::Itertools;
use std::{cmp::Ordering, fmt::Debug, iter::Peekable, str::Chars};

mod helper;

enum Packet {
    Value(u32),
    List(Vec<Packet>),
}

impl Packet {
    fn parse(line: &str) -> Vec<Self> {
        let mut interior = line[1..line.len() - 1].chars().peekable();
        let mut data = vec![];

        loop {
            let Some(c) = interior.peek() else {
                break;
            };

            match c {
                '[' => data.push(Self::parse_list(&mut interior)),
                c if c.is_digit(10) => data.push(Self::parse_number(&mut interior)),
                _ => {
                    interior.next();
                }
            }
        }

        data
    }

    fn parse_list(chars: &mut Peekable<Chars>) -> Self {
        let mut list = vec![];
        chars.next(); // remove list start character

        loop {
            match chars.peek() {
                Some(c) if c.is_digit(10) => list.push(Self::parse_number(chars)),
                Some(c) if *c == ']' => {
                    chars.next();
                    break;
                }
                Some(c) if *c == '[' => {
                    list.push(Self::parse_list(chars));
                }
                Some(_) => {
                    chars.next();
                }
                None => panic!("List never closed!"),
            }
        }

        Self::List(list)
    }

    fn parse_number(chars: &mut Peekable<Chars>) -> Self {
        let n: String = chars
            .peeking_take_while(|c| *c != ',' && *c != ']')
            .collect();
        Self::Value(n.parse().unwrap())
    }

    fn compare(&self, right: &Packet) -> Ordering {
        match (self, right) {
            (Packet::Value(l), Packet::Value(r)) => l.cmp(r),
            (Packet::Value(l), Packet::List(r)) => Self::compare_lists(&vec![Self::Value(*l)], r),
            (Packet::List(l), Packet::Value(r)) => Self::compare_lists(l, &vec![Self::Value(*r)]),
            (Packet::List(l), Packet::List(r)) => Self::compare_lists(l, r),
        }
    }

    fn compare_lists(left: &Vec<Packet>, right: &Vec<Packet>) -> Ordering {
        for either_or_both in left.iter().zip_longest(right) {
            if !either_or_both.has_left() {
                return Ordering::Less;
            }
            if !either_or_both.has_right() {
                return Ordering::Greater;
            }

            let (l, r) = either_or_both.both().unwrap();

            let comparison = l.compare(r);

            if comparison == Ordering::Equal {
                continue;
            } else {
                return comparison;
            }
        }

        Ordering::Equal // if both lists are empty or every item match
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

    let mut indexes = vec![];

    for (index, (left, right)) in lines.iter().tuples().enumerate() {
        let left = Packet::parse(&left);
        let right = Packet::parse(&right);

        match Packet::compare_lists(&left, &right) {
            Ordering::Less => {
                indexes.push(index);
                println!("{left:?}");
                println!("{right:?}");
                println!();
            }
            Ordering::Equal => (),
            Ordering::Greater => (),
        }
    }
    // println!("Day 13 part 1: {:?}", indexes);

    println!("Day 13 part 1: {:?}", indexes.iter().map(|i| i + 1).sum::<usize>());
}
