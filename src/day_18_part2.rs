/*
    --- Part Two ---
    This important natural resource will need to last for at least thousands of years. Are the Elves collecting this lumber sustainably?

    What will the total resource value of the lumber collection area be after 1000000000 minutes?
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
            Self::Trees      => '|',
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
            tiles: tiles,
            size: size as usize,
        }
    }

    fn determine_next_tile(&self, point: &Point) -> Option<Tile> {
        let adjacents = point.adjacents();
        let trees_count = adjacents.iter()
                    .map(|adj| self.tiles.get(adj))
                    .filter(|&t| t == Some(&Tile::Trees))
                    .count();
        let lumberyard_count = adjacents.iter()
                    .map(|adj| self.tiles.get(adj))
                    .filter(|&t| t == Some(&Tile::Lumberyard))
                    .count();
        match self.tiles.get(&point) {
            Some(Tile::OpenGround) => {
                if trees_count >= 3 {
                    return Some(Tile::Trees);
                }
            },
            Some(Tile::Trees) => {
                if lumberyard_count >= 3 {
                    return Some(Tile::Lumberyard);
                }
            },
            Some(Tile::Lumberyard) => {
                if lumberyard_count == 0 || trees_count == 0 {
                    return Some(Tile::OpenGround);
                }
            },
            None => panic!("Unexpected empty tile at {}", point),
        }

        None
    }

    fn sim(&mut self, minutes: u32) {
        let mut states: HashMap<String, u32> = HashMap::new();
        let mut new_tiles: HashMap<Point, Tile> = HashMap::new();
        let mut i = 0;
        while i < minutes {
            if let Some(old_minute) = states.insert(self.to_string(), i) {
                let period = i - old_minute;
                let remaining = minutes - i;
                let jump_cycles = remaining / period;
                i += jump_cycles * period;
            }

            new_tiles.clear();
            for &point in self.tiles.keys() {
                if let Some(t) = self.determine_next_tile(&point) {
                    new_tiles.insert(point, t);
                }
            }
            self.tiles.extend(&new_tiles);

            i += 1;
        }
    }

    fn resource_value(&self) -> u32 {
        let trees_count = self.tiles.values()
            .filter(|&tile| tile == &Tile::Trees)
            .count() as u32;
        let lumberyard_count = self.tiles.values()
            .filter(|&tile| tile == &Tile::Lumberyard)
            .count() as u32;
        trees_count * lumberyard_count
    }
}

impl fmt::Display for Construction {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let range = Point::get_range(self.tiles.keys()).unwrap();
        for y in (range.1).0 ..= (range.1).1 {
            for x in (range.0).0 ..= (range.0).1 {
                if let Some(tile) = self.tiles.get(&Point { x, y }) {
                    write!(f, "{}", tile.to_char())?;
                } else {
                    write!(f, " ")?;
                }
            }
            writeln!(f, "")?;
        }

        Ok(())
    }
}

#[aoc(day18, part2)]
pub fn solve(input: &str) -> u32 {
    let mut construction = Construction::from_string(input);
    construction.sim(1_000_000_000);
    //println!("{}", construction);

    let resource_value = construction.resource_value();
    println!("Resource value: {}", resource_value);
    assert_eq!(resource_value, 169106);
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
