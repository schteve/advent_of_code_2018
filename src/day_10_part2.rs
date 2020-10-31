/*
    --- Part Two ---
    Good thing you didn't have to wait, because that would have taken a long time - much longer than the 3 seconds in the example above.

    Impressed by your sub-hour communication capabilities, the Elves are curious: exactly how many seconds would they have needed to wait for that message to appear?
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

    fn step_until_minimum_range(&mut self) -> u32 {
        let calc_range_value = |(x, y): ((i32, i32), (i32, i32))| x.1 - x.0 + y.1 - y.0;
        let mut last_range = self.get_range();

        let mut count = 0;
        loop {
            self.step(1);
            count += 1;

            let range = self.get_range();
            let value = calc_range_value(range);
            let last_value = calc_range_value(last_range);
            if value > last_value {
                self.step(-1); // Assume we found the minimum last time; back up a step.
                count -= 1;
                break;
            } else {
                last_range = range;
            }
        }
        count
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

#[aoc(day10, part2)]
pub fn solve(input: &str) -> u32 {
    let mut star_map = StarMap::from_string(input);
    let step_count = star_map.step_until_minimum_range();
    println!("Seconds until message appears: {}", step_count);
    assert_eq!(step_count, 10656);
    10656
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
        let step_count = star_map.step_until_minimum_range();
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
        assert_eq!(step_count, 3);
    }
}
