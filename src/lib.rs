use std::{hash::{Hash, Hasher}, collections::HashMap, ops::{Add, Sub, Mul, Div}};

const EXAMPLE: &str = include_str!("../inputs/example.txt");
const INPUT: &str = include_str!("../inputs/input.txt");

pub fn solve() {
    // let monkeys: HashMap<String, Monkey> = EXAMPLE.lines().map(Monkey::parse).map(|m| (m.name.clone(), m)).collect();
    let monkeys: HashMap<String, Monkey> = INPUT.lines().map(Monkey::parse).map(|m| (m.name.clone(), m)).collect();

    let r = monkeys.get("root").unwrap().eval(&monkeys);

    // let no_humn = monkeys.get("njlw").unwrap().eval(&monkeys);
    // let humn = monkeys.get("gvfh").unwrap().eval(&monkeys);

    // println!("NO HUMAN: {}", no_humn);
    // println!("HUMAN: {humn}");

    // println!("diff: {}", no_humn - humn);
    println!("{}", r);
}

#[derive(Debug, Clone)]
struct Monkey {
    name: String,
    kind: MonkeyType,
    value: String,
}

impl Monkey {
    fn parse(line: &str) -> Self {
        let name = line[0..4].to_string();

        let value = line[6..].to_string();

        let kind = match value.parse::<f64>() {
            Ok(_) => MonkeyType::Value,
            Err(_) => MonkeyType::Operation,
        };

        Monkey { name, kind, value }
    }

    fn eval(&self, monkeys: &HashMap<String, Monkey>) -> f64 {
        if self.name.as_str() == "humn" {
            // return 3_412_650_897_410;
            return 3_412_650_897_405.;
        }

        match self.kind {
            MonkeyType::Value => self.value.parse().unwrap(),
            MonkeyType::Operation => {
                let content: Vec<_> = self.value.split_whitespace().collect();
                let monkey1 = monkeys.get(content[0]).unwrap().eval(monkeys);
                let operator = Operator::parse(content[1]);
                let monkey2 = monkeys.get(content[2]).unwrap().eval(monkeys);

                match operator {
                    Operator::Add => monkey1.add(monkey2),
                    Operator::Sub => monkey1.sub(monkey2),
                    Operator::Multi => monkey1.mul(monkey2),
                    Operator::Div => monkey1.div(monkey2),
                }
            },
        }
    }
}

impl Hash for Monkey {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.name.hash(state);
    }
}

#[derive(Debug, Clone, Copy)]
enum MonkeyType {
    Operation,
    Value,
}

enum Operator {
    Add,
    Sub,
    Multi,
    Div,
}

impl Operator {
    fn parse(s: &str) -> Self {
        match s {
            "+" => Self::Add,
            "-" => Self::Sub,
            "*" => Self::Multi,
            "/" => Self::Div,
            any => panic!("Invalid string for operator: {any}")
        }
    }
}