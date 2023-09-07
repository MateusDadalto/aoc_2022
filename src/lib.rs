const EXAMPLE: &str = include_str!("../inputs/example.txt");
const INPUT: &str = include_str!("../inputs/input.txt");

pub fn solve() {
    let example: Vec<Cube> = EXAMPLE.lines().map(Cube::parse).collect();
    let input: Vec<Cube> = INPUT.lines().map(Cube::parse).collect();


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
    fn corners(&self) -> [Coord; 8] {
        [
            self.coord + (0, 0, 0).into(), // left-bottom-front (origin)
            self.coord + (1, 0, 0).into(), // right-bottom-front
            self.coord + (1, 0, 1).into(), // right-bottom-back
            self.coord + (0, 0, 1).into(), // left-bottom-back
            self.coord + (0, 1, 0).into(), // left-top-front
            self.coord + (0, 1, 1).into(), // left-top-back
            self.coord + (1, 1, 0).into(), // right-top-front
            self.coord + (1, 1, 1).into(), // right-top-back
        ]
    }

}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
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
