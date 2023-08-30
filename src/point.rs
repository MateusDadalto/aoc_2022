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

    pub fn is_in_radius(self, p: Point, r: usize) -> bool {
        (self - p).size() <= r
    }

    pub fn distance(self, rhs: Self) -> usize {
        (self - rhs).size()
    }

    pub fn outside_points(self, radius: usize) -> Vec<Point> {
        let outside_radius = (radius + 1) as isize;
        let mut points = vec![];
        for x in -outside_radius..=outside_radius {
            let y_top = (outside_radius - x).abs();
            let y_bottom = -y_top;

            let top_point = Point {
                x: self.x + x,
                y: self.y + y_top,
            };
            let bottom_point = Point {
                x: self.x + x,
                y: self.y + y_bottom,
            };

            if top_point.is_in_bound() {
                points.push(top_point);
            }

            if bottom_point.is_in_bound() {
                points.push(bottom_point);
            }
        }

        points
    }

    fn is_in_bound(self) -> bool {
        self.x >= 0 && self.x <= 4_000_000 && self.y >= 0 && self.y <= 4_000_000
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
