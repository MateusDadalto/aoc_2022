use std::{collections::HashSet, ops::Range, iter::Rev};

use crate::helper;

struct Grid {
    values: Vec<Vec<Tree>>
}

struct Tree {
    height: i8,
    line: usize,
    col: usize,
}

impl Tree {
    
    fn get_view(&self, grid: &Grid) -> Vec<Vec<(usize, usize)>> {
        let mut all_view = vec![];

        all_view.push(self.get_view_horizontal(grid, self.left_view(grid).0, self.left_view(grid).1));
        all_view.push(self.get_view_horizontal(grid, self.right_view(grid).0, self.right_view(grid).1));
        all_view.push(self.get_view_vertical(grid, self.top_view(grid).0, self.top_view(grid).1));
        all_view.push(self.get_view_vertical(grid, self.bottom_view(grid).0, self.bottom_view(grid).1));

        all_view
    }

    fn left_view(&self, _grid: &Grid) -> (usize, Rev<Range<usize>>)
    {
        (self.line, (0..self.col).rev())
    }

    fn right_view(&self, grid: &Grid) -> (usize, Range<usize>) {
        (self.line, self.col+1..grid.cols())
    }

    fn top_view(&self, _grid: &Grid) -> (Rev<Range<usize>>, usize) {
        ((0..self.line).rev(), self.col)
    }

    fn bottom_view(&self, grid: &Grid) -> (Range<usize>, usize) {
        (self.line+1..grid.lines(), self.col)
    }

    fn get_view_horizontal<T>(&self, grid: &Grid, line: usize, cols: T) -> Vec<(usize, usize)>
    where
        T: Iterator<Item = usize>
    {
        let mut visible_trees = vec![];

        for col in cols {
            let tree = &grid.values[line][col];
            visible_trees.push(tree.position());

            if tree.height >= self.height {
                break;
            }
        }

        visible_trees
    }

    fn get_view_vertical<T>(&self, grid: &Grid, lines: T, col: usize) -> Vec<(usize, usize)>
    where
        T: Iterator<Item = usize>
    {
        let mut visible_trees = vec![];

        for line in lines {
            let tree = &grid.values[line][col];
            visible_trees.push(tree.position());
            
            if tree.height >= self.height {
                break;
            }
        }

        visible_trees
    }

    fn get_scenic_score(&self, grid: &Grid) -> usize {
        let all_visible = self.get_view(grid);

        all_visible.iter()
            .map(|direction| direction.len())
            .reduce(|acc, e| acc*e)
            .unwrap_or(1)
    }
}

impl Grid {
    fn lines(&self) -> usize {
        self.values.len()
    }

    fn cols(&self) -> usize {
        self.values.get(0).map_or(0, |col| col.len())
    }

    fn scenic_score(&self, tree: &Tree) {
           
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

    let mut max_scenic_score = 0;
    let mut best_tree: &Tree = &Tree { height: 0, line: 0, col: 0 };

    for tree in grid.values.iter().flatten() {
        let score = tree.get_scenic_score(&grid);

        if max_scenic_score < score {
            max_scenic_score = score;
            best_tree = tree;
        }
    }

    println!("Day 8 part 2: {max_scenic_score}");
    _draw_grid(&grid, best_tree);
}

fn _draw_grid(grid: &Grid, best_tree: &Tree) {
    let visible_trees: Vec<(usize, usize)> = best_tree.get_view(grid).into_iter().flatten().collect();

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
