/*
    --- Day 10: The Stars Align ---
    It's no use; your navigation system simply isn't capable of providing walking directions in the arctic circle, and certainly not in 1018.

    The Elves suggest an alternative. In times like these, North Pole rescue operations will arrange points of light in the sky to guide missing Elves back to base. Unfortunately, the message is easy to miss: the points move slowly enough that it takes hours to align them, but have so much momentum that they only stay aligned for a second. If you blink at the wrong time, it might be hours before another message appears.

    You can see these points of light floating in the distance, and record their position in the sky and their velocity, the relative change in position per second (your puzzle input). The coordinates are all given from your perspective; given enough time, those positions and velocities will move the points into a cohesive message!

    Rather than wait, you decide to fast-forward the process and calculate what the points will eventually spell.

    For example, suppose you note the following points:

    position=< 9,  1> velocity=< 0,  2>
    position=< 7,  0> velocity=<-1,  0>
    position=< 3, -2> velocity=<-1,  1>
    position=< 6, 10> velocity=<-2, -1>
    position=< 2, -4> velocity=< 2,  2>
    position=<-6, 10> velocity=< 2, -2>
    position=< 1,  8> velocity=< 1, -1>
    position=< 1,  7> velocity=< 1,  0>
    position=<-3, 11> velocity=< 1, -2>
    position=< 7,  6> velocity=<-1, -1>
    position=<-2,  3> velocity=< 1,  0>
    position=<-4,  3> velocity=< 2,  0>
    position=<10, -3> velocity=<-1,  1>
    position=< 5, 11> velocity=< 1, -2>
    position=< 4,  7> velocity=< 0, -1>
    position=< 8, -2> velocity=< 0,  1>
    position=<15,  0> velocity=<-2,  0>
    position=< 1,  6> velocity=< 1,  0>
    position=< 8,  9> velocity=< 0, -1>
    position=< 3,  3> velocity=<-1,  1>
    position=< 0,  5> velocity=< 0, -1>
    position=<-2,  2> velocity=< 2,  0>
    position=< 5, -2> velocity=< 1,  2>
    position=< 1,  4> velocity=< 2,  1>
    position=<-2,  7> velocity=< 2, -2>
    position=< 3,  6> velocity=<-1, -1>
    position=< 5,  0> velocity=< 1,  0>
    position=<-6,  0> velocity=< 2,  0>
    position=< 5,  9> velocity=< 1, -2>
    position=<14,  7> velocity=<-2,  0>
    position=<-3,  6> velocity=< 2, -1>
    Each line represents one point. Positions are given as <X, Y> pairs: X represents how far left (negative) or right (positive) the point appears, while Y represents how far up (negative) or down (positive) the point appears.

    At 0 seconds, each point has the position given. Each second, each point's velocity is added to its position. So, a point with velocity <1, -2> is moving to the right, but is moving upward twice as quickly. If this point's initial position were <3, 9>, after 3 seconds, its position would become <6, 3>.

    Over time, the points listed above would move like this:

    Initially:
    ........#.............
    ................#.....
    .........#.#..#.......
    ......................
    #..........#.#.......#
    ...............#......
    ....#.................
    ..#.#....#............
    .......#..............
    ......#...............
    ...#...#.#...#........
    ....#..#..#.........#.
    .......#..............
    ...........#..#.......
    #...........#.........
    ...#.......#..........

    After 1 second:
    ......................
    ......................
    ..........#....#......
    ........#.....#.......
    ..#.........#......#..
    ......................
    ......#...............
    ....##.........#......
    ......#.#.............
    .....##.##..#.........
    ........#.#...........
    ........#...#.....#...
    ..#...........#.......
    ....#.....#.#.........
    ......................
    ......................

    After 2 seconds:
    ......................
    ......................
    ......................
    ..............#.......
    ....#..#...####..#....
    ......................
    ........#....#........
    ......#.#.............
    .......#...#..........
    .......#..#..#.#......
    ....#....#.#..........
    .....#...#...##.#.....
    ........#.............
    ......................
    ......................
    ......................

    After 3 seconds:
    ......................
    ......................
    ......................
    ......................
    ......#...#..###......
    ......#...#...#.......
    ......#...#...#.......
    ......#####...#.......
    ......#...#...#.......
    ......#...#...#.......
    ......#...#...#.......
    ......#...#..###......
    ......................
    ......................
    ......................
    ......................

    After 4 seconds:
    ......................
    ......................
    ......................
    ............#.........
    ........##...#.#......
    ......#.....#..#......
    .....#..##.##.#.......
    .......##.#....#......
    ...........#....#.....
    ..............#.......
    ....#......#...#......
    .....#.....##.........
    ...............#......
    ...............#......
    ......................
    ......................
    After 3 seconds, the message appeared briefly: HI. Of course, your message will be much longer and will take many more seconds to appear.

    What message will eventually appear in the sky?
*/

use super::common::Point;
use regex::Regex;
use std::fmt;

struct StarMap {
    stars: Vec<Point>,
    velocity: Vec<Point>,
}

impl StarMap {
    fn from_string(input: &str) -> Self {
        let mut stars: Vec<Point> = Vec::new();
        let mut velocity: Vec<Point> = Vec::new();

        let re = Regex::new(r"position=<\s*(-?\d+),\s+(-?\d+)> velocity=<\s*(-?\d+),\s+(-?\d+)>")
            .unwrap();
        for cap in re.captures_iter(input) {
            let p = Point {
                x: cap[1].parse::<i32>().unwrap(),
                y: cap[2].parse::<i32>().unwrap(),
            };
            stars.push(p);

            let p = Point {
                x: cap[3].parse::<i32>().unwrap(),
                y: cap[4].parse::<i32>().unwrap(),
            };
            velocity.push(p);
        }

        Self { stars, velocity }
    }

    fn get_range(&self) -> ((i32, i32), (i32, i32)) {
        if self.stars.is_empty() == true {
            return ((0, 0), (0, 0));
        }

        let mut x_range = (self.stars[0].x, self.stars[0].x);
        let mut y_range = (self.stars[0].y, self.stars[0].y);

        for p in &self.stars {
            if p.x < x_range.0 {
                x_range = (p.x, x_range.1);
            } else if p.x > x_range.1 {
                x_range = (x_range.0, p.x);
            }

            if p.y < y_range.0 {
                y_range = (p.y, y_range.1);
            } else if p.y > y_range.1 {
                y_range = (y_range.0, p.y);
            }
        }

        (x_range, y_range)
    }

    fn step(&mut self, count: i32) {
        if count < 0 {
            for _ in count..0 {
                for (star, vel) in self.stars.iter_mut().zip(self.velocity.iter()) {
                    *star -= *vel;
                }
            }
        } else {
            for _ in 0..count {
                for (star, vel) in self.stars.iter_mut().zip(self.velocity.iter()) {
                    *star += *vel;
                }
            }
        }
    }

    fn step_until_minimum_range(&mut self) {
        let calc_range_value = |(x, y): ((i32, i32), (i32, i32))| x.1 - x.0 + y.1 - y.0;
        let mut last_range = self.get_range();

        loop {
            self.step(1);

            let range = self.get_range();
            let value = calc_range_value(range);
            let last_value = calc_range_value(last_range);
            if value > last_value {
                self.step(-1); // Assume we found the minimum last time; back up a step.
                break;
            } else {
                last_range = range;
            }
        }
    }
}

impl fmt::Display for StarMap {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f)?;
        let (x_range, y_range) = self.get_range();
        for y in y_range.0..=y_range.1 {
            for x in x_range.0..=x_range.1 {
                let p = Point { x, y };
                if self.stars.contains(&p) == true {
                    write!(f, "#")?;
                } else {
                    write!(f, ".")?;
                }
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

#[aoc(day10, part1)]
pub fn solve(input: &str) -> String {
    let mut star_map = StarMap::from_string(input);
    star_map.step_until_minimum_range();
    let message = star_map.to_string();
    assert_eq!(
        message.trim(),
        "
#....#..#.......######....##....#....#..#####....####...######
#....#..#............#...#..#...#...#...#....#..#....#.......#
.#..#...#............#..#....#..#..#....#....#..#............#
.#..#...#...........#...#....#..#.#.....#....#..#...........#.
..##....#..........#....#....#..##......#####...#..........#..
..##....#.........#.....######..##......#....#..#..###....#...
.#..#...#........#......#....#..#.#.....#....#..#....#...#....
.#..#...#.......#.......#....#..#..#....#....#..#....#..#.....
#....#..#.......#.......#....#..#...#...#....#..#...##..#.....
#....#..######..######..#....#..#....#..#####....###.#..######"
            .trim()
    );
    message
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_step() {
        let input = "
position=< 9,  1> velocity=< 0,  2>
position=< 7,  0> velocity=<-1,  0>
position=< 3, -2> velocity=<-1,  1>
position=< 6, 10> velocity=<-2, -1>
position=< 2, -4> velocity=< 2,  2>
position=<-6, 10> velocity=< 2, -2>
position=< 1,  8> velocity=< 1, -1>
position=< 1,  7> velocity=< 1,  0>
position=<-3, 11> velocity=< 1, -2>
position=< 7,  6> velocity=<-1, -1>
position=<-2,  3> velocity=< 1,  0>
position=<-4,  3> velocity=< 2,  0>
position=<10, -3> velocity=<-1,  1>
position=< 5, 11> velocity=< 1, -2>
position=< 4,  7> velocity=< 0, -1>
position=< 8, -2> velocity=< 0,  1>
position=<15,  0> velocity=<-2,  0>
position=< 1,  6> velocity=< 1,  0>
position=< 8,  9> velocity=< 0, -1>
position=< 3,  3> velocity=<-1,  1>
position=< 0,  5> velocity=< 0, -1>
position=<-2,  2> velocity=< 2,  0>
position=< 5, -2> velocity=< 1,  2>
position=< 1,  4> velocity=< 2,  1>
position=<-2,  7> velocity=< 2, -2>
position=< 3,  6> velocity=<-1, -1>
position=< 5,  0> velocity=< 1,  0>
position=<-6,  0> velocity=< 2,  0>
position=< 5,  9> velocity=< 1, -2>
position=<14,  7> velocity=<-2,  0>
position=<-3,  6> velocity=< 2, -1>";
        let mut star_map = StarMap::from_string(input);

        let message = star_map.to_string();
        assert_eq!(
            message.trim(),
            "
........#.............
................#.....
.........#.#..#.......
......................
#..........#.#.......#
...............#......
....#.................
..#.#....#............
.......#..............
......#...............
...#...#.#...#........
....#..#..#.........#.
.......#..............
...........#..#.......
#...........#.........
...#.......#.........."
                .trim()
        );

        star_map.step(1);
        let message = star_map.to_string();
        assert_eq!(
            message.trim(),
            "
........#....#....
......#.....#.....
#.........#......#
..................
....#.............
..##.........#....
....#.#...........
...##.##..#.......
......#.#.........
......#...#.....#.
#...........#.....
..#.....#.#......."
                .trim()
        );

        star_map.step(1);
        let message = star_map.to_string();
        assert_eq!(
            message.trim(),
            "
..........#...
#..#...####..#
..............
....#....#....
..#.#.........
...#...#......
...#..#..#.#..
#....#.#......
.#...#...##.#.
....#........."
                .trim()
        );

        star_map.step(1);
        let message = star_map.to_string();
        assert_eq!(
            message.trim(),
            "
#...#..###
#...#...#.
#...#...#.
#####...#.
#...#...#.
#...#...#.
#...#...#.
#...#..###"
                .trim()
        );

        star_map.step(1);
        let message = star_map.to_string();
        assert_eq!(
            message.trim(),
            "
........#....
....##...#.#.
..#.....#..#.
.#..##.##.#..
...##.#....#.
.......#....#
..........#..
#......#...#.
.#.....##....
...........#.
...........#."
                .trim()
        );
    }

    #[test]
    fn test_step_until_minimum_range() {
        let input = "
position=< 9,  1> velocity=< 0,  2>
position=< 7,  0> velocity=<-1,  0>
position=< 3, -2> velocity=<-1,  1>
position=< 6, 10> velocity=<-2, -1>
position=< 2, -4> velocity=< 2,  2>
position=<-6, 10> velocity=< 2, -2>
position=< 1,  8> velocity=< 1, -1>
position=< 1,  7> velocity=< 1,  0>
position=<-3, 11> velocity=< 1, -2>
position=< 7,  6> velocity=<-1, -1>
position=<-2,  3> velocity=< 1,  0>
position=<-4,  3> velocity=< 2,  0>
position=<10, -3> velocity=<-1,  1>
position=< 5, 11> velocity=< 1, -2>
position=< 4,  7> velocity=< 0, -1>
position=< 8, -2> velocity=< 0,  1>
position=<15,  0> velocity=<-2,  0>
position=< 1,  6> velocity=< 1,  0>
position=< 8,  9> velocity=< 0, -1>
position=< 3,  3> velocity=<-1,  1>
position=< 0,  5> velocity=< 0, -1>
position=<-2,  2> velocity=< 2,  0>
position=< 5, -2> velocity=< 1,  2>
position=< 1,  4> velocity=< 2,  1>
position=<-2,  7> velocity=< 2, -2>
position=< 3,  6> velocity=<-1, -1>
position=< 5,  0> velocity=< 1,  0>
position=<-6,  0> velocity=< 2,  0>
position=< 5,  9> velocity=< 1, -2>
position=<14,  7> velocity=<-2,  0>
position=<-3,  6> velocity=< 2, -1>";
        let mut star_map = StarMap::from_string(input);
        star_map.step_until_minimum_range();
        let message = star_map.to_string();
        assert_eq!(
            message.trim(),
            "
#...#..###
#...#...#.
#...#...#.
#####...#.
#...#...#.
#...#...#.
#...#...#.
#...#..###"
                .trim()
        );
    }
}
