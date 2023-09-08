const EXAMPLE: &str = include_str!("../inputs/example.txt");
const INPUT: &str = include_str!("../inputs/input.txt");

pub fn solve() {
    let example: Vec<Cube> = EXAMPLE.lines().map(Cube::parse).collect();
    let input: Vec<Cube> = INPUT.lines().map(Cube::parse).collect();
    let mut total = 0;

    for cube in input.iter() {
        let exposed_surfaces = 6 - input.iter().filter(|&c| cube.is_neighboor(c.clone())).count();

        total += exposed_surfaces;
    }

    println!("{total}");
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Cube(i32, i32, i32);

impl Cube {
    fn parse(line: &str) -> Self {
        let numbers: Vec<&str> = line.split(',').collect();

        if numbers.len() != 3 {
            panic!("wrong input");
        }

        Cube(
            numbers[0].parse().unwrap(),
            numbers[1].parse().unwrap(),
            numbers[2].parse().unwrap(),
        )
    }

    fn is_neighboor(self, other: Self) -> bool {
        let neighboors = [
            self + Cube(1, 0, 0),
            self + Cube(-1, 0, 0),
            self + Cube(0, 1, 0),
            self + Cube(0, -1, 0),
            self + Cube(0, 0, 1),
            self + Cube(0, 0, -1),
        ];

        neighboors.contains(&other)
    }
}

impl std::ops::Add for Cube {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Cube(self.0 + rhs.0, self.1 + rhs.1, self.2 + rhs.2)
    }
}
