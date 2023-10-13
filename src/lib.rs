use std::collections::{HashMap, HashSet};

// const EXAMPLE: &str = include_str!("../inputs/example.txt");
const INPUT: &str = include_str!("../inputs/input.txt");

pub fn solve() {
    let mut elves: HashSet<Coord> = Default::default();

    for (row, line) in INPUT.lines().enumerate() {
        elves.extend(
            line.chars()
                .enumerate()
                .filter(|(_, c)| *c == '#')
                .map(|(col, _)| Coord {
                    col: col as isize,
                    row: row as isize,
                }),
        )
    }
    draw_board(&elves);

    let directions_cycle = [
        Direction::North,
        Direction::South,
        Direction::West,
        Direction::East,
    ].iter().cycle();

    for i in 0..10 {
        let d: Vec<&Direction> = directions_cycle.clone().skip(i%4).take(4).collect();
        // println!("{d:?}");
        let props = first_half(&elves, &d);
        // println!("{:?}", elves);
        // println!("{:#?}", props);

        elves.clear();
        for (prop, candidates) in props {
            // only one elf wants to move to that coord
            if candidates.len() == 1 {
                elves.insert(prop);
                continue;
            }

            // more than one elf wants to move to that coord
            // we'll keep them where they are
            elves.extend(candidates);
        }
    }

    draw_board(&elves);
}

fn first_half(elves: &HashSet<Coord>, directions: &[&Direction]) -> HashMap<Coord, Vec<Coord>> {
    let mut propositions: HashMap<Coord, Vec<Coord>> = Default::default();
    // get each elf proposition
    for elf in elves {
        let mut prop = elf.clone();
        let nbrs = elf.get_neighboors().map(|c| elves.contains(&c));
        if nbrs.iter().any(|n| *n) {
            // if neighboors propose walk
            'direction: for d in directions {
                match d {
                    Direction::North => {
                        if nbrs[0..3].iter().any(|n| *n) {
                            continue 'direction;
                        }

                        prop = elf.walk(Direction::North);
                        break 'direction;
                    }
                    Direction::South => {
                        if nbrs[4..7].iter().any(|n| *n) {
                            continue 'direction;
                        }

                        prop = elf.walk(Direction::South);
                        break 'direction;
                    }
                    Direction::East => {
                        if nbrs[2..5].iter().any(|n| *n) {
                            continue 'direction;
                        }

                        prop = elf.walk(Direction::East);
                        break 'direction;
                    }
                    Direction::West => {
                        if nbrs[6..].iter().any(|n| *n) || nbrs[0] {
                            continue 'direction;
                        }

                        prop = elf.walk(Direction::West);
                        break 'direction;
                    }
                }
            }
        }

        propositions
            .entry(prop)
            .and_modify(|coords| coords.push(*elf))
            .or_insert(vec![*elf]);
    }

    return propositions;
}

fn draw_board(elves: &HashSet<Coord>) {
    let mut max_row = isize::MIN;
    let mut min_row = isize::MAX;
    let mut max_col = isize::MIN;
    let mut min_col = isize::MAX;

    for elf in elves {
        max_row = max_row.max(elf.row);
        min_row = min_row.min(elf.row);
        max_col = max_col.max(elf.col);
        min_col = min_col.min(elf.col);
    }

    let mut empty = 0;
    println!("/// ELF BOARD ///");
    for i in min_row..=max_row {
        let mut line = String::new();
        for j in min_col..=max_col {
            if elves.contains(&Coord { row: i, col: j }) {
                line = format!("{line}#");
            } else {
                line = format!("{line}.");
                empty += 1;
            }
        }

        println!("{line}");
    }
    println!("///     END    ///");
    println!("Dimensions: {} x {}", max_row - min_row, max_col - min_col);
    println!("Empty grounds: {}", empty);
}

#[derive(Debug)]
enum Direction {
    North,
    South,
    West,
    East,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Coord {
    row: isize,
    col: isize,
}

impl std::ops::Add<(isize, isize)> for Coord {
    type Output = Self;

    fn add(self, rhs: (isize, isize)) -> Self::Output {
        Coord {
            row: self.row + rhs.0,
            col: self.col + rhs.1,
        }
    }
}

impl Coord {
    fn get_neighboors(self) -> [Coord; 8] {
        [
            self + (-1, -1), // NW
            self + (-1, 0),  // N
            self + (-1, 1),  // NE
            self + (0, 1),   // E
            self + (1, 1),   // SE
            self + (1, 0),   // S
            self + (1, -1),  // SW
            self + (0, -1),  // W
        ]
    }

    fn walk(self, d: Direction) -> Self {
        match d {
            Direction::North => self + (-1, 0),
            Direction::South => self + (1, 0),
            Direction::West => self + (0, -1),
            Direction::East => self + (0, 1),
        }
    }
}
