
use std::fmt;
use std::ops;

#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
pub struct Point {
    pub x: i32,
    pub y: i32,
}

impl Point {
    pub fn manhattan(a: Point, b: Point) -> u32 {
        let delta = a - b;
        let distance = delta.x.abs() + delta.y.abs();
        distance as u32
    }
}

impl ops::Add<Point> for Point {
    type Output = Point;

    fn add(self, rhs: Point) -> Point {
        Point {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl ops::AddAssign<Point> for Point {
    fn add_assign(&mut self, rhs: Point) {
        *self = *self + rhs;
    }
}

impl ops::Sub<Point> for Point {
    type Output = Point;

    fn sub(self, rhs: Point) -> Point {
        Point {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
        }
    }
}

impl ops::SubAssign<Point> for Point {
    fn sub_assign(&mut self, rhs: Point) {
        *self = *self - rhs;
    }
}

impl fmt::Display for Point {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    fn test_manhattan() {
        let a = Point { x: 0, y: 0 };
        let b = Point { x: 0, y: 0 };
        assert_eq!(Point::manhattan(a, b), 0);

        let a = Point { x: 1, y: 2 };
        let b = Point { x: 3, y: 4 };
        assert_eq!(Point::manhattan(a, b), 4);

        let a = Point { x: -1, y: -2 };
        let b = Point { x: 3, y: 4 };
        assert_eq!(Point::manhattan(a, b), 10);
    }

    #[test]
    fn test_add() {
        let a = Point { x: 0, y: 0 };
        let b = Point { x: 0, y: 0 };
        assert_eq!(a + b, Point { x: 0, y: 0 });

        let a = Point { x: 1, y: 1 };
        let b = Point { x: 2, y: 2 };
        assert_eq!(a + b, Point { x: 3, y: 3 });

        let a = Point { x: 1, y: 1 };
        let b = Point { x: -1, y: -1 };
        assert_eq!(a + b, Point { x: 0, y: 0 });

        let a = Point { x: 1_000_000_000, y: -1_000_000_000 };
        let b = Point { x: -1, y: 1 };
        assert_eq!(a + b, Point { x: 999_999_999, y: -999_999_999 });
    }

    #[test]
    fn test_sub() {
        let a = Point { x: 0, y: 0 };
        let b = Point { x: 0, y: 0 };
        assert_eq!(a - b, Point { x: 0, y: 0 });

        let a = Point { x: 1, y: 1 };
        let b = Point { x: 2, y: 2 };
        assert_eq!(a - b, Point { x: -1, y: -1 });

        let a = Point { x: 1, y: 1 };
        let b = Point { x: -1, y: -1 };
        assert_eq!(a - b, Point { x: 2, y: 2 });

        let a = Point { x: 1_000_000_000, y: -1_000_000_000 };
        let b = Point { x: -1, y: 1 };
        assert_eq!(a - b, Point { x: 1_000_000_001, y: -1_000_000_001 });

        let a = Point { x: 0x7FFFFFFF, y: -0x7FFFFFFF };
        let b = Point { x: 0x7FFFFFFF, y: -0x7FFFFFFF };
        let c = Point { x: 0x7FFFFFFF, y: -0x7FFFFFFF };
        assert_eq!(a - b - c, Point { x: -0x7FFFFFFF, y: 0x7FFFFFFF });
    }
}