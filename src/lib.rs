use std::collections::HashSet;

mod helper;

enum Direction {
    Right,
    Left,
    Up,
    Down
}

impl Direction {
    fn parse(c: &str) -> Direction {
        match c.to_ascii_uppercase().as_str() {
            "R" => Direction::Right,
            "L" => Direction::Left,
            "U" => Direction::Up,
            "D" => Direction::Down,
            _ => panic!("Not a valid character")
        }
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
struct Position {
    x: i32,
    y: i32
}

impl Position {
    fn is_disjointed(&self, other: &Position) -> bool {
        let distance = *self - *other;

        distance.x.abs() > 1 || distance.y.abs() > 1
    }
}

impl std::ops::Sub for Position{
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
        }
    }
}

struct Knot {
    position: Position
}

impl Knot {
    fn apply_motion(&mut self, direction: &Direction) {
        match direction {
            Direction::Right => self.position.x += 1,
            Direction::Left => self.position.x -= 1,
            Direction::Up => self.position.y += 1,
            Direction::Down => self.position.y -= 1,
        }
    }

    fn follow_head(&mut self, head: &Knot) {
        if !self.is_disjointed(head) {
            return;
        }

        let head_distance = head.position - self.position;
        
        self.position.x += sig_num(head_distance.x) as i32;
        self.position.y += sig_num(head_distance.y) as i32;
    }

    fn is_disjointed(&self, head: &Knot) -> bool {
        self.position.is_disjointed(&head.position)
    }
}

pub fn solve() {
    let lines = helper::get_file_lines_iter("inputs/input.txt");
    let mut head = Knot {position: Position { x: 0, y: 0 }};
    let mut tail =  Knot {position: Position { x: 0, y: 0 }};

    let mut positions = HashSet::new();
    positions.insert(tail.position);

    for line in lines {
        let instructions: Vec<String> = line.unwrap().split_whitespace().map(|s| s.to_owned()).collect();
        let direction = Direction::parse(instructions[0].as_str());
        let n = instructions[1].parse::<u32>().unwrap();

        for _ in 0..n {
            head.apply_motion(&direction);

            tail.follow_head(&head);
            positions.insert(tail.position);
        }
    }

    println!("Day 9 part 1: {}", positions.len())
}

fn sig_num(n: i32) -> i8 {
    match n {
        n if n > 0 => 1,
        0 => 0,
        _ => -1
    }
}