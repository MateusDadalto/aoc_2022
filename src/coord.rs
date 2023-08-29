use std::ops::{Sub, Add, Neg};

use crate::point::Point;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Coord {
    pub x: usize,
    pub y: usize,
}

impl Coord {
    pub fn distance(self, rhs: Self) -> usize {
        (self - rhs).size()
    }

    pub fn size(self) -> usize {
        self.x + self.y
    }

    pub fn get_neighbours(self, radius: usize) -> Vec<Coord> {
        let area = radius.pow(2) + (radius + 1).pow(2);
        let mut neighbours = Vec::with_capacity(area);

        // range x from -radius..radius
        // range y from -(radius - x)..(radius - x)
        for x in (radius as isize).wrapping_neg()..=(radius as isize) {
            for y in ((radius - x.unsigned_abs()) as isize).neg()..=((radius - x.unsigned_abs()) as isize) {
                let coord = self.add_signed((x, y).into());

                match coord {
                    Some(c) if c != self => neighbours.push(c),
                    _ => continue,
                }
            }
        }
        // println!("{neighbours:?}");
        neighbours
    }

    fn add_signed(self, rhs: Point) -> Option<Self>{
        let x = self.x.checked_add_signed(rhs.x)?;
        let y = self.y.checked_add_signed(rhs.y)?;

        Some(Coord {x, y})
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

impl From<Point> for Coord {
    fn from(value: Point) -> Self {
        Coord {
            x: value.x as usize,
            y: value.y as usize,
        }
    }
}

impl Sub for Coord {
    type Output = Self;

    fn sub(self, other: Self) -> Self::Output {
        Self {
            x: self.x.abs_diff(other.x),
            y: self.y.abs_diff(other.y),
        }
    }
}

impl Add for Coord {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}
