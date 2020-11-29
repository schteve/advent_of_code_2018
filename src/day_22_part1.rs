/*
    --- Day 22: Mode Maze ---
    This is it, your final stop: the year -483. It's snowing and dark outside; the only light you can see is coming from a small cottage in the distance. You make your way there and knock on the door.

    A portly man with a large, white beard answers the door and invites you inside. For someone living near the North Pole in -483, he must not get many visitors, but he doesn't act surprised to see you. Instead, he offers you some milk and cookies.

    After talking for a while, he asks a favor of you. His friend hasn't come back in a few hours, and he's not sure where he is. Scanning the region briefly, you discover one life signal in a cave system nearby; his friend must have taken shelter there. The man asks if you can go there to retrieve his friend.

    The cave is divided into square regions which are either dominantly rocky, narrow, or wet (called its type). Each region occupies exactly one coordinate in X,Y format where X and Y are integers and zero or greater. (Adjacent regions can be the same type.)

    The scan (your puzzle input) is not very detailed: it only reveals the depth of the cave system and the coordinates of the target. However, it does not reveal the type of each region. The mouth of the cave is at 0,0.

    The man explains that due to the unusual geology in the area, there is a method to determine any region's type based on its erosion level. The erosion level of a region can be determined from its geologic index. The geologic index can be determined using the first rule that applies from the list below:

    The region at 0,0 (the mouth of the cave) has a geologic index of 0.
    The region at the coordinates of the target has a geologic index of 0.
    If the region's Y coordinate is 0, the geologic index is its X coordinate times 16807.
    If the region's X coordinate is 0, the geologic index is its Y coordinate times 48271.
    Otherwise, the region's geologic index is the result of multiplying the erosion levels of the regions at X-1,Y and X,Y-1.
    A region's erosion level is its geologic index plus the cave system's depth, all modulo 20183. Then:

    If the erosion level modulo 3 is 0, the region's type is rocky.
    If the erosion level modulo 3 is 1, the region's type is wet.
    If the erosion level modulo 3 is 2, the region's type is narrow.
    For example, suppose the cave system's depth is 510 and the target's coordinates are 10,10. Using % to represent the modulo operator, the cavern would look as follows:

    At 0,0, the geologic index is 0. The erosion level is (0 + 510) % 20183 = 510. The type is 510 % 3 = 0, rocky.
    At 1,0, because the Y coordinate is 0, the geologic index is 1 * 16807 = 16807. The erosion level is (16807 + 510) % 20183 = 17317. The type is 17317 % 3 = 1, wet.
    At 0,1, because the X coordinate is 0, the geologic index is 1 * 48271 = 48271. The erosion level is (48271 + 510) % 20183 = 8415. The type is 8415 % 3 = 0, rocky.
    At 1,1, neither coordinate is 0 and it is not the coordinate of the target, so the geologic index is the erosion level of 0,1 (8415) times the erosion level of 1,0 (17317), 8415 * 17317 = 145722555. The erosion level is (145722555 + 510) % 20183 = 1805. The type is 1805 % 3 = 2, narrow.
    At 10,10, because they are the target's coordinates, the geologic index is 0. The erosion level is (0 + 510) % 20183 = 510. The type is 510 % 3 = 0, rocky.
    Drawing this same cave system with rocky as ., wet as =, narrow as |, the mouth as M, the target as T, with 0,0 in the top-left corner, X increasing to the right, and Y increasing downward, the top-left corner of the map looks like this:

    M=.|=.|.|=.|=|=.
    .|=|=|||..|.=...
    .==|....||=..|==
    =.|....|.==.|==.
    =|..==...=.|==..
    =||.=.=||=|=..|=
    |.=.===|||..=..|
    |..==||=.|==|===
    .=..===..=|.|||.
    .======|||=|=.|=
    .===|=|===T===||
    =|||...|==..|=.|
    =.=|=.=..=.||==|
    ||=|=...|==.=|==
    |=.=||===.|||===
    ||.|==.|.|.||=||
    Before you go in, you should determine the risk level of the area. For the rectangle that has a top-left corner of region 0,0 and a bottom-right corner of the region containing the target, add up the risk level of each individual region: 0 for rocky regions, 1 for wet regions, and 2 for narrow regions.

    In the cave system above, because the mouth is at 0,0 and the target is at 10,10, adding up the risk level of all regions with an X coordinate from 0 to 10 and a Y coordinate from 0 to 10, this total is 114.

    What is the total risk level for the smallest rectangle that includes 0,0 and the target's coordinates?
*/

use crate::common::modulo;
use crate::common::Point;
use std::collections::HashMap;
use std::fmt;

#[derive(Clone, Copy, Debug, PartialEq)]
enum RegionType {
    Rocky,
    Wet,
    Narrow,
}

impl RegionType {
    fn from_erosion_level(level: u32) -> Self {
        match modulo(level, 3) {
            0 => Self::Rocky,
            1 => Self::Wet,
            2 => Self::Narrow,
            x => unreachable!("Impossible erosion level: {}", x),
        }
    }

    fn to_char(&self) -> char {
        match self {
            Self::Rocky => '.',
            Self::Wet => '=',
            Self::Narrow => '|',
        }
    }

    fn to_risk_level(&self) -> u32 {
        match self {
            Self::Rocky => 0,
            Self::Wet => 1,
            Self::Narrow => 2,
        }
    }
}

struct Cave {
    depth: u32,
    target: Point,

    // For memoization only
    memo_geologic_index: HashMap<Point, u32>,
    memo_erosion_level: HashMap<Point, u32>,
    memo_region_type: HashMap<Point, RegionType>,
}

impl Cave {
    fn new() -> Self {
        Self {
            depth: 0,
            target: Point::new(),
            memo_geologic_index: HashMap::new(),
            memo_erosion_level: HashMap::new(),
            memo_region_type: HashMap::new(),
        }
    }

    fn from_string(input: &str) -> Self {
        let mut lines = input.lines();
        let depth = lines
            .next()
            .expect("Input terminated early")
            .strip_prefix("depth: ")
            .expect("Depth not found")
            .parse::<u32>()
            .expect("Invalid depth");
        let target = Point::from_string(
            lines
                .next()
                .expect("Input terminated early")
                .strip_prefix("target: ")
                .expect("Target not found"),
        );

        Self {
            depth,
            target,
            memo_geologic_index: HashMap::new(),
            memo_erosion_level: HashMap::new(),
            memo_region_type: HashMap::new(),
        }
    }

    fn geologic_index(&mut self, p: Point) -> u32 {
        if let Some(&gi) = self.memo_geologic_index.get(&p) {
            // If we already know the value, use it
            gi
        } else {
            // Otherwise, calculate it
            let gi = if p == Point::new() {
                0
            } else if p == self.target {
                0
            } else if p.y == 0 {
                p.x as u32 * 16807
            } else if p.x == 0 {
                p.y as u32 * 48271
            } else {
                let e1 = self.erosion_level(Point { x: p.x - 1, y: p.y });
                let e2 = self.erosion_level(Point { x: p.x, y: p.y - 1 });
                e1 * e2
            };
            self.memo_geologic_index.insert(p, gi);
            gi
        }
    }

    fn erosion_level(&mut self, p: Point) -> u32 {
        if let Some(&el) = self.memo_erosion_level.get(&p) {
            // If we already know the value, use it
            el
        } else {
            // Otherwise, calculate it
            let el = modulo(self.geologic_index(p) + self.depth, 20183);
            self.memo_erosion_level.insert(p, el);
            el
        }
    }

    fn region_type(&mut self, p: Point) -> RegionType {
        if let Some(&rt) = self.memo_region_type.get(&p) {
            // If we already know the value, use it
            rt
        } else {
            // Otherwise, calculate it
            let el = self.erosion_level(p);
            let rt = RegionType::from_erosion_level(el);
            self.memo_region_type.insert(p, rt);
            rt
        }
    }

    fn create_map(&mut self) {
        for y in 0..=self.target.y {
            for x in 0..=self.target.x {
                let p = Point { x, y };
                self.region_type(p);
            }
        }
    }

    fn risk_level(&mut self) -> u32 {
        let mut total_risk = 0;
        for y in 0..=self.target.y {
            for x in 0..=self.target.x {
                let p = Point { x, y };
                total_risk += self.region_type(p).to_risk_level();
            }
        }

        total_risk
    }
}

impl fmt::Display for Cave {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for y in 0..=self.target.y {
            for x in 0..=self.target.x {
                let p = Point { x, y };
                if p == Point::new() {
                    write!(f, "M")?;
                } else if p == self.target {
                    write!(f, "T")?;
                } else if let Some(region_type) = self.memo_region_type.get(&p) {
                    write!(f, "{}", region_type.to_char())?;
                } else {
                    write!(f, " ")?;
                }
            }
            writeln!(f)?;
        }

        Ok(())
    }
}

#[aoc(day22, part1)]
pub fn solve(input: &str) -> u32 {
    let mut cave = Cave::from_string(input);
    let risk_level = cave.risk_level();
    println!("Risk level: {}", risk_level);
    assert_eq!(risk_level, 6256);
    risk_level
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_region_type() {
        let mut cave = Cave::new();
        cave.depth = 510;
        cave.target = Point { x: 10, y: 10 };

        let p = Point { x: 0, y: 0 };
        let gi = cave.geologic_index(p);
        let el = cave.erosion_level(p);
        let rt = RegionType::from_erosion_level(el);
        assert_eq!(gi, 0);
        assert_eq!(el, 510);
        assert_eq!(rt, RegionType::Rocky);

        let p = Point { x: 1, y: 0 };
        let gi = cave.geologic_index(p);
        let el = cave.erosion_level(p);
        let rt = RegionType::from_erosion_level(el);
        assert_eq!(gi, 16807);
        assert_eq!(el, 17317);
        assert_eq!(rt, RegionType::Wet);

        let p = Point { x: 0, y: 1 };
        let gi = cave.geologic_index(p);
        let el = cave.erosion_level(p);
        let rt = RegionType::from_erosion_level(el);
        assert_eq!(gi, 48271);
        assert_eq!(el, 8415);
        assert_eq!(rt, RegionType::Rocky);

        let p = Point { x: 1, y: 1 };
        let gi = cave.geologic_index(p);
        let el = cave.erosion_level(p);
        let rt = RegionType::from_erosion_level(el);
        assert_eq!(gi, 145722555);
        assert_eq!(el, 1805);
        assert_eq!(rt, RegionType::Narrow);

        let p = Point { x: 10, y: 10 };
        let gi = cave.geologic_index(p);
        let el = cave.erosion_level(p);
        let rt = RegionType::from_erosion_level(el);
        assert_eq!(gi, 0);
        assert_eq!(el, 510);
        assert_eq!(rt, RegionType::Rocky);
    }

    #[test]
    fn test_risk_level() {
        let mut cave = Cave::new();
        cave.depth = 510;
        cave.target = Point { x: 10, y: 10 };
        assert_eq!(cave.risk_level(), 114);
    }
}
