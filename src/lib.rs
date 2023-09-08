const EXAMPLE: &str = include_str!("../inputs/example.txt");
const INPUT: &str = include_str!("../inputs/input.txt");

pub fn solve() {
    let example: Vec<Cube> = EXAMPLE.lines().map(Cube::parse).collect();
    let input: Vec<Cube> = INPUT.lines().map(Cube::parse).collect();
    let mut total = input.len() * 6;

    for (i, cube) in input.iter().enumerate() {
        let exposed_surfaces = input
            .iter()
            .skip(i)
            .filter(|&c| cube.is_neighboor(c.clone()))
            .count();

        total -= 2*exposed_surfaces;
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
        self.0.abs_diff(other.0) + self.1.abs_diff(other.1) + self.2.abs_diff(other.2) == 1
    }
}
