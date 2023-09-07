use std::collections::BTreeSet;

const EXAMPLE: &str = include_str!("../inputs/example.txt");
const INPUT: &str = include_str!("../inputs/input.txt");

pub fn solve() {
    let example: Vec<Cube> = EXAMPLE.lines().map(Cube::parse).collect();
    let input: Vec<Cube> = INPUT.lines().map(Cube::parse).collect();

    let mut surfaces = BTreeSet::new();

    for cube in input {
        let cube_surfaces = BTreeSet::from(cube.surfaces());
        surfaces = surfaces
            .symmetric_difference(&cube_surfaces)
            .cloned()
            .collect();
    }

    println!("{}", surfaces.len());
}

// cord are its left-bottom-back coords, meaning it fills (x,y,z) to (x+1, y+1, z+1) cubic space
struct Cube {
    coord: Coord,
}

impl Cube {
    fn parse(line: &str) -> Self {
        let numbers: Vec<&str> = line.split(',').collect();

        if numbers.len() != 3 {
            panic!("wrong input");
        }

        Cube {
            coord: Coord {
                x: numbers[0].parse().unwrap(),
                y: numbers[1].parse().unwrap(),
                z: numbers[2].parse().unwrap(),
            },
        }
    }

    // every cube is defined by its corners, which are its origin + the corners of a unit cube in (0,0,0);
    // fn corners(&self) -> [Coord; 8] {
    //     [
    //         self.coord + (0, 0, 0).into(), // left-bottom-front (origin)
    //         self.coord + (1, 0, 0).into(), // right-bottom-front
    //         self.coord + (1, 0, 1).into(), // right-bottom-back
    //         self.coord + (0, 0, 1).into(), // left-bottom-back
    //         self.coord + (0, 1, 0).into(), // left-top-front
    //         self.coord + (0, 1, 1).into(), // left-top-back
    //         self.coord + (1, 1, 0).into(), // right-top-front
    //         self.coord + (1, 1, 1).into(), // right-top-back
    //     ]
    // }

    fn surfaces(&self) -> Surfaces {
        let mut surfaces = BTreeSet::new();
        surfaces.insert(BTreeSet::from([
            self.coord + (0, 0, 0).into(),
            self.coord + (1, 0, 0).into(),
            self.coord + (1, 1, 0).into(),
            self.coord + (0, 1, 0).into(),
        ]));
        surfaces.insert(BTreeSet::from([
            self.coord + (0, 0, 0).into(),
            self.coord + (1, 0, 0).into(),
            self.coord + (1, 1, 0).into(),
            self.coord + (0, 1, 0).into(),
        ])); // front
        surfaces.insert(BTreeSet::from([
            self.coord + (0, 0, 1).into(),
            self.coord + (1, 0, 1).into(),
            self.coord + (1, 1, 1).into(),
            self.coord + (0, 1, 1).into(),
        ])); // back
        surfaces.insert(BTreeSet::from([
            self.coord + (0, 0, 0).into(),
            self.coord + (1, 0, 0).into(),
            self.coord + (1, 0, 1).into(),
            self.coord + (0, 0, 1).into(),
        ])); // bottom
        surfaces.insert(BTreeSet::from([
            self.coord + (0, 1, 0).into(),
            self.coord + (1, 1, 0).into(),
            self.coord + (1, 1, 1).into(),
            self.coord + (0, 1, 1).into(),
        ])); // top
        surfaces.insert(BTreeSet::from([
            self.coord + (0, 0, 0).into(),
            self.coord + (0, 1, 0).into(),
            self.coord + (0, 1, 1).into(),
            self.coord + (0, 0, 1).into(),
        ])); // left
        surfaces.insert(BTreeSet::from([
            self.coord + (1, 0, 0).into(),
            self.coord + (1, 0, 1).into(),
            self.coord + (1, 1, 0).into(),
            self.coord + (1, 1, 1).into(),
        ])); // right

        surfaces
    }
}

type Surfaces = BTreeSet<BTreeSet<Coord>>;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
struct Coord {
    x: usize,
    y: usize,
    z: usize,
}

impl std::ops::Add for Coord {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Coord {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
        }
    }
}

impl From<(usize, usize, usize)> for Coord {
    fn from(value: (usize, usize, usize)) -> Self {
        Coord {
            x: value.0,
            y: value.1,
            z: value.2,
        }
    }
}
