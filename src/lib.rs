use std::{hash::{Hash, Hasher}, collections::HashMap, ops::{Add, Sub, Mul, Div}};

const EXAMPLE: &str = include_str!("../inputs/example.txt");
const INPUT: &str = include_str!("../inputs/input.txt");
const ROOT: &str = "root";
const HUMAN: &str = "humn";

pub fn solve() {
    // let monkeys: HashMap<String, Monkey> = EXAMPLE.lines().map(Monkey::parse).map(|m| (m.name.clone(), m)).collect();
    let mut monkeys: HashMap<String, Monkey> = INPUT.lines().map(Monkey::parse).map(|m| (m.name.clone(), m)).collect();

    {
        let root = monkeys.get_mut(ROOT).unwrap();
        root.value = root.value.replace("+", "-");
    }

    let root = monkeys.get(ROOT).unwrap();
    // make it so root will need to be 0. (lhs - rhs = 0)
    let mut root_result = root.eval(&monkeys);
    let mut a = 0.; //wild guesses, I could narrow it down but where is the fun?
    let a_sign = eval_humn_at(a, &mut monkeys).signum();
    let mut b = 100_000_000_000_000.;
    let b_sign = eval_humn_at(b, &mut monkeys).signum();

    // lets solve it numerically, why not?????? who doesn't love the good ol' bisection
    while !are_floats_equal(root_result, 0.) {
        let guess = (a + b) / 2.;
        root_result = eval_humn_at(guess, &mut monkeys);

        if root_result.signum() == a_sign {
            a = guess;
            
        } else if root_result.signum() == b_sign {
            b = guess;
        } else {
            panic!("look at me, I'm the captain now"); // https://www.youtube.com/watch?v=WxhTbxMSvT0
        }
    }

    println!("Day 20 part 2 {}", monkeys.get(HUMAN).unwrap().value);
}

fn eval_humn_at(x: f64, monkeys: &mut HashMap<String, Monkey>) -> f64 {
    monkeys.get_mut(HUMAN).unwrap().value = x.to_string();

    monkeys.get(ROOT).unwrap().eval(&monkeys)
}

#[derive(Debug, Clone)]
// in hindsight, I'd change monkey to be an enum, no need for kind... but we're too deep already
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

fn are_floats_equal(a: f64, b: f64) -> bool {
    (a - b).abs() < 1e-5
}