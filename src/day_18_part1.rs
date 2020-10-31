/*
    --- Day 18: Settlers of The North Pole ---
    On the outskirts of the North Pole base construction project, many Elves are collecting lumber.

    The lumber collection area is 50 acres by 50 acres; each acre can be either open ground (.), trees (|), or a lumberyard (#). You take a scan of the area (your puzzle input).

    Strange magic is at work here: each minute, the landscape looks entirely different. In exactly one minute, an open acre can fill with trees, a wooded acre can be converted to a lumberyard, or a lumberyard can be cleared to open ground (the lumber having been sent to other projects).

    The change to each acre is based entirely on the contents of that acre as well as the number of open, wooded, or lumberyard acres adjacent to it at the start of each minute. Here, "adjacent" means any of the eight acres surrounding that acre. (Acres on the edges of the lumber collection area might have fewer than eight adjacent acres; the missing acres aren't counted.)

    In particular:

    An open acre will become filled with trees if three or more adjacent acres contained trees. Otherwise, nothing happens.
    An acre filled with trees will become a lumberyard if three or more adjacent acres were lumberyards. Otherwise, nothing happens.
    An acre containing a lumberyard will remain a lumberyard if it was adjacent to at least one other lumberyard and at least one acre containing trees. Otherwise, it becomes open.
    These changes happen across all acres simultaneously, each of them using the state of all acres at the beginning of the minute and changing to their new form by the end of that same minute. Changes that happen during the minute don't affect each other.

    For example, suppose the lumber collection area is instead only 10 by 10 acres with this initial configuration:

    Initial state:
    .#.#...|#.
    .....#|##|
    .|..|...#.
    ..|#.....#
    #.#|||#|#|
    ...#.||...
    .|....|...
    ||...#|.#|
    |.||||..|.
    ...#.|..|.

    After 1 minute:
    .......##.
    ......|###
    .|..|...#.
    ..|#||...#
    ..##||.|#|
    ...#||||..
    ||...|||..
    |||||.||.|
    ||||||||||
    ....||..|.

    After 2 minutes:
    .......#..
    ......|#..
    .|.|||....
    ..##|||..#
    ..###|||#|
    ...#|||||.
    |||||||||.
    ||||||||||
    ||||||||||
    .|||||||||

    After 3 minutes:
    .......#..
    ....|||#..
    .|.||||...
    ..###|||.#
    ...##|||#|
    .||##|||||
    ||||||||||
    ||||||||||
    ||||||||||
    ||||||||||

    After 4 minutes:
    .....|.#..
    ...||||#..
    .|.#||||..
    ..###||||#
    ...###||#|
    |||##|||||
    ||||||||||
    ||||||||||
    ||||||||||
    ||||||||||

    After 5 minutes:
    ....|||#..
    ...||||#..
    .|.##||||.
    ..####|||#
    .|.###||#|
    |||###||||
    ||||||||||
    ||||||||||
    ||||||||||
    ||||||||||

    After 6 minutes:
    ...||||#..
    ...||||#..
    .|.###|||.
    ..#.##|||#
    |||#.##|#|
    |||###||||
    ||||#|||||
    ||||||||||
    ||||||||||
    ||||||||||

    After 7 minutes:
    ...||||#..
    ..||#|##..
    .|.####||.
    ||#..##||#
    ||##.##|#|
    |||####|||
    |||###||||
    ||||||||||
    ||||||||||
    ||||||||||

    After 8 minutes:
    ..||||##..
    ..|#####..
    |||#####|.
    ||#...##|#
    ||##..###|
    ||##.###||
    |||####|||
    ||||#|||||
    ||||||||||
    ||||||||||

    After 9 minutes:
    ..||###...
    .||#####..
    ||##...##.
    ||#....###
    |##....##|
    ||##..###|
    ||######||
    |||###||||
    ||||||||||
    ||||||||||

    After 10 minutes:
    .||##.....
    ||###.....
    ||##......
    |##.....##
    |##.....##
    |##....##|
    ||##.####|
    ||#####|||
    ||||#|||||
    ||||||||||
    After 10 minutes, there are 37 wooded acres and 31 lumberyards. Multiplying the number of wooded acres by the number of lumberyards gives the total resource value after ten minutes: 37 * 31 = 1147.

    What will the total resource value of the lumber collection area be after 10 minutes?
*/

use crate::common::Point;
use std::collections::HashMap;
use std::fmt;

#[derive(Clone, Copy, Eq, PartialEq)]
enum Tile {
    OpenGround,
    Trees,
    Lumberyard,
}

impl Tile {
    fn from_char(c: char) -> Self {
        match c {
            '.' => Self::OpenGround,
            '|' => Self::Trees,
            '#' => Self::Lumberyard,
            _ => panic!("Unknown tile: {}", c),
        }
    }

    fn to_char(&self) -> char {
        match *self {
            Self::OpenGround => '.',
            Self::Trees => '|',
            Self::Lumberyard => '#',
        }
    }
}

struct Construction {
    tiles: HashMap<Point, Tile>,
    size: usize,
}

impl Construction {
    fn from_string(input: &str) -> Self {
        let mut tiles = HashMap::new();
        let mut p = Point::new();
        for line in input.trim().lines() {
            for c in line.chars() {
                tiles.insert(p, Tile::from_char(c));
                p.x += 1;
            }
            p.x = 0;
            p.y += 1;
        }

        let range = Point::get_range(tiles.keys()).unwrap();
        let size = (range.0).1 - (range.0).0 + 1;
        assert_eq!(size, (range.1).1 - (range.1).0 + 1);

        Self {
            tiles,
            size: size as usize,
        }
    }

    fn determine_next_tile(&self, point: &Point) -> Option<Tile> {
        let adjacents = point.adjacents();
        let trees_count = adjacents
            .iter()
            .map(|adj| self.tiles.get(adj))
            .filter(|&t| t == Some(&Tile::Trees))
            .count();
        let lumberyard_count = adjacents
            .iter()
            .map(|adj| self.tiles.get(adj))
            .filter(|&t| t == Some(&Tile::Lumberyard))
            .count();
        match self.tiles.get(&point) {
            Some(Tile::OpenGround) => {
                if trees_count >= 3 {
                    return Some(Tile::Trees);
                }
            }
            Some(Tile::Trees) => {
                if lumberyard_count >= 3 {
                    return Some(Tile::Lumberyard);
                }
            }
            Some(Tile::Lumberyard) => {
                if lumberyard_count == 0 || trees_count == 0 {
                    return Some(Tile::OpenGround);
                }
            }
            None => panic!("Unexpected empty tile at {}", point),
        }

        None
    }

    fn sim(&mut self, minutes: u32) {
        let mut new_tiles: HashMap<Point, Tile> = HashMap::new();
        for _ in 0..minutes {
            new_tiles.clear();
            for &point in self.tiles.keys() {
                if let Some(t) = self.determine_next_tile(&point) {
                    new_tiles.insert(point, t);
                }
            }
            self.tiles.extend(&new_tiles);
        }
    }

    fn resource_value(&self) -> u32 {
        let trees_count = self
            .tiles
            .values()
            .filter(|&tile| tile == &Tile::Trees)
            .count() as u32;
        let lumberyard_count = self
            .tiles
            .values()
            .filter(|&tile| tile == &Tile::Lumberyard)
            .count() as u32;
        trees_count * lumberyard_count
    }
}

impl fmt::Display for Construction {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let range = Point::get_range(self.tiles.keys()).unwrap();
        for y in (range.1).0..=(range.1).1 {
            for x in (range.0).0..=(range.0).1 {
                if let Some(tile) = self.tiles.get(&Point { x, y }) {
                    write!(f, "{}", tile.to_char())?;
                } else {
                    write!(f, " ")?;
                }
            }
            writeln!(f)?;
        }

        Ok(())
    }
}

#[aoc(day18, part1)]
pub fn solve(input: &str) -> u32 {
    let mut construction = Construction::from_string(input);
    construction.sim(10);
    //println!("{}", construction);

    let resource_value = construction.resource_value();
    println!("Resource value: {}", resource_value);
    assert_eq!(resource_value, 653184);
    resource_value
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_sim() {
        let input = "
.#.#...|#.
.....#|##|
.|..|...#.
..|#.....#
#.#|||#|#|
...#.||...
.|....|...
||...#|.#|
|.||||..|.
...#.|..|.";
        let mut construction = Construction::from_string(input);

        let result = "
.......##.
......|###
.|..|...#.
..|#||...#
..##||.|#|
...#||||..
||...|||..
|||||.||.|
||||||||||
....||..|.";
        construction.sim(1);
        assert_eq!(construction.to_string().trim(), result.trim());

        let result = "
.......#..
......|#..
.|.|||....
..##|||..#
..###|||#|
...#|||||.
|||||||||.
||||||||||
||||||||||
.|||||||||";
        construction.sim(1);
        assert_eq!(construction.to_string().trim(), result.trim());

        let result = "
.......#..
....|||#..
.|.||||...
..###|||.#
...##|||#|
.||##|||||
||||||||||
||||||||||
||||||||||
||||||||||";
        construction.sim(1);
        assert_eq!(construction.to_string().trim(), result.trim());

        let result = "
.....|.#..
...||||#..
.|.#||||..
..###||||#
...###||#|
|||##|||||
||||||||||
||||||||||
||||||||||
||||||||||";
        construction.sim(1);
        assert_eq!(construction.to_string().trim(), result.trim());

        let result = "
....|||#..
...||||#..
.|.##||||.
..####|||#
.|.###||#|
|||###||||
||||||||||
||||||||||
||||||||||
||||||||||";
        construction.sim(1);
        assert_eq!(construction.to_string().trim(), result.trim());

        let result = "
...||||#..
...||||#..
.|.###|||.
..#.##|||#
|||#.##|#|
|||###||||
||||#|||||
||||||||||
||||||||||
||||||||||";
        construction.sim(1);
        assert_eq!(construction.to_string().trim(), result.trim());

        let result = "
...||||#..
..||#|##..
.|.####||.
||#..##||#
||##.##|#|
|||####|||
|||###||||
||||||||||
||||||||||
||||||||||";
        construction.sim(1);
        assert_eq!(construction.to_string().trim(), result.trim());

        let result = "
..||||##..
..|#####..
|||#####|.
||#...##|#
||##..###|
||##.###||
|||####|||
||||#|||||
||||||||||
||||||||||";
        construction.sim(1);
        assert_eq!(construction.to_string().trim(), result.trim());

        let result = "
..||###...
.||#####..
||##...##.
||#....###
|##....##|
||##..###|
||######||
|||###||||
||||||||||
||||||||||";
        construction.sim(1);
        assert_eq!(construction.to_string().trim(), result.trim());

        let result = "
.||##.....
||###.....
||##......
|##.....##
|##.....##
|##....##|
||##.####|
||#####|||
||||#|||||
||||||||||";
        construction.sim(1);
        assert_eq!(construction.to_string().trim(), result.trim());

        assert_eq!(construction.resource_value(), 1147);
    }
}
