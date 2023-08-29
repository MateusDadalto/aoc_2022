use std::ops::Sub;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Point {
    pub x: isize,
    pub y: isize,
}

impl Sub for Point {
    type Output = Self;

    fn sub(self, other: Self) -> Self::Output {
        Self {
            x: self.x - other.x,
            y: self.y - other.y,
        }
    }
}

impl Point {
    pub fn parse(text: &str) -> Self {
        let x = text.find("x=").unwrap();
        let x: String = text[x + 2..].chars().take_while(|c| *c != ',').collect();

        let y = text.find("y=").unwrap();
        let y = text[y + 2..].to_string();

        Point {
            x: x.parse().unwrap(),
            y: y.parse().unwrap(),
        }
    }
}

impl From<(isize, isize)> for Point {
    fn from(value: (isize, isize)) -> Self {
        Point {
            x: value.0,
            y: value.1,
        }
    }
}