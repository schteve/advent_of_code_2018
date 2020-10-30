/*
    --- Day 17: Reservoir Research ---
    You arrive in the year 18. If it weren't for the coat you got in 1018, you would be very cold: the North Pole base hasn't even been constructed.

    Rather, it hasn't been constructed yet. The Elves are making a little progress, but there's not a lot of liquid water in this climate, so they're getting very dehydrated. Maybe there's more underground?

    You scan a two-dimensional vertical slice of the ground nearby and discover that it is mostly sand with veins of clay. The scan only provides data with a granularity of square meters, but it should be good enough to determine how much water is trapped there. In the scan, x represents the distance to the right, and y represents the distance down. There is also a spring of water near the surface at x=500, y=0. The scan identifies which square meters are clay (your puzzle input).

    For example, suppose your scan shows the following veins of clay:

    x=495, y=2..7
    y=7, x=495..501
    x=501, y=3..7
    x=498, y=2..4
    x=506, y=1..2
    x=498, y=10..13
    x=504, y=10..13
    y=13, x=498..504
    Rendering clay as #, sand as ., and the water spring as +, and with x increasing to the right and y increasing downward, this becomes:

    44444455555555
    99999900000000
    45678901234567
    0 ......+.......
    1 ............#.
    2 .#..#.......#.
    3 .#..#..#......
    4 .#..#..#......
    5 .#.....#......
    6 .#.....#......
    7 .#######......
    8 ..............
    9 ..............
    10 ....#.....#...
    11 ....#.....#...
    12 ....#.....#...
    13 ....#######...
    The spring of water will produce water forever. Water can move through sand, but is blocked by clay. Water always moves down when possible, and spreads to the left and right otherwise, filling space that has clay on both sides and falling out otherwise.

    For example, if five squares of water are created, they will flow downward until they reach the clay and settle there. Water that has come to rest is shown here as ~, while sand through which water has passed (but which is now dry again) is shown as |:

    ......+.......
    ......|.....#.
    .#..#.|.....#.
    .#..#.|#......
    .#..#.|#......
    .#....|#......
    .#~~~~~#......
    .#######......
    ..............
    ..............
    ....#.....#...
    ....#.....#...
    ....#.....#...
    ....#######...
    Two squares of water can't occupy the same location. If another five squares of water are created, they will settle on the first five, filling the clay reservoir a little more:

    ......+.......
    ......|.....#.
    .#..#.|.....#.
    .#..#.|#......
    .#..#.|#......
    .#~~~~~#......
    .#~~~~~#......
    .#######......
    ..............
    ..............
    ....#.....#...
    ....#.....#...
    ....#.....#...
    ....#######...
    Water pressure does not apply in this scenario. If another four squares of water are created, they will stay on the right side of the barrier, and no water will reach the left side:

    ......+.......
    ......|.....#.
    .#..#.|.....#.
    .#..#~~#......
    .#..#~~#......
    .#~~~~~#......
    .#~~~~~#......
    .#######......
    ..............
    ..............
    ....#.....#...
    ....#.....#...
    ....#.....#...
    ....#######...
    At this point, the top reservoir overflows. While water can reach the tiles above the surface of the water, it cannot settle there, and so the next five squares of water settle like this:

    ......+.......
    ......|.....#.
    .#..#||||...#.
    .#..#~~#|.....
    .#..#~~#|.....
    .#~~~~~#|.....
    .#~~~~~#|.....
    .#######|.....
    ........|.....
    ........|.....
    ....#...|.#...
    ....#...|.#...
    ....#~~~~~#...
    ....#######...
    Note especially the leftmost |: the new squares of water can reach this tile, but cannot stop there. Instead, eventually, they all fall to the right and settle in the reservoir below.

    After 10 more squares of water, the bottom reservoir is also full:

    ......+.......
    ......|.....#.
    .#..#||||...#.
    .#..#~~#|.....
    .#..#~~#|.....
    .#~~~~~#|.....
    .#~~~~~#|.....
    .#######|.....
    ........|.....
    ........|.....
    ....#~~~~~#...
    ....#~~~~~#...
    ....#~~~~~#...
    ....#######...
    Finally, while there is nowhere left for the water to settle, it can reach a few more tiles before overflowing beyond the bottom of the scanned data:

    ......+.......    (line not counted: above minimum y value)
    ......|.....#.
    .#..#||||...#.
    .#..#~~#|.....
    .#..#~~#|.....
    .#~~~~~#|.....
    .#~~~~~#|.....
    .#######|.....
    ........|.....
    ...|||||||||..
    ...|#~~~~~#|..
    ...|#~~~~~#|..
    ...|#~~~~~#|..
    ...|#######|..
    ...|.......|..    (line not counted: below maximum y value)
    ...|.......|..    (line not counted: below maximum y value)
    ...|.......|..    (line not counted: below maximum y value)
    How many tiles can be reached by the water? To prevent counting forever, ignore tiles with a y coordinate smaller than the smallest y coordinate in your scan data or larger than the largest one. Any x coordinate is valid. In this example, the lowest y coordinate given is 1, and the highest is 13, causing the water spring (in row 0) and the water falling off the bottom of the render (in rows 14 through infinity) to be ignored.

    So, in the example above, counting both water at rest (~) and other sand tiles the water can hypothetically reach (|), the total number of tiles the water can reach is 57.

    How many tiles can the water reach within the range of y values in your scan?
*/

use crate::common::Cardinal;
use crate::common::Point;
use regex::Regex;
use std::collections::HashMap;
use std::fmt;

#[derive(Clone, Copy, Eq, PartialEq)]
enum Tile {
    Sand,
    Clay,
    Water,
    DriedSand,
    Spring,
}

impl Tile {
    fn to_char(&self) -> char {
        match *self {
            Self::Sand      => '.',
            Self::Clay      => '#',
            Self::Water     => '~',
            Self::DriedSand => '|',
            Self::Spring    => '+',
        }
    }
}

struct GeologicMap {
    tiles: HashMap<Point, Tile>,
    x_range: (i32, i32),
    y_range: (i32, i32),
    spring: Point,
}

impl GeologicMap {
    fn from_string(input: &str) -> Self {
        let mut tiles = HashMap::new();

        let re_x = Regex::new(r"x=(\d+), y=(\d+)..(\d+)").unwrap();
        let re_y = Regex::new(r"y=(\d+), x=(\d+)..(\d+)").unwrap();

        for cap_x in re_x.captures_iter(input) {
            let x   = cap_x[1].parse::<i32>().unwrap();
            let y_0 = cap_x[2].parse::<i32>().unwrap();
            let y_1 = cap_x[3].parse::<i32>().unwrap();

            for y in y_0 ..= y_1 {
                tiles.insert(Point { x, y }, Tile::Clay);
            }
        }

        for cap_y in re_y.captures_iter(input) {
            let y   = cap_y[1].parse::<i32>().unwrap();
            let x_0 = cap_y[2].parse::<i32>().unwrap();
            let x_1 = cap_y[3].parse::<i32>().unwrap();

            for x in x_0 ..= x_1 {
                tiles.insert(Point { x, y }, Tile::Clay);
            }
        }

        let range = Point::get_range(tiles.keys()).unwrap(); // Must not include the spring so we do this first

        let spring = Point { x: 500, y: 0};
        tiles.insert(spring, Tile::Spring);

        Self {
            tiles,
            x_range: range.0,
            y_range: range.1,
            spring,
        }
    }

    fn fill_path(&mut self, points: &[Point], tile: Tile) {
        let k_v_iter = points.iter()
            .cloned()
            .map(|point| (point, tile));
        self.tiles.extend(k_v_iter);
    }

    fn is_blocked(&self, point: &Point) -> bool {
        match self.tiles.get(point) {
            Some(Tile::Sand) | Some(Tile::DriedSand) | None => false,
            Some(Tile::Clay) | Some(Tile::Water) => true,
            Some(Tile::Spring) => panic!("Tried to flow into a spring"),
        }
    }

    fn ray_cast(&self, start: &Point, direction: Cardinal) -> (bool, Vec<Point>) {
        let mut next = *start;
        let mut path: Vec<Point> = Vec::new();
        loop {
            next = next.step(direction, 1);
            if self.is_blocked(&next) == true {
                // Blocked, end here
                return (true, path);
            } else {
                // Not blocked
                path.push(next);

                let next_below = next.step(Cardinal::South, 1);
                if self.is_blocked(&next_below) == false {
                    // Not blocked, and there's room to flow down
                    return (false, path);
                }
            }
        }
    }

    fn source_flow(&mut self, source: &Point) -> Vec<Point> {
        let mut path: Vec<Point> = vec![source.step(Cardinal::South, 1)]; // Water starts flowing one tile below the source
        let mut new_sources: Vec<Point> = Vec::new();

        // If we can move down, do that.
        // If we can't move down, move sideways in both directions.
        // If it's blocked on both sides then the water is contained and we end.
        // If it's not blocked on at least one side, the water runs off the side(s) and the sand becomes dried.
        while path.is_empty() == false {
            let head = path[path.len() - 1];
            let point_below = head.step(Cardinal::South, 1);
            if point_below.y > self.y_range.1 {
                // Ran off the bottom of the map. Commit this path - turn it to DriedSand.
                self.fill_path(&path, Tile::DriedSand);
                break; // Done processing this path
            }

            // Next point is in bounds
            if self.is_blocked(&point_below) == false {
                // The way is open
                path.push(point_below);
            } else {
                // The way is blocked, determine if we can flow left, right, both, or neither
                let (left_blocked, left_path) = self.ray_cast(&head, Cardinal::West);
                let (right_blocked, right_path) = self.ray_cast(&head, Cardinal::East);
                if left_blocked == true && right_blocked == true {
                    // Can't flow in either direction. Fill with water.
                    self.tiles.insert(head, Tile::Water);
                    self.fill_path(&left_path, Tile::Water);
                    self.fill_path(&right_path, Tile::Water);

                    // Back up one step and continue flowing. This ensures that bowls get filled up.
                    path.pop();
                } else {
                    // Spawn new path if left side wasn't blocked
                    if left_blocked == false {
                        new_sources.push(left_path[left_path.len() - 1]); // Path is guaranteed to be at least 1 length since it takes one step over the edge before getting here
                    }

                    // Spawn new path if right side wasn't blocked
                    if right_blocked == false {
                        new_sources.push(right_path[right_path.len() - 1]); // Path is guaranteed to be at least 1 length since it takes one step over the edge before getting here
                    }

                    // Flow is not blocked. Add the left and right sides to the path.
                    path.extend(left_path);
                    path.extend(right_path);

                    // Commit this path - turn it to DriedSand
                    self.fill_path(&path, Tile::DriedSand);
                    break; // Done processing this path
                }
            }
        }

        new_sources
    }

    fn water_flow(&mut self) {
        let mut sources: Vec<Point> = vec![self.spring];
        let mut snapshot = self.to_string();
        loop {
            for source in sources.drain(..).collect::<Vec<Point>>() {
                sources.extend(self.source_flow(&source));
            }
            sources.sort(); // This is on balance faster than using a HashSet or BTreeSet to perform deduplication
            sources.dedup();

            if sources.is_empty() == true {
                if snapshot == self.to_string() {
                    // No change during this iteration, we're done
                    break;
                } else {
                    // Something changed, so continue simulating
                    snapshot = self.to_string();
                    sources.push(self.spring);
                }
            }
        }
    }

    fn count_water_can_touch(&self) -> u32 {
        self.tiles.iter()
            .filter(|&(point, _tile)| point.y >= self.y_range.0 && point.y <= self.y_range.1)
            .filter(|&(_point, tile)| tile == &Tile::Water || tile == &Tile::DriedSand)
            .count() as u32
    }
}

impl fmt::Display for GeologicMap {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let (x_range, y_range) = Point::get_range(self.tiles.keys()).unwrap(); // Don't use self range because it won't include any water that flowed out of bounds left or right
        for y in y_range.0 ..= y_range.1 {
            //write!(f, "{:4} ", y)?;
            for x in x_range.0 ..= x_range.1 {
                if let Some(tile) = self.tiles.get(&Point { x, y }) {
                    write!(f, "{}", tile.to_char())?;
                } else {
                    write!(f, ".")?;
                }
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

#[aoc(day17, part1)]
pub fn solve(input: &str) -> u32 {
    let mut geo_map = GeologicMap::from_string(input);
    geo_map.water_flow();
    //println!("{}", geo_map);
    let water_can_touch = geo_map.count_water_can_touch();
    println!("Water can touch: {}", water_can_touch);
    assert_eq!(water_can_touch, 39649);
    water_can_touch
}


#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_water_flow() {
        let input = "
x=495, y=2..7
y=7, x=495..501
x=501, y=3..7
x=498, y=2..4
x=506, y=1..2
x=498, y=10..13
x=504, y=10..13
y=13, x=498..504";
        let result = "
.....+......
.....|.....#
#..#||||...#
#..#~~#|....
#..#~~#|....
#~~~~~#|....
#~~~~~#|....
#######|....
.......|....
..|||||||||.
..|#~~~~~#|.
..|#~~~~~#|.
..|#~~~~~#|.
..|#######|.";
        let mut geo_map = GeologicMap::from_string(input);
        geo_map.water_flow();
        assert_eq!(geo_map.to_string().trim(), result.trim());
        assert_eq!(geo_map.count_water_can_touch(), 57);
    }
}
