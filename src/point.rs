use std::ops::Sub;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
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

    pub fn size(self) -> usize {
        self.x.unsigned_abs() + self.y.unsigned_abs()
    }

    pub fn is_in_radius(self, p: Point, r: usize) -> bool{
        (self-p).size() <= r
    }

    pub fn distance(self, rhs: Self) -> usize {
        (self - rhs).size()
    }

    pub fn range_x(start_x: isize, end_x:isize, y: isize) -> Vec<Self> {
        (start_x..=end_x)
            .map(move |x| {
                Point {x, y}
            }).collect()
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