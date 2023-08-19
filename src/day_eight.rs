use std::collections::HashSet;

use crate::helper;

struct Grid {
    values: Vec<Vec<Tree>>
}

struct Tree {
    height: i8,
    line: usize,
    col: usize,
}

impl Grid {
    fn lines(&self) -> usize {
        self.values.len()
    }

    fn cols(&self) -> usize {
        self.values.get(0).map_or(0, |col| col.len())
    }

    fn visible(&self) -> HashSet<(usize, usize)> {
        // unrealistic estimation that all trees are visible from at least one side
        let mut visibles = HashSet::with_capacity(self.lines() * self.cols());

        visibles.extend(self.visible_from_right());
        visibles.extend(self.visible_from_bottom());
        visibles.extend(self.visible_from_left());
        visibles.extend(self.visible_from_top());

        visibles
    }

    fn visible_from_right(&self) -> Vec<(usize, usize)> {
        self.get_visibilities_horizontal(0..self.lines(), 0..self.cols())
    }

    fn visible_from_left(&self) -> Vec<(usize, usize)> {
        self.get_visibilities_horizontal(0..self.lines(), (0..self.cols()).rev())
    }

    fn visible_from_bottom(&self) -> Vec<(usize, usize)> {
        self.get_visibilities_vertical((0..self.lines()).rev(), 0..self.cols())
    }

    fn visible_from_top(&self) -> Vec<(usize, usize)> {
        self.get_visibilities_vertical(0..self.lines(), 0..self.cols())
    }

    fn get_visibilities_horizontal<T, U>(&self, line_range: T, column_range: U) -> Vec<(usize, usize)> 
    where
        T: Iterator<Item = usize>,
        U: Iterator<Item = usize> + Clone
    {
        let mut visible = Vec::with_capacity(self.lines());
        
        for line in line_range {
            let mut max_height: i8 = i8::MIN;

            for col in column_range.clone() {
                let tree = &self.values[line][col];
                if tree.height > max_height {
                    max_height = tree.height;
                    visible.push(tree.position());
                }
            }
        }

        visible
    }

    fn get_visibilities_vertical<T, U>(&self, line_range: U, column_range: T) -> Vec<(usize, usize)> 
    where
        T: Iterator<Item = usize>,
        U: Iterator<Item = usize> + Clone
    {
        let mut visible = Vec::with_capacity(self.lines());
        
        for col in column_range {
            let mut max_height: i8 = i8::MIN;

            for line in line_range.clone() {
                let tree = &self.values[line][col];
                if tree.height > max_height {
                    max_height = tree.height;
                    visible.push(tree.position());
                }
            }
        }

        visible
    }
}

impl Tree {
    fn position(&self) -> (usize, usize) {
        (self.line, self.col)
    }
}

pub fn solve() {
    let lines = helper::get_file_lines_iter("inputs/day_eight.txt");
    let total_lines = lines.size_hint().0;
    let mut grid = Grid {values: Vec::with_capacity(total_lines)};

    // build grid
    for (line, text) in lines.enumerate() {
        let cols = text.as_deref().unwrap().chars();
        let total_cols = text.as_deref().unwrap().len();

        let mut columns = Vec::with_capacity(total_cols);
        
        for (col, char) in cols.enumerate() {
            let tree = Tree {
                col,
                line,
                height: char.to_digit(10).unwrap() as i8
            };

            columns.push(tree);
        }

        grid.values.push(columns);
    }
    let visible_trees = grid.visible();

    println!("Day 8 part 1: {}", visible_trees.len());

    _draw_grid(&grid, &visible_trees);
}

fn _draw_grid(grid: &Grid, visible_trees: &HashSet<(usize, usize)>) {
    for l in grid.values.iter() {
        let mut line = String::new();

        for c in l.iter() {
            if visible_trees.contains(&c.position()) {
                line = format!("{line}\x1b[41m{}\x1b[0m", c.height)
            } else {
                line = format!("{line}{}", c.height)
            }
        }

        println!("{line}");
    }
}
