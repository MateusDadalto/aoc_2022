use std::{fmt::{Display, Debug}, collections::HashSet, hash::Hash, f32::consts::E};

mod helper;

#[derive(Clone, Copy)]
enum Step {
    Start,
    End,
    Middle(u8)
}


impl Display for Step {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Step::Start => f.write_str("S"),
            Step::End => f.write_str("E"),
            Step::Middle(x) => f.write_str(((x + b'a') as char).to_string().as_str()),
        }
    }
}

impl Step {
    fn parse(c: char) -> Step {
        match c {
            'S' => Self::Start,
            'E' => Self::End,
            _ => Self::Middle((c as u8) - b'a')
        }
    }

    fn elevation(self) -> u8 {
        match self {
            Step::Start => 0,
            Step::End => b'z' - b'a',
            Step::Middle(x) => x,
        }
    }
}

struct Grid {
    height: usize,
    width: usize,
    steps: Vec<Step>,
    unvisited: HashSet<GridCoord>,
}

impl Grid {
    fn build(lines: Vec<String>) -> Self{
        let height = lines.len();
        let width = lines[0].len();

        let steps: Vec<Step> = lines.iter()
            .flat_map(|line| line.chars())
            .map(|c| Step::parse(c))
            .collect();

        let unvisited: HashSet<GridCoord> = steps.iter()
            .enumerate()
            .map(|(index, _)| GridCoord::from_index(index, width))
            .collect();

        Grid { height, width, steps, unvisited }
    }

    fn get(&self, coord: GridCoord) -> Option<Step> {
        if coord.x > self.height {
            return None;
        } else if coord.y > self.width {
            return None;
        }

        self.steps.get(coord.x*self.width + coord.y).cloned()
    }

    fn draw(&self, steps: &HashSet<GridCoord>) {
        for i in 0..self.height {
            let mut line = String::new();

            for j in 0..self.width {
                let element = self.get((i, j).into()).unwrap();
                if steps.contains(&(i,j).into()) {
                    line = format!("{}\x1b[41m{}\x1b[0m", line, element);
                } else {
                    line = format!("{}{}", line, element);
                }
            }
            println!("{line}");
        }
    }
    
    fn get_neighbors(&self, coord: GridCoord) -> Vec<GridCoord> {
        let diff: [(isize, isize); 4] = [(-1, 0), (1, 0), (0,-1), (0, 1)];

        diff.into_iter()
            .filter(|(l, c)| {
                let line: Option<usize> = coord.x.checked_add_signed(l.clone());
                let col: Option<usize> = coord.y.checked_add_signed(c.clone());

                line.is_some_and(|n| n < self.height) && col.is_some_and(|n| n < self.width)
            })
            .map(|(l, c)| (coord.x.checked_add_signed(l).unwrap(), coord.y.checked_add_signed(c).unwrap()).into())
            .collect()
    }

    fn get_starting_step(&self) -> GridCoord {
        self.steps.iter().enumerate().find_map(|(i, s)| match s {
            Step::Start => Some(GridCoord::from_index(i, self.width)),
            _ => None,
        }).unwrap()
    }

    fn get_ending_step(&self) -> GridCoord {
        self.steps.iter().enumerate().find_map(|(i, s)| match s {
            Step::End => Some(GridCoord::from_index(i, self.width)),
            _ => None,
        }).unwrap()
    }

}

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
struct GridCoord {
    x: usize,
    y: usize,
}

impl From<(usize, usize)> for GridCoord {
    fn from(value: (usize, usize)) -> Self {
        GridCoord { x: value.0, y: value.1 }
    }
}

impl GridCoord {
    fn from_index(index: usize, grid_width: usize) -> Self {
        GridCoord { 
            x: index/grid_width,
            y: index%grid_width 
        }
    }
}

impl Debug for GridCoord {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_tuple("").field(&self.x).field(&self.y).finish()
    }
}

pub fn solve() {
    let lines: Vec<String> = helper::get_file_lines_iter("inputs/input.txt").map(|r| r.unwrap()).collect();

    let mut grid = Grid::build(lines);

    grid.draw(&HashSet::from_iter([grid.get_starting_step()]));

    let mut current_steps = HashSet::new();
    current_steps.insert(grid.get_starting_step());

    let end = grid.get_ending_step();
    let mut n_steps = 0;
    while !current_steps.contains(&end) {
        let mut next_steps: HashSet<GridCoord> = HashSet::new();

        for step_coord in current_steps {
            for next_coord in grid.get_neighbors(step_coord) {
                if grid.unvisited.contains(&next_coord) {
                    let curr = grid.get(step_coord).unwrap();
                    let next = grid.get(next_coord).unwrap();

                    match next.elevation().checked_sub(curr.elevation()) {
                        Some(x) if x > 1 => (),
                        _ => {
                            grid.unvisited.remove(&next_coord);
                            next_steps.insert(next_coord);
                        },
                    }
                }
            }
        }

        current_steps = next_steps;
        n_steps += 1;
        // grid.draw(&current_steps);
    }

    println!("Day 12 part 1: {n_steps}");
}