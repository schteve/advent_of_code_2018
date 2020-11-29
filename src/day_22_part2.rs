/*
    --- Part Two ---
    Okay, it's time to go rescue the man's friend.

    As you leave, he hands you some tools: a torch and some climbing gear. You can't equip both tools at once, but you can choose to use neither.

    Tools can only be used in certain regions:

    In rocky regions, you can use the climbing gear or the torch. You cannot use neither (you'll likely slip and fall).
    In wet regions, you can use the climbing gear or neither tool. You cannot use the torch (if it gets wet, you won't have a light source).
    In narrow regions, you can use the torch or neither tool. You cannot use the climbing gear (it's too bulky to fit).
    You start at 0,0 (the mouth of the cave) with the torch equipped and must reach the target coordinates as quickly as possible. The regions with negative X or Y are solid rock and cannot be traversed. The fastest route might involve entering regions beyond the X or Y coordinate of the target.

    You can move to an adjacent region (up, down, left, or right; never diagonally) if your currently equipped tool allows you to enter that region. Moving to an adjacent region takes one minute. (For example, if you have the torch equipped, you can move between rocky and narrow regions, but cannot enter wet regions.)

    You can change your currently equipped tool or put both away if your new equipment would be valid for your current region. Switching to using the climbing gear, torch, or neither always takes seven minutes, regardless of which tools you start with. (For example, if you are in a rocky region, you can switch from the torch to the climbing gear, but you cannot switch to neither.)

    Finally, once you reach the target, you need the torch equipped before you can find him in the dark. The target is always in a rocky region, so if you arrive there with climbing gear equipped, you will need to spend seven minutes switching to your torch.

    For example, using the same cave system as above, starting in the top left corner (0,0) and moving to the bottom right corner (the target, 10,10) as quickly as possible, one possible route is as follows, with your current position marked X:

    Initially:
    X=.|=.|.|=.|=|=.
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

    Down:
    M=.|=.|.|=.|=|=.
    X|=|=|||..|.=...
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

    Right:
    M=.|=.|.|=.|=|=.
    .X=|=|||..|.=...
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

    Switch from using the torch to neither tool:
    M=.|=.|.|=.|=|=.
    .X=|=|||..|.=...
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

    Right 3:
    M=.|=.|.|=.|=|=.
    .|=|X|||..|.=...
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

    Switch from using neither tool to the climbing gear:
    M=.|=.|.|=.|=|=.
    .|=|X|||..|.=...
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

    Down 7:
    M=.|=.|.|=.|=|=.
    .|=|=|||..|.=...
    .==|....||=..|==
    =.|....|.==.|==.
    =|..==...=.|==..
    =||.=.=||=|=..|=
    |.=.===|||..=..|
    |..==||=.|==|===
    .=..X==..=|.|||.
    .======|||=|=.|=
    .===|=|===T===||
    =|||...|==..|=.|
    =.=|=.=..=.||==|
    ||=|=...|==.=|==
    |=.=||===.|||===
    ||.|==.|.|.||=||

    Right:
    M=.|=.|.|=.|=|=.
    .|=|=|||..|.=...
    .==|....||=..|==
    =.|....|.==.|==.
    =|..==...=.|==..
    =||.=.=||=|=..|=
    |.=.===|||..=..|
    |..==||=.|==|===
    .=..=X=..=|.|||.
    .======|||=|=.|=
    .===|=|===T===||
    =|||...|==..|=.|
    =.=|=.=..=.||==|
    ||=|=...|==.=|==
    |=.=||===.|||===
    ||.|==.|.|.||=||

    Down 3:
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
    =|||.X.|==..|=.|
    =.=|=.=..=.||==|
    ||=|=...|==.=|==
    |=.=||===.|||===
    ||.|==.|.|.||=||

    Right:
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
    =|||..X|==..|=.|
    =.=|=.=..=.||==|
    ||=|=...|==.=|==
    |=.=||===.|||===
    ||.|==.|.|.||=||

    Down:
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
    =.=|=.X..=.||==|
    ||=|=...|==.=|==
    |=.=||===.|||===
    ||.|==.|.|.||=||

    Right 4:
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
    =.=|=.=..=X||==|
    ||=|=...|==.=|==
    |=.=||===.|||===
    ||.|==.|.|.||=||

    Up 2:
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
    .===|=|===X===||
    =|||...|==..|=.|
    =.=|=.=..=.||==|
    ||=|=...|==.=|==
    |=.=||===.|||===
    ||.|==.|.|.||=||

    Switch from using the climbing gear to the torch:
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
    .===|=|===X===||
    =|||...|==..|=.|
    =.=|=.=..=.||==|
    ||=|=...|==.=|==
    |=.=||===.|||===
    ||.|==.|.|.||=||
    This is tied with other routes as the fastest way to reach the target: 45 minutes. In it, 21 minutes are spent switching tools (three times, seven minutes each) and the remaining 24 minutes are spent moving.

    What is the fewest number of minutes you can take to reach the target?
*/

use crate::common::modulo;
use crate::common::Point;
use std::cmp::Ordering;
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
}

#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
enum Tool {
    Neither,
    Torch,
    ClimbingGear,
}

impl Tool {
    fn is_compatible(&self, rt: RegionType) -> bool {
        match rt {
            RegionType::Rocky => *self != Self::Neither,
            RegionType::Wet => *self != Self::Torch,
            RegionType::Narrow => *self != Self::ClimbingGear,
        }
    }

    fn switch(&self, rt: RegionType) -> Self {
        match rt {
            RegionType::Rocky => match self {
                Self::Neither => panic!("Invalid tool for region type"),
                Self::Torch => Self::ClimbingGear,
                Self::ClimbingGear => Self::Torch,
            },
            RegionType::Wet => match self {
                Self::Neither => Self::ClimbingGear,
                Self::Torch => panic!("Invalid tool for region type"),
                Self::ClimbingGear => Self::Neither,
            },
            RegionType::Narrow => match self {
                Self::Neither => Self::Torch,
                Self::Torch => Self::Neither,
                Self::ClimbingGear => panic!("Invalid tool for region type"),
            },
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
struct Node {
    location: Point,
    tool: Tool,
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

    fn find_fastest_time_to_target(&mut self) -> u32 {
        let mut minutes: HashMap<Node, u32> = HashMap::new();

        let start = Node {
            location: Point::new(),
            tool: Tool::Torch,
        };
        let mut frontier: Vec<(Node, u32)> = vec![(start, 0)];

        while frontier.is_empty() == false {
            let (curr_node, curr_time) = frontier.pop().unwrap();

            // Check if the target was reached
            let target = Node {
                location: self.target,
                tool: Tool::Torch,
            };
            if curr_node == target {
                // Target reached, return the time it took to reach
                return curr_time;
            }

            // First add movements in each direction
            for next in curr_node.location.orthogonals() {
                if next.x < 0 || next.y < 0 {
                    // Out of bounds
                    continue;
                }

                if curr_node.tool.is_compatible(self.region_type(next)) == false {
                    // Incompatible with the current tool
                    continue;
                }

                let next_node = Node {
                    location: next,
                    tool: curr_node.tool,
                };
                let next_time = curr_time + 1;

                // Record (or overwrite) the best time for the next node if it is better than what has been seen before
                if let Some(best_time) = minutes.get_mut(&next_node) {
                    if next_time < *best_time {
                        // Node has already been seen and we have a better time
                        *best_time = next_time;
                    } else {
                        // Already reached this node from a quicker path, ignore it
                        continue;
                    }
                } else {
                    minutes.insert(next_node, next_time);
                }

                // Add to the frontier
                frontier.push((next_node, next_time));
            }

            // Then add tool change
            let next_tool = curr_node.tool.switch(self.region_type(curr_node.location));
            let next_node = Node {
                location: curr_node.location,
                tool: next_tool,
            };
            let next_time = curr_time + 7;

            // Record (or overwrite) the best time for the next node if it is better than what has been seen before
            if let Some(best_time) = minutes.get_mut(&next_node) {
                if next_time < *best_time {
                    // Node has already been seen and we have a better time - shouldn't happen in practice
                    *best_time = next_time;
                    frontier.push((next_node, next_time));
                } else {
                    // Already reached this node from a quicker path, ignore it
                }
            } else {
                minutes.insert(next_node, next_time);
                frontier.push((next_node, next_time));
            }

            // Sort the frontier so that the next node checked is also the shortest path to that node. Remove any duplicates.
            frontier.sort_unstable_by(|a, b| match b.1.cmp(&a.1) {
                Ordering::Equal => b.0.cmp(&a.0),
                ord => ord,
            });
            frontier.dedup();
        }

        panic!("Failed to find target!");
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

#[aoc(day22, part2)]
pub fn solve(input: &str) -> u32 {
    let mut cave = Cave::from_string(input);
    let fastest_time = cave.find_fastest_time_to_target();
    println!("Fastest time: {}", fastest_time);
    assert_eq!(fastest_time, 973);
    fastest_time
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
    fn test_find_fastest_time_to_target() {
        let mut cave = Cave::new();
        cave.depth = 510;
        cave.target = Point { x: 10, y: 10 };
        let fastest_time = cave.find_fastest_time_to_target();
        assert_eq!(fastest_time, 45);
    }
}
