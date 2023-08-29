use std::{fmt::Debug, fs::OpenOptions, io::Write};

mod helper;

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug)]
struct Coord {
    x: usize,
    y: usize,
}

impl Coord {
    fn parse(str: &str) -> Self {
        let coord_str = str.trim();

        coord_str.split_once(",").unwrap().into()
    }

    fn range(self, end: Self) -> Vec<Self> {
        if self.x == end.x {
            let min = usize::min(self.y, end.y);
            let max = usize::max(self.y, end.y);
            return (min..=max)
                .map(|i| Coord { x: self.x, y: i })
                .collect();
        } else if self.y == end.y {
            let min = usize::min(self.x, end.x);
            let max = usize::max(self.x, end.x);
            return (min..=max)
                .map(|i| Coord { x: i, y: self.y })
                .collect();
        }

        vec![]
    }

    fn fall_coord(self, direction: Direction) -> Coord {
        let options = self.get_fall_candidates();

        match direction {
            Direction::Down => options.0,
            Direction::DownLeft => options.1,
            Direction::DownRight => options.2,
        }
    }

    fn get_fall_candidates(self) -> (Coord, Coord, Coord) {
        (
            (self.x, self.y + 1).into(),        // down
            (self.x - 1, self.y + 1).into(),    // down left
            (self.x + 1, self.y + 1).into(),    // down right
        )
    }
}

impl From<(&str, &str)> for Coord {
    fn from(value: (&str, &str)) -> Self {
        Coord {
            x: value.0.parse().unwrap(),
            y: value.1.parse().unwrap(),
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

#[derive(Clone, Copy, PartialEq, Eq)]
enum SandState {
    Falling,
    Rest,
}

#[derive(Clone, Copy, PartialEq, Eq)]
enum Tile {
    Air,
    Rock,
    Sand(SandState),
}

#[derive(Clone, Copy)]
enum Direction {
    Down,
    DownLeft,
    DownRight,
}

impl Debug for Tile {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Air => write!(f, "."),
            Self::Rock => write!(f, "#"),
            Self::Sand(arg0) => match arg0 {
                SandState::Falling => write!(f, "Ö"),
                SandState::Rest => write!(f, "o"),
            },
        }
    }
}

struct Grid {
    height: usize,
    width: usize,
    tiles: Vec<Tile>,
    sand_source: Coord,
    falling_sand: Option<Coord>,
    full: bool,
}

impl Grid {
    fn build(height: usize, width: usize) -> Self {
        let tiles = vec![Tile::Air; height * width];

        Grid {
            height,
            width,
            tiles,
            sand_source: Coord {
                x: 500,
                y: 0,
            },
            falling_sand: None,
            full: false,
        }
    }

    fn add_rocks(&mut self, rock_coordinates: Vec<Vec<Coord>>) {
        rock_coordinates.iter().for_each(|coords| {
            coords.iter().reduce(|prev, next| {
                for coord in prev.range(next.clone()) {
                    let i = self.get_index(coord).unwrap();
                    self.tiles[i] = Tile::Rock;
                }

                next
            });
        })
    }

    fn get_index(&self, coord: Coord) -> Option<usize> {
        if coord.x > self.width || coord.y > self.height {
            println!("Coordinate not found {:?}", coord);

            return None;
        }

        Some(coord.y * self.width + coord.x)
    }

    fn get(&self, coord: Coord) -> Tile {
        self.tiles[self.get_index(coord).expect("coordinate should exist")]
    }

    fn _try_get(&self, coord: Coord) -> Option<Tile> {
        self.get_index(coord)
            .and_then(|index| self.tiles.get(index).cloned())
    }

    fn draw(&self, file_path: &str) {
        let mut f = OpenOptions::new().write(true).truncate(true).create(true).open(file_path).unwrap();
        let mut content = vec![];
        for y in 0..self.height {
            let mut line = String::new();
            for x in 0..self.width {
                if self.sand_source == (x, y).into() {
                    line = format!("{line}+");
                    continue;
                }

                let tile = self.get((x, y).into());
                line = format!("{line}{tile:?}");
            }
            // println!("{line}")
            writeln!(content, "{line}").unwrap();
        }

        f.write_all(&content).unwrap();
    }

    fn step(&mut self) {
        if self.falling_sand.is_none() {
            self.falling_sand = Some(self.sand_source);
        }

        let sand = self.falling_sand.take().unwrap();

        if self.is_free_falling(sand) {
            self.full = true;
            return;
        }

        let next = self.get_fall_direction(sand);

        if let Some(coord) = next {
            let prev_index = self.get_index(sand).unwrap();
            let next_index = self.get_index(coord).unwrap();

            self.tiles[prev_index] = Tile::Air;
            self.tiles[next_index] = Tile::Sand(SandState::Falling);
            self.falling_sand = Some(coord);
        } else {
            let sand_index = self.get_index(sand).unwrap();
            self.tiles[sand_index] = Tile::Sand(SandState::Rest);
        }
    }

    fn get_fall_direction(&self, sand: Coord) -> Option<Coord>{
        for direction in [Direction::Down, Direction::DownLeft, Direction::DownRight] {
            let tile = self.get(sand.fall_coord(direction));

            if tile == Tile::Air {
                return Some(sand.fall_coord(direction));
            }
        }

        None
    }

    // sand is free falling when it gets to the grid latteral or lower border |_|
    fn is_free_falling(&self, sand: Coord) -> bool{
        sand.x == 0 ||
            sand.x == self.width - 1  ||
            sand.y == self.height - 1
    }

    fn count_sand(&self) -> usize {
        self.tiles.iter().filter(|tile| **tile == Tile::Sand(SandState::Rest)).count()
    }
}

pub fn solve() {
    let mut rock_coordinates: Vec<Vec<Coord>> = helper::get_file_lines_iter("inputs/input.txt")
        .map(|l| {
            let line = l.unwrap();
            line.split("->").map(Coord::parse).collect()
        })
        .collect();

    let max_x = rock_coordinates.iter().flatten().max_by_key(|c| c.x).unwrap().x;
    let max_y = rock_coordinates.iter().flatten().max_by_key(|c| c.y).unwrap().y;

    // Height and width are max + 1 because values ranges from 0 to n, meaning length = max + 1
    let mut grid = Grid::build(max_y + 2 + 1, max_x*2 + 1);
    
    rock_coordinates.push(
        Coord::from((0, max_y + 2)).range((max_x*2, max_y + 2).into())
    );

    grid.add_rocks(rock_coordinates);


    while grid.get(grid.sand_source) != Tile::Sand(SandState::Rest) {
        grid.step();
    }

    grid.draw("out.txt");

    println!("Day 14 part 2 {}", grid.count_sand());
}

// change coordinates to start near to x = 0
// fn normalize_coords(coords: Vec<Vec<Coord>>) -> (Vec<Vec<Coord>>, usize) {
//     let min_x = coords.iter().flatten().min_by_key(|c| c.x).unwrap().x;

//     let coords = coords
//         .into_iter()
//         .map(|vec| {
//             vec.into_iter()
//                 .map(|c| Coord {
//                     x: c.x - min_x + 1, // leave 1 air gap in the beginning of the grid
//                     y: c.y,
//                 })
//                 .collect()
//         })
//         .collect();

//     (coords, min_x)
// }
