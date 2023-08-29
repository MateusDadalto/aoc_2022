use std::{fs::OpenOptions, io::Write, fmt::Debug};

use coord::Coord;
use point::Point;

mod helper;
mod point;
mod coord;

#[derive(Clone, Copy)]
enum Tile {
    Beacon,
    Sensor(Coord),
    Empty,
    Unknown,
}

impl Debug for Tile {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Beacon => write!(f, "B"),
            Self::Sensor(_) => write!(f, "S"),
            Self::Unknown => write!(f, "."),
            Self::Empty => write!(f, "#"),
        }
    }
}

struct Grid {
    height: usize,
    width: usize,
    tiles: Vec<Tile>,
}

impl Grid {
    fn build(max_x: usize, max_y: usize) -> Self {
        let width = max_x + 1;  // max value goes from 0 to max, meaning width = max + 1
        let height = max_y + 1; // max value goes from 0 to max, meaning height = max + 1
        Grid {
            width,
            height,
            tiles: vec![Tile::Unknown; width * height]
        }
    }

    fn add_sensor_and_beacon(&mut self, inputs: Vec<(Coord, Coord)>) {
        for (sensor, beacon) in inputs {
            let sensor_index = self.get_index(sensor);
            let beacon_index = self.get_index(beacon);

            self.tiles[sensor_index] = Tile::Sensor(beacon);
            self.tiles[beacon_index] = Tile::Beacon;

            self.add_sensor_information(sensor, beacon);
        }
    }

    fn add_sensor_information(&mut self, sensor: Coord, beacon: Coord) {
        let radius = sensor.distance(beacon);
        for coord in sensor.get_neighbours(radius) {
            if let Some(index) = self.try_get_index(coord) {
                match self.tiles[index] {
                    Tile::Beacon => (),
                    Tile::Sensor(_) => (),
                    Tile::Empty => (),
                    Tile::Unknown => {
                        self.tiles[index] = Tile::Empty;
                    },
                }
            }
        }
    }

    fn try_get_index(&self, coord: Coord) -> Option<usize> {
        if self.width > coord.x && self.height > coord.y {
            return Some(self.get_index(coord));
        }

        None
    }

    fn get_index(&self, coord: Coord) -> usize {
        coord.y * self.width + coord.x
    }

    fn get(&self, coord: Coord) -> Tile {
        self.tiles[self.get_index(coord)]
    }

    fn draw(&self, file_path: &str) {
        let mut f = OpenOptions::new().write(true).truncate(true).create(true).open(file_path).unwrap();
        let mut content = vec![];
        for y in 0..self.height {
            let mut line = String::new();
            for x in 0..self.width {
                let tile = self.get((x, y).into());
                line = format!("{line}{tile:?}");
            }
            // println!("{line}")
            writeln!(content, "{line}").unwrap();
        }

        f.write_all(&content).unwrap();
    }
}

pub fn solve() {
    let lines = helper::get_file_lines_iter("inputs/input.txt");

    let mut input: Vec<(Point, Point)> = vec![];
    for line in lines {
        let line = line.unwrap();
        let parts = line.split_once(':').unwrap();
        let sensor_point = Point::parse(parts.0);
        let beacon_point = Point::parse(parts.1);

        input.push((sensor_point, beacon_point));
    }

    let (input, normalizing_point) = normalize_input(input);

    let max_x = input
        .iter()
        .flat_map(|parts| [parts.0, parts.1])
        .max_by_key(|p| p.x)
        .unwrap()
        .x;
    let max_y = input
        .iter()
        .flat_map(|parts| [parts.0, parts.1])
        .max_by_key(|p| p.y)
        .unwrap()
        .y;

    // println!("{input:?}");
    println!("{normalizing_point:?}");
    println!("{max_x:?}, {max_y:?}");

    let mut grid = Grid::build(max_x, max_y);

    grid.add_sensor_and_beacon(input);

    grid.draw("out.txt");

}

// move every input closer to 0 and POSITIVE returning the normalizing factor
fn normalize_input(input: Vec<(Point, Point)>) -> (Vec<(Coord, Coord)>, Point) {
    let min_x = input
        .iter()
        .flat_map(|parts| [parts.0, parts.1])
        .min_by_key(|p| p.x)
        .unwrap();
    let min_y = input
        .iter()
        .flat_map(|parts| [parts.0, parts.1])
        .min_by_key(|p| p.y)
        .unwrap();
    let normalizing_point = Point {
        x: min_x.x,
        y: min_y.y,
    };

    let normalized = input
        .into_iter()
        .map(|p| (p.0 - normalizing_point, p.1 - normalizing_point))
        .map(|p| (p.0.into(), p.1.into()))
        .collect();

    (normalized, normalizing_point)
}
