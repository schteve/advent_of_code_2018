/*
    --- Part Two ---
    There isn't much you can do to prevent crashes in this ridiculous system. However, by predicting the crashes, the Elves know where to be in advance and instantly remove the two crashing carts the moment any crash occurs.

    They can proceed like this for a while, but eventually, they're going to run out of carts. It could be useful to figure out where the last cart that hasn't crashed will end up.

    For example:

    />-<\
    |   |
    | /<+-\
    | | | v
    \>+</ |
    |   ^
    \<->/

    /---\
    |   |
    | v-+-\
    | | | |
    \-+-/ |
    |   |
    ^---^

    /---\
    |   |
    | /-+-\
    | v | |
    \-+-/ |
    ^   ^
    \---/

    /---\
    |   |
    | /-+-\
    | | | |
    \-+-/ ^
    |   |
    \---/
    After four very expensive crashes, a tick ends with only one cart remaining; its final location is 6,4.

    What is the location of the last cart at the end of the first tick where it is the only cart left?
*/

use crate::common::Cardinal;
use crate::common::Point;
use crate::common::Turn;
use std::collections::HashMap;
use std::fmt;

enum NextTurn {
    Left,
    Straight,
    Right,
}

struct Cart {
    location: Point,
    orientation: Cardinal,
    next_turn: NextTurn,
    crashed: bool,
}

impl Cart {
    fn from_char(c: char) -> Self {
        Self {
            location: Point::new(),
            orientation: Cardinal::from_arrow(c),
            next_turn: NextTurn::Left,
            crashed: false,
        }
    }

    fn set_location(mut self, p: Point) -> Self {
        self.location = p;
        self
    }
}

#[derive(Hash)]
enum Track {
    Horizontal,
    Vertical,
    CornerA, // On a square, this is the upper left corner or the lower right corner
    CornerB, // On a square, this is the upper right corner or the lower left corner
    Intersection,
    Empty,
}

impl Track {
    fn from_char(c: char) -> Self {
        match c {
            '-' | '<' | '>' => Self::Horizontal,
            '|' | '^' | 'v' => Self::Vertical,
            '/' => Self::CornerA,
            '\\' => Self::CornerB,
            '+' => Self::Intersection,
            ' ' => Self::Empty,
            _ => panic!("Unknown input character"),
        }
    }

    fn to_char(&self) -> char {
        match self {
            Self::Horizontal   => '-',
            Self::Vertical     => '|',
            Self::CornerA      => '/',
            Self::CornerB      => '\\',
            Self::Intersection => '+',
            Self::Empty        => ' ',
        }
    }
}

struct TrackMap {
    tracks: HashMap<Point, Track>,
    carts: Vec<Cart>,
}

impl TrackMap {
    fn from_string(input: &str) -> Self {
        let mut tracks = HashMap::new();
        let mut carts = Vec::new();

        let mut p = Point::new();
        for line in input.lines() {
            p.x = 0;
            for c in line.chars() {
                let track = Track::from_char(c);
                tracks.insert(p, track);

                if c == '^' || c == 'v' || c == '>' || c == '<' {
                    let cart = Cart::from_char(c).set_location(p);
                    carts.push(cart);
                }

                p.x += 1;
            }
            p.y += 1;
        }

        Self {
            tracks,
            carts,
        }
    }

    fn tick(&mut self) {
        // Carts must be processed in order. Sort them by row and then by column.
        self.carts.sort_by(|a, b| Point::cmp_y_x(&a.location, &b.location));

        for i in 0..self.carts.len() {
            if self.carts[i].crashed == true {
                // Don't process crashed carts
                continue;
            }

            let next_point = self.carts[i].location.step(self.carts[i].orientation, 1);
            self.carts[i].location = next_point;

            for j in 0..self.carts.len() {
                if self.carts[j].location == next_point && self.carts[j].crashed == false && j != i {
                    // Crash!
                    self.carts[i].crashed = true;
                    self.carts[j].crashed = true;
                }
            }

            let next_track = self.tracks.get(&next_point);
            match next_track {
                Some(Track::Horizontal) => (),
                Some(Track::Vertical) => (),
                Some(Track::CornerA) => {
                    self.carts[i].orientation = match self.carts[i].orientation {
                        Cardinal::North => self.carts[i].orientation.turn(Turn::Right),
                        Cardinal::South => self.carts[i].orientation.turn(Turn::Right),
                        Cardinal::East => self.carts[i].orientation.turn(Turn::Left),
                        Cardinal::West => self.carts[i].orientation.turn(Turn::Left),
                    };
                },
                Some(Track::CornerB) => {
                    self.carts[i].orientation = match self.carts[i].orientation {
                        Cardinal::North => self.carts[i].orientation.turn(Turn::Left),
                        Cardinal::South => self.carts[i].orientation.turn(Turn::Left),
                        Cardinal::East => self.carts[i].orientation.turn(Turn::Right),
                        Cardinal::West => self.carts[i].orientation.turn(Turn::Right),
                    };
                },
                Some(Track::Intersection) => {
                    match self.carts[i].next_turn {
                        NextTurn::Left => {
                            self.carts[i].orientation = self.carts[i].orientation.turn(Turn::Left);
                            self.carts[i].next_turn = NextTurn::Straight;
                        },
                        NextTurn::Straight => self.carts[i].next_turn = NextTurn::Right,
                        NextTurn::Right => {
                            self.carts[i].orientation = self.carts[i].orientation.turn(Turn::Right);
                            self.carts[i].next_turn = NextTurn::Left;
                        },
                    }
                },
                Some(Track::Empty) => panic!("Unexpected empty track: {}", next_point),
                None => panic!("No track found: {}", next_point),
            }
        }
    }

    fn run_until_last_crash(&mut self) -> Point {
        while self.carts.iter().filter(|&cart| cart.crashed == false).count() > 1 {
            self.tick();
        }

        self.carts.iter()
            .filter(|&cart| cart.crashed == false)
            .map(|cart| cart.location)
            .next()
            .unwrap()
    }

    fn get_range(&self) -> ((i32, i32), (i32, i32)) {
        let mut tracks_iter = self.tracks.iter();
        if let Some((point, _track)) = tracks_iter.next() {
            tracks_iter.fold(((point.x, point.x), (point.y, point.y)),
                |(acc_x, acc_y), (p, _)|
                    ((acc_x.0.min(p.x), acc_x.1.max(p.x)),
                     (acc_y.0.min(p.y), acc_y.1.max(p.y))))
        } else {
            ((0, 0), (0, 0))
        }
    }
}

impl fmt::Display for TrackMap {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let range = self.get_range();
        for y in (range.1).0 .. (range.1).1 {
            for x in (range.0).0 .. (range.0).1 {
                let p = Point { x, y };
                if let Some(cart) = self.carts.iter().find(|c| c.location == p) {
                    write!(f, "{}", cart.orientation.to_arrow())?;
                } else if let Some(track) = self.tracks.get(&p) {
                    write!(f, "{}", track.to_char())?;
                } else {
                    write!(f, ".")?;
                }
            }
            writeln!(f)?;
        }
        writeln!(f)
    }
}

#[aoc(day13, part2)]
pub fn solve(input: &str) -> Point {
    let mut track_map = TrackMap::from_string(input);
    //println!("{}", track_map);
    let last_cart = track_map.run_until_last_crash();
    println!("Last cart: {}", last_cart);
    assert_eq!(last_cart, Point { x: 88, y: 64 });
    last_cart
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_run_until_last_crash() {
        let input = r"/>-<\
|   |
| /<+-\
| | | v
\>+</ |
  |   ^
  \<->/";
        let mut track_map = TrackMap::from_string(input);
        let crash_site = track_map.run_until_last_crash();
        assert_eq!(crash_site, Point { x: 6, y: 4 });
    }
}
