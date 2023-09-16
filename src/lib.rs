#[allow(dead_code)]
const EXAMPLE: &str = include_str!("../inputs/example.txt");
#[allow(dead_code)]
const INPUT: &str = include_str!("../inputs/input.txt");

fn parse_input(input: &str) -> (Grid, Vec<Instruction>) {
    let mut input = input.lines();
    let mut grid_lines: Vec<Vec<Tile>> = vec![];
    while let Some(line) = input.next() {
        if line.is_empty() {
            break;
        }

        grid_lines.push(Tile::parse_line(line));
    }

    let commands = parse_commands(input.next().unwrap());

    (Grid::new(grid_lines), commands)
}

fn parse_commands(commands: &str) -> Vec<Instruction> {
    let mut instructions = vec![];
    let mut digits = vec![];

    for c in commands.chars() {
        if c.is_numeric() {
            digits.push(c.to_digit(10).unwrap() as usize);
        } else {
            let n = digits.iter().fold(0, |acc, d| acc * 10 + d);
            digits.clear();

            instructions.push(Instruction::WALK(n));

            let turn = match c {
                'R' => Instruction::ROTATE(Turn::R),
                'L' => Instruction::ROTATE(Turn::L),
                x => panic!("Wrong instruction: {}", x),
            };

            instructions.push(turn);
        }
    }

    if !digits.is_empty() {
        let n = digits.iter().fold(0, |acc, d| acc * 10 + d);
        instructions.push(Instruction::WALK(n));
    }

    return instructions;
}

pub fn solve() {
    let (grid, commands) = parse_input(INPUT);
    // let (grid, commands) = parse_input(INPUT);
    let mut current = grid.get_starting_position();
    let mut facing = Direction::RIGHT;

    // grid.draw();
    println!("{:?}, {facing:?}", current);

    for i in commands {
        println!("{i:?}");
        match i {
            Instruction::ROTATE(t) => {
                facing = facing.rotate(t);
            },
            Instruction::WALK(n) => {
                for _ in 0..n {
                    let next = grid.step(current, facing);

                    if next == current {
                        break;
                    }

                    current = next;
                    println!("{:?}, {facing:?}", current);
                }
            },
        }
    }

    println!("positon: {:?}, facing: {:?}", current, facing);
    let key = 1000 * (current.y + 1) + 4 * (current.x + 1) + (facing as usize);
    println!("Day 22 part 1: {}", key);
}

struct Grid {
    tiles: Vec<Vec<Tile>>,
}

impl Grid {
    fn new(tiles: Vec<Vec<Tile>>) -> Self {
        Grid { tiles: tiles }
    }

    #[allow(dead_code)]
    fn draw(&self) {
        for l in &self.tiles {
            let mut line = String::new();

            for t in l {
                match t {
                    Tile::Empty => line = format!("{}{}", line, " "),
                    Tile::Wall => line = format!("{}{}", line, "#"),
                    Tile::Path => line = format!("{}{}", line, "."),
                }
            }

            println!("{}", line);
        }
    }

    fn get_starting_position(&self) -> Coord {
        let x = self.tiles[0].iter().position(|t| *t != Tile::Empty).unwrap();

        (x, 0).into()
    }

    fn get_line_first_position(&self, l: usize) -> usize {
        let line = &self.tiles[l];

        line.iter().position(|t| *t != Tile::Empty).unwrap().clone()
    }

    // get index of the first line with a valid tile in that column
    fn get_column_first_position(&self, c: usize) -> usize {
        // println!("{c}");
        self.tiles
            .iter()
            .position(|l| l.get(c).is_some_and(|t| *t != Tile::Empty))
            .unwrap()
    }

    fn get_column_last_position(&self, c: usize) -> usize {
        self.tiles.len()
            - 1
            - self
                .tiles
                .iter()
                .rev()
                .position(|l| l.get(c).is_some_and(|t| *t != Tile::Empty))
                .unwrap()
    }

    fn step(&self, c: Coord, d: Direction) -> Coord {
        match d {
            Direction::UP => self.step_up(c),
            Direction::RIGHT => self.step_right(c),
            Direction::DOWN => self.step_down(c),
            Direction::LEFT => self.step_left(c),
        }
    }

    fn step_right(&self, c: Coord) -> Coord {
        let mut new_c = c + (1, 0).into();

        let line = &self.tiles[c.y];
        if new_c.x >= line.len() {
            new_c.x = self.get_line_first_position(new_c.y);
        }

        match self.get(new_c) {
            Tile::Empty => panic!("tile should either be wall or path"),
            Tile::Wall => c,
            Tile::Path => new_c,
        }
    }

    fn step_left(&self, coord: Coord) -> Coord {
        let line = &self.tiles[coord.y];
        //last tile will never be empty due to how inputs are
        let last_tile_coord = (line.len() - 1, coord.y).into();
        let last_tile = self.get(last_tile_coord);
        // get x-1 or last tile coord if x -1 < 0
        let new_c = coord.checked_sub((1, 0).into()).unwrap_or(last_tile_coord);
        let tile = line[new_c.x];

        match tile {
            Tile::Path => new_c,
            Tile::Empty if last_tile == Tile::Path => last_tile_coord,
            Tile::Empty => coord,
            Tile::Wall => coord,
        }
    }

    fn step_up(&self, coord: Coord) -> Coord {
        // go up or wrap all the way down if y-1 < 0;
        let new_c = coord
            .checked_sub((0, 1).into())
            .unwrap_or_else(|| (coord.x, self.tiles.len() - 1).into());

        match self.get_or_empty(new_c) {
            Tile::Path => new_c,
            Tile::Wall => coord,
            Tile::Empty => {
                let last_valid_position = (new_c.x, self.get_column_last_position(new_c.x)).into();

                if self.get(last_valid_position) == Tile::Wall {
                    return coord;
                }

                return last_valid_position;
            }
        }
    }

    fn step_down(&self, coord: Coord) -> Coord {
        let new_c;
        if coord.y + 1 < self.tiles.len() {
            new_c = coord + (0, 1).into();
        } else {
            new_c = (coord.x, 0).into();
        }

        match self.get_or_empty(new_c) {
            Tile::Path => new_c,
            Tile::Wall => coord,
            Tile::Empty => {
                let first_valid_position =
                    (new_c.x, self.get_column_first_position(new_c.x)).into();

                if self.get(first_valid_position) == Tile::Wall {
                    return coord;
                }

                return first_valid_position;
            }
        }
    }

    fn get(&self, coord: Coord) -> Tile {
        self.tiles[coord.y][coord.x]
    }

    fn get_or_empty(&self, coord: Coord) -> Tile {
        *self
            .tiles
            .get(coord.y)
            .unwrap()
            .get(coord.x)
            .unwrap_or(&Tile::Empty)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Tile {
    Empty,
    Wall,
    Path,
}

impl Tile {
    fn parse_line(l: &str) -> Vec<Self> {
        l.chars().map(Self::parse).collect()
    }

    fn parse(c: char) -> Self {
        match c {
            '.' => Tile::Path,
            '#' => Tile::Wall,
            _ => Tile::Empty,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Coord {
    x: usize,
    y: usize,
}

impl Coord {
    fn checked_sub(self, rhs: Self) -> Option<Self> {
        let x = self.x.checked_sub(rhs.x)?;
        let y = self.y.checked_sub(rhs.y)?;

        Some(Coord { x, y })
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

impl std::ops::Add for Coord {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Coord {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Direction {
    RIGHT,
    DOWN,
    LEFT,
    UP,
}

impl Direction {
    fn rotate(self, t: Turn) -> Self{
        match t {
            Turn::R => match self {
                Direction::UP => Direction::RIGHT,
                Direction::RIGHT => Direction::DOWN,
                Direction::DOWN => Direction::LEFT,
                Direction::LEFT => Direction::UP,
            },
            Turn::L => match self {
                Direction::UP => Direction::LEFT,
                Direction::RIGHT => Direction::UP,
                Direction::DOWN => Direction::RIGHT,
                Direction::LEFT => Direction::DOWN,
            },
        }
    }

}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Instruction {
    ROTATE(Turn),
    WALK(usize),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Turn {
    R,
    L,
}
