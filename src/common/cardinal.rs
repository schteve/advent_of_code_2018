
use std::fmt;

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Cardinal {
    North,
    South,
    East,
    West,
}

impl Cardinal {
    pub fn from_arrow(c: char) -> Self {
        match c {
            '^' => Cardinal::North,
            'v' => Cardinal::South,
            '>' => Cardinal::East,
            '<' => Cardinal::West,
            _ => panic!("Cardinal: invalid input '{}'", c),
        }
    }

    pub fn to_arrow(&self) -> char {
        match *self {
            Self::North => '^',
            Self::South => 'v',
            Self::East  => '>',
            Self::West  => '<',
        }
    }

    pub fn turn(&self, dir: Turn) -> Self {
        match dir {
            Turn::Left => {
                match *self {
                    Self::North => Self::West,
                    Self::South => Self::East,
                    Self::East  => Self::North,
                    Self::West  => Self::South,
                }
            },

            Turn::Right => {
                match *self {
                    Self::North => Self::East,
                    Self::South => Self::West,
                    Self::East  => Self::South,
                    Self::West  => Self::North,
                }
            },
        }
    }

    pub fn opposite(&self) -> Self {
        match *self {
            Self::North => Self::South,
            Self::South => Self::North,
            Self::East  => Self::West,
            Self::West  => Self::East,
        }
    }
}

impl fmt::Display for Cardinal {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            Self::North => write!(f, "North"),
            Self::South => write!(f, "South"),
            Self::East  => write!(f, "East"),
            Self::West  => write!(f, "West"),
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Turn {
    Left,
    Right,
}

impl fmt::Display for Turn {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            Self::Left  => write!(f, "L"),
            Self::Right => write!(f, "R"),
        }
    }
}
