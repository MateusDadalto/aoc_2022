use std::{fmt::Debug, time::Instant};

const EXAMPLE: &str = include_str!("../inputs/example.txt");
const INPUT: &str = include_str!("../inputs/input.txt");

pub fn solve() {
    let mut winds_cycle = INPUT.trim().chars().map(Direction::parse).cycle();

    let mut fall_space = FallSpace::new(10, 7);

    let mut n_rocks: usize = 0;
    let mut current_rock = fall_space.spawn_rock();

    let start = Instant::now();

    while n_rocks < 1_000_000_000_000 {
        let wind = winds_cycle.next().unwrap();

        current_rock = fall_space.move_by_wind(current_rock, wind);

        current_rock = fall_space.move_down(current_rock).unwrap_or_else(|| {
            n_rocks += 1;
            let new_rock = fall_space.spawn_rock();

            new_rock
        });

        if n_rocks%1_000_000_000 == 0{
            println!("Elapsed: {}, Rocks: {n_rocks:#?}", start.elapsed().as_secs_f32());
        }
    }

    println!("Day 17 part 1: {}", fall_space.rock_height);

}

struct FallSpace {
    height: usize,
    width: usize,
    tiles: Vec<Vec<Tile>>,
    rocks: Box<dyn Iterator<Item = RockType>>,
    rock_height: usize,
}

impl FallSpace {
    fn draw(&self, rock: RockInfo) {
        for (y,l) in self.tiles.iter().enumerate().rev() {
            let mut line = String::from("|");

            for (x,t) in l.iter().enumerate() {
                if rock.coords().contains(&(x,y).into()) {
                    line = format!("{line}@", );
                    continue;
                }
                line = format!("{line}{t:?}");
            }

            line = format!("{line}|");
            println!("{line}");
        }

        println!("|_______|");
        println!("");
    }

    fn new(height: usize, width: usize) -> Self {
        let tiles = vec![vec![Tile::Air; width]; height];

        let rocks = Box::new(
            [
                RockType::Line,
                RockType::Plus,
                RockType::InvertedL,
                RockType::Bar,
                RockType::Square,
            ]
            .into_iter()
            .cycle(),
        );

        FallSpace {
            height,
            width,
            tiles,
            rocks,
            rock_height: 0,
        }
    }

    fn get(&self, c: Coord) -> Tile {
        self.tiles[c.y][c.x]
    }

    fn set(&mut self, c: Coord, t: Tile) {
        self.tiles[c.y][c.x] = t;
    }

    fn increase_height(&mut self) {
        self.height += 10;
        self.tiles.extend(vec![vec![Tile::Air; self.width]; 10]);
    }

    fn spawn_rock(&mut self) -> RockInfo {
        let next_rock = self.rocks.next().unwrap();
        let starting_y = self.rock_height + 3;
        let starting_x = match next_rock {
            RockType::Plus => 3, // bottom tile of plus is one more to the left
            _ => 2,
        };

        if starting_y + 3 >= self.height {
            self.increase_height();
        }

        RockInfo {
            kind: next_rock,
            coord: (starting_x, starting_y).into(),
        }
    }

    fn move_by_wind(&self, rock: RockInfo, wind: Direction) -> RockInfo {
        let can_move = !rock
            .coords()
            .iter()
            .map(|c| c.try_move(wind, self.width))
            .any(|o| o.is_none() || self.get(o.unwrap()) == Tile::Rock);

        if can_move {
            return rock.move_wind(wind);
        }

        rock
    }

    fn move_down(&mut self, rock: RockInfo) -> Option<RockInfo> {
        let can_move = rock.coords()
            .iter()
            .all(|c| {
                c.y != 0 && self.get(*c - (0, 1).into()) != Tile::Rock
            });
        
        if can_move {
            return Some(rock.move_down());
        } 

        self.add_rock(rock);

        None
    }

    fn add_rock(&mut self, rock: RockInfo) {
        rock.coords().into_iter().for_each(|c| {
            self.rock_height = self.rock_height.max(c.y + 1);
            self.set(c, Tile::Rock);
        });
    }
}

#[derive(Clone, Copy, PartialEq, Eq)]
enum Tile {
    Rock,
    Air,
}

impl Debug for Tile {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Rock => write!(f, "#"),
            Self::Air => write!(f, "."),
        }
    }
}

#[derive(Clone, Copy)]
enum Direction {
    Right,
    Left,
}

impl Direction {
    fn parse(c: char) -> Self {
        match c {
            '>' => Direction::Right,
            '<' => Direction::Left,
            any => panic!("Wtf is your input: {any}"),
        }
    }
}
#[derive(Clone, Copy)]
enum RockType {
    Line,
    Plus,
    InvertedL,
    Bar,
    Square,
}

impl RockType {}

#[derive(Clone, Copy, PartialEq, Eq)]
struct Coord {
    x: usize,
    y: usize,
}

impl Coord {
    fn try_move(self, wind: Direction, right_limit: usize) -> Option<Self> {
        match wind {
            Direction::Right => {
                if self.x + 1 >= right_limit {
                    return None;
                } else {
                    Some((self.x + 1, self.y).into())
                }
            }
            Direction::Left => self.x.checked_sub(1).and_then(|x| Some((x, self.y).into())),
        }
    }
}

impl From<(usize, usize)> for Coord {
    fn from(value: (usize, usize)) -> Self {
        Coord {
            x: value.0,
            y: value.1,
        }
    }
}

impl std::ops::Sub for Coord {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Coord {
            x: self.x - rhs.x,
            y: self.y - rhs.y
        }
    }
}

#[derive(Clone, Copy)]
struct RockInfo {
    kind: RockType,
    coord: Coord, // Coord is the value of its most bottom-left tile
}

impl RockInfo {
    fn move_wind(self, wind: Direction) -> Self {
        match wind {
            Direction::Right => RockInfo {
                kind: self.kind,
                coord: (self.coord.x + 1, self.coord.y).into(),
            },
            Direction::Left => RockInfo {
                kind: self.kind,
                coord: (self.coord.x - 1, self.coord.y).into(),
            },
        }
    }

    fn move_down(self) -> Self {
        return RockInfo {
            kind: self.kind,
            coord: (self.coord.x, self.coord.y - 1).into(),
        };
    }

    fn coords(&self) -> Vec<Coord> {
        let c = self.coord;
        match self.kind {
            RockType::Line => vec![
                c,
                (c.x + 1, c.y).into(),
                (c.x + 2, c.y).into(),
                (c.x + 3, c.y).into(),
            ],
            RockType::Plus => vec![
                c,
                (c.x, c.y + 1).into(),
                (c.x + 1, c.y + 1).into(),
                (c.x - 1, c.y + 1).into(),
                (c.x, c.y + 2).into(),
            ],
            RockType::InvertedL => vec![
                c,
                (c.x + 1, c.y).into(),
                (c.x + 2, c.y).into(),
                (c.x + 2, c.y + 1).into(),
                (c.x + 2, c.y + 2).into(),
            ],
            RockType::Bar => vec![
                c,
                (c.x, c.y + 1).into(),
                (c.x, c.y + 2).into(),
                (c.x, c.y + 3).into(),
            ],
            RockType::Square => vec![
                c,
                (c.x, c.y + 1).into(),
                (c.x + 1, c.y).into(),
                (c.x + 1, c.y + 1).into(),
            ],
        }
    }
}
