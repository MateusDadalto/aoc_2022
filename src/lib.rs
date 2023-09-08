use std::{ops::RangeInclusive, collections::HashSet};

const EXAMPLE: &str = include_str!("../inputs/example.txt");
const INPUT: &str = include_str!("../inputs/input.txt");

pub fn solve() {
    // let input: Vec<Cube> = EXAMPLE.lines().map(Cube::parse).collect();
    let input: Vec<Cube> = INPUT.lines().map(Cube::parse).collect();
    let input_set: HashSet<Cube> = HashSet::from_iter(input.iter().cloned());
    let mut total = input.len() * 6;

    let mut max_x = 0;
    let mut max_y = 0;
    let mut max_z = 0;

    for (i, cube) in input.iter().enumerate() {
        max_x = max_x.max(cube.0);
        max_y = max_y.max(cube.1);
        max_z = max_z.max(cube.2);

        let exposed_surfaces = input
            .iter()
            .skip(i)
            .filter(|&c| cube.is_neighbor(c.clone()))
            .count();

        total -= 2*exposed_surfaces;
    }
    
    println!("{total}");

    // with bounds starting with 0 i've missed the answer by 2
    // meaning I needed more space to search in the lower border
    // indeed when I checked my input the lowest y and z were 0
    // shame on me
    let bounds = Bounds (
        -1..=max_x+1,
        -1..=max_y+1,
        -1..=max_z+1
    );

    let mut search_stack = Vec::new();
    let mut seen_stack = Vec::new();
    search_stack.push(Cube(0,0,0));

    let mut outside_surface = 0;
    
    while let Some(cube) = search_stack.pop() {

        if seen_stack.contains(&cube) || !bounds.contains(cube) {
            continue;
        }


        for neighbor in cube.get_neighbors() {
            if seen_stack.contains(&neighbor) {
                continue;
            }

            if input_set.contains(&neighbor) {
                outside_surface += 1;
                continue;
            }

            // push neighbors that are not lava or have not been seen yet
            search_stack.push(neighbor);
        }

        seen_stack.push(cube);
    }

    // println!("{} {} {}",max_x, max_y, max_z);
    println!("{outside_surface}");
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
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

    fn is_neighbor(self, other: Self) -> bool {
        self.0.abs_diff(other.0) + self.1.abs_diff(other.1) + self.2.abs_diff(other.2) == 1
    }

    fn get_neighbors(self) -> [Cube; 6] {
        [
            self + (1, 0, 0),
            self + (-1, 0, 0),
            self + (0, 1, 0),
            self + (0, -1, 0),
            self + (0, 0, 1),
            self + (0, 0, -1)
        ]
    }
}

impl std::ops::Add<(i32, i32, i32)> for Cube {
    type Output = Self;

    fn add(self, rhs: (i32, i32, i32)) -> Self::Output {
        Cube (
            self.0 + rhs.0,
            self.1 + rhs.1,
            self.2 + rhs.2,
        )
    }
}

struct Bounds (
    RangeInclusive<i32>,
    RangeInclusive<i32>,
    RangeInclusive<i32>,
);

impl Bounds {
    fn contains(&self, cube: Cube) -> bool {
        self.0.contains(&cube.0) &&
        self.1.contains(&cube.1) &&
        self.2.contains(&cube.2)
    }
}