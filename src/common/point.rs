
use crate::common::Cardinal;
use std::cmp::Ordering;
use std::fmt;
use std::ops;

#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
pub struct Point {
    pub x: i32,
    pub y: i32,
}

impl Point {
    pub fn new() -> Self {
        Self {
            x: 0,
            y: 0,
        }
    }

    pub fn manhattan(a: Self, b: Self) -> u32 {
        let delta = a - b;
        let distance = delta.x.abs() + delta.y.abs();
        distance as u32
    }

    pub fn cmp_x_y(a: &Self, b: &Self) -> Ordering {
        let compare = a.x.cmp(&b.x);
        if compare == Ordering::Equal {
            a.y.cmp(&b.y)
        } else {
            compare
        }
    }

    pub fn cmp_y_x(a: &Self, b: &Self) -> Ordering {
        let compare = a.y.cmp(&b.y);
        if compare == Ordering::Equal {
            a.x.cmp(&b.x)
        } else {
            compare
        }
    }

    pub fn step(&self, direction: Cardinal, count: i32) -> Self {
        match direction {
            Cardinal::North => *self + (0, -count),
            Cardinal::South => *self + (0, count),
            Cardinal::East => *self + (count, 0),
            Cardinal::West => *self + (-count, 0),
        }
    }

    pub fn orthogonals(&self) -> Vec<Self> {
        vec![*self + (0, -1),
             *self + (0, 1),
             *self + (1, 0),
             *self + (-1, 0)]
    }

    pub fn diagonals(&self) -> Vec<Self> {
        vec![*self + (-1, -1),
             *self + (1, -1),
             *self + (1, 1),
             *self + (-1, 1)]
    }
}

impl ops::Add<Self> for Point {
    type Output = Self;

    fn add(self, rhs: Self) -> Self {
        Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl ops::Add<(i32, i32)> for Point {
    type Output = Self;

    fn add(self, rhs: (i32, i32)) -> Self {
        Self {
            x: self.x + rhs.0,
            y: self.y + rhs.1,
        }
    }
}

impl ops::AddAssign<Self> for Point {
    fn add_assign(&mut self, rhs: Self) {
        *self = *self + rhs;
    }
}

impl ops::AddAssign<(i32, i32)> for Point {
    fn add_assign(&mut self, rhs: (i32, i32)) {
        *self = *self + rhs;
    }
}

impl ops::Sub<Self> for Point {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self {
        Self {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
        }
    }
}

impl ops::Sub<(i32, i32)> for Point {
    type Output = Self;

    fn sub(self, rhs: (i32, i32)) -> Self {
        Self {
            x: self.x - rhs.0,
            y: self.y - rhs.1,
        }
    }
}

impl ops::SubAssign<Self> for Point {
    fn sub_assign(&mut self, rhs: Self) {
        *self = *self - rhs;
    }
}

impl ops::SubAssign<(i32, i32)> for Point {
    fn sub_assign(&mut self, rhs: (i32, i32)) {
        *self = *self - rhs;
    }
}

impl fmt::Display for Point {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}

impl PartialOrd for Point {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Point {
    fn cmp(&self, other: &Self) -> Ordering {
        Self::cmp_y_x(self, other)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
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
    fn test_cmp_x_y() {
        let a = Point { x: 0, y: 0 };
        let b = Point { x: 0, y: 0 };
        assert_eq!(Point::cmp_x_y(&a, &b), Ordering::Equal);

        let a = Point { x: 1, y: 2 };
        let b = Point { x: 3, y: 4 };
        assert_eq!(Point::cmp_x_y(&a, &b), Ordering::Less);

        let a = Point { x: 1, y: 2 };
        let b = Point { x: 3, y: -5 };
        assert_eq!(Point::cmp_x_y(&a, &b), Ordering::Less);

        let a = Point { x: 1, y: 2 };
        let b = Point { x: 1, y: 4 };
        assert_eq!(Point::cmp_x_y(&a, &b), Ordering::Less);

        let a = Point { x: 1, y: 2 };
        let b = Point { x: 1, y: -5 };
        assert_eq!(Point::cmp_x_y(&a, &b), Ordering::Greater);
    }

    #[test]
    fn test_cmp_y_x() {
        let a = Point { x: 0, y: 0 };
        let b = Point { x: 0, y: 0 };
        assert_eq!(Point::cmp_y_x(&a, &b), Ordering::Equal);

        let a = Point { x: 1, y: 2 };
        let b = Point { x: 3, y: 4 };
        assert_eq!(Point::cmp_y_x(&a, &b), Ordering::Less);

        let a = Point { x: 1, y: 2 };
        let b = Point { x: 3, y: -5 };
        assert_eq!(Point::cmp_y_x(&a, &b), Ordering::Greater);

        let a = Point { x: 2, y: 1 };
        let b = Point { x: 4, y: 1 };
        assert_eq!(Point::cmp_y_x(&a, &b), Ordering::Less);

        let a = Point { x: 2, y: 1 };
        let b = Point { x: -5, y: 1 };
        assert_eq!(Point::cmp_y_x(&a, &b), Ordering::Greater);
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