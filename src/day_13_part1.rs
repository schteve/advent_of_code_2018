/*
    --- Day 13: Mine Cart Madness ---
    A crop of this size requires significant logistics to transport produce, soil, fertilizer, and so on. The Elves are very busy pushing things around in carts on some kind of rudimentary system of tracks they've come up with.

    Seeing as how cart-and-track systems don't appear in recorded history for another 1000 years, the Elves seem to be making this up as they go along. They haven't even figured out how to avoid collisions yet.

    You map out the tracks (your puzzle input) and see where you can help.

    Tracks consist of straight paths (| and -), curves (/ and \), and intersections (+). Curves connect exactly two perpendicular pieces of track; for example, this is a closed loop:

    /----\
    |    |
    |    |
    \----/
    Intersections occur when two perpendicular paths cross. At an intersection, a cart is capable of turning left, turning right, or continuing straight. Here are two loops connected by two intersections:

    /-----\
    |     |
    |  /--+--\
    |  |  |  |
    \--+--/  |
    |     |
    \-----/
    Several carts are also on the tracks. Carts always face either up (^), down (v), left (<), or right (>). (On your initial map, the track under each cart is a straight path matching the direction the cart is facing.)

    Each time a cart has the option to turn (by arriving at any intersection), it turns left the first time, goes straight the second time, turns right the third time, and then repeats those directions starting again with left the fourth time, straight the fifth time, and so on. This process is independent of the particular intersection at which the cart has arrived - that is, the cart has no per-intersection memory.

    Carts all move at the same speed; they take turns moving a single step at a time. They do this based on their current location: carts on the top row move first (acting from left to right), then carts on the second row move (again from left to right), then carts on the third row, and so on. Once each cart has moved one step, the process repeats; each of these loops is called a tick.

    For example, suppose there are two carts on a straight track:

    |  |  |  |  |
    v  |  |  |  |
    |  v  v  |  |
    |  |  |  v  X
    |  |  ^  ^  |
    ^  ^  |  |  |
    |  |  |  |  |
    First, the top cart moves. It is facing down (v), so it moves down one square. Second, the bottom cart moves. It is facing up (^), so it moves up one square. Because all carts have moved, the first tick ends. Then, the process repeats, starting with the first cart. The first cart moves down, then the second cart moves up - right into the first cart, colliding with it! (The location of the crash is marked with an X.) This ends the second and last tick.

    Here is a longer example:

    /->-\
    |   |  /----\
    | /-+--+-\  |
    | | |  | v  |
    \-+-/  \-+--/
    \------/

    /-->\
    |   |  /----\
    | /-+--+-\  |
    | | |  | |  |
    \-+-/  \->--/
    \------/

    /---v
    |   |  /----\
    | /-+--+-\  |
    | | |  | |  |
    \-+-/  \-+>-/
    \------/

    /---\
    |   v  /----\
    | /-+--+-\  |
    | | |  | |  |
    \-+-/  \-+->/
    \------/

    /---\
    |   |  /----\
    | /->--+-\  |
    | | |  | |  |
    \-+-/  \-+--^
    \------/

    /---\
    |   |  /----\
    | /-+>-+-\  |
    | | |  | |  ^
    \-+-/  \-+--/
    \------/

    /---\
    |   |  /----\
    | /-+->+-\  ^
    | | |  | |  |
    \-+-/  \-+--/
    \------/

    /---\
    |   |  /----<
    | /-+-->-\  |
    | | |  | |  |
    \-+-/  \-+--/
    \------/

    /---\
    |   |  /---<\
    | /-+--+>\  |
    | | |  | |  |
    \-+-/  \-+--/
    \------/

    /---\
    |   |  /--<-\
    | /-+--+-v  |
    | | |  | |  |
    \-+-/  \-+--/
    \------/

    /---\
    |   |  /-<--\
    | /-+--+-\  |
    | | |  | v  |
    \-+-/  \-+--/
    \------/

    /---\
    |   |  /<---\
    | /-+--+-\  |
    | | |  | |  |
    \-+-/  \-<--/
    \------/

    /---\
    |   |  v----\
    | /-+--+-\  |
    | | |  | |  |
    \-+-/  \<+--/
    \------/

    /---\
    |   |  /----\
    | /-+--v-\  |
    | | |  | |  |
    \-+-/  ^-+--/
    \------/

    /---\
    |   |  /----\
    | /-+--+-\  |
    | | |  X |  |
    \-+-/  \-+--/
    \------/
    After following their respective paths for a while, the carts eventually crash. To help prevent crashes, you'd like to know the location of the first crash. Locations are given in X,Y coordinates, where the furthest left column is X=0 and the furthest top row is Y=0:

            111
    0123456789012
    0/---\
    1|   |  /----\
    2| /-+--+-\  |
    3| | |  X |  |
    4\-+-/  \-+--/
    5  \------/
    In this example, the location of the first crash is 7,3.
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
}

impl Cart {
    fn from_char(c: char) -> Self {
        Self {
            location: Point::new(),
            orientation: Cardinal::from_arrow(c),
            next_turn: NextTurn::Left,
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

    fn tick(&mut self) -> Option<Point> {
        // Carts must be processed in order. Sort them by row and then by column.
        self.carts.sort_by(|a, b| Point::cmp_y_x(&a.location, &b.location));

        for i in 0..self.carts.len() {
            let next_point = self.carts[i].location.step(self.carts[i].orientation, 1);
            self.carts[i].location = next_point;

            for j in 0..self.carts.len() {
                if self.carts[j].location == next_point && j != i {
                    // Crash!
                    return Some(next_point);
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

        None
    }

    fn run_until_first_crash(&mut self) -> Point {
        loop {
            if let Some(crash_site) = self.tick() {
                return crash_site;
            }
        }
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

#[aoc(day13, part1)]
pub fn solve(input: &str) -> Point {
    let mut track_map = TrackMap::from_string(input);
    //println!("{}", track_map);
    let crash_site = track_map.run_until_first_crash();
    println!("Crash site: {}", crash_site);
    assert_eq!(crash_site, Point { x: 103, y: 85 });
    crash_site
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_run_until_first_crash() {
        let input = r"/->-\
|   |  /----\
| /-+--+-\  |
| | |  | v  |
\-+-/  \-+--/
    \------/";
        let mut track_map = TrackMap::from_string(input);
        let crash_site = track_map.run_until_first_crash();
        assert_eq!(crash_site, Point { x: 7, y: 3 });
    }
}
