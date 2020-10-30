/*
    --- Part Two ---
    You discover a dial on the side of the device; it seems to let you select a square of any size, not just 3x3. Sizes from 1x1 to 300x300 are supported.

    Realizing this, you now must find the square of any size with the largest total power. Identify this square by including its size as a third parameter after the top-left coordinate: a 9x9 square with a top-left corner of 3,5 is identified as 3,5,9.

    For example:

    For grid serial number 18, the largest total square (with a total power of 113) is 16x16 and has a top-left corner of 90,269, so its identifier is 90,269,16.
    For grid serial number 42, the largest total square (with a total power of 119) is 12x12 and has a top-left corner of 232,251, so its identifier is 232,251,12.
    What is the X,Y,size identifier of the square with the largest total power?
*/

use crate::common::Point;
use std::collections::HashMap;

struct Grid {
    power: HashMap<(Point, u32), i32>, // Power level for each Point / size combination
    size: u32,
}

impl Grid {
    fn new(size: u32) -> Self {
        Self {
            power: HashMap::new(),
            size,
        }
    }

    fn power_level(p: Point, serial: i32) -> i32 {
        let rack_id = p.x + 10;
        let mut power = rack_id * p.y;
        power += serial;
        power *= rack_id;
        power = (power % 1000) / 100;
        power -= 5;
        power
    }

    fn fuel_grid(&mut self, serial: i32) {
        for x in 0..self.size {
            for y in 0..self.size {
                let p = Point { x: x as i32, y: y as i32 };
                let power = Grid::power_level(p, serial);
                self.power.insert((p, 1), power);
            }
        }
    }

    fn total_power(&mut self, point: Point, size: u32) -> i32 {
        // Memoize
        if let Some(&power) = self.power.get(&(point, size)) {
            return power;
        }

        assert!(size != 0);

        let mut total_power = 0;

        if size % 2 == 0 {
            // Square can be evenly divided into 4 sub-squares
            let subsquare = size / 2;
            for x in 0..2 {
                for y in 0..2 {
                    let p = Point {
                        x: point.x + x * subsquare as i32,
                        y: point.y + y * subsquare as i32,
                    };
                    total_power += self.total_power(p, subsquare); // self.square_power[&(p, subsquare)];
                }
            }
        } else {
            // Square cannot be evenly divided. Split it into one large (size - 1) and many small (1).

            // The large square
            total_power += self.power[&(point, size - 1)];

            // Many small squares. Bottom row, then right row, being careful of the point that is in both.
            for x in 0..size {
                let p = Point {
                    x: point.x + x as i32,
                    y: point.y + size as i32 - 1,
                };
                total_power += self.total_power(p, 1);
            }
            for y in 0 .. (size - 1) {
                let p = Point {
                    x: point.x + size as i32 - 1,
                    y: point.y + y as i32,
                };
                total_power += self.total_power(p, 1);
            }
        }

        self.power.insert((point, size), total_power);
        total_power
    }

    fn max_power_point(&mut self) -> (Point, u32) {
        let mut max_power = None;
        let mut max_point = None;
        let mut max_size = None;
        for size in 2 ..= self.size {
            //println!("Size: {}", size);
            for x in 0 .. (self.size - size) {
                for y in 0 .. (self.size - size) {
                    let p = Point {
                        x: x as i32,
                        y: y as i32,
                    };
                    let power = self.total_power(p, size);
                    if max_power.is_none() || power > max_power.unwrap() {
                        max_power = Some(power);
                        max_point = Some(p);
                        max_size = Some(size);
                    }
                }
            }
        }
        (max_point.unwrap(), max_size.unwrap())
    }
}

#[aoc(day11, part2)]
pub fn solve(input: &str) -> String {
    let serial = input.trim().parse::<i32>().unwrap();

    let mut grid = Grid::new(300);
    grid.fuel_grid(serial);

    let (max_power_point, max_power_size) = grid.max_power_point();
    println!("Largest total power: {}, {}", max_power_point, max_power_size);
    assert_eq!(max_power_point, Point { x: 236, y: 252 });
    assert_eq!(max_power_size, 12);

    use std::fmt::Write;
    let mut result = String::new();
    write!(&mut result, "{}, {}", max_power_point, max_power_size).expect("String write fail");
    result
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_power_level() {
        let point = Point { x: 3, y: 5 };
        let serial = 8;
        let power = Grid::power_level(point, serial);
        assert_eq!(power, 4);

        let point = Point { x: 122, y: 79 };
        let serial = 57;
        let power = Grid::power_level(point, serial);
        assert_eq!(power, -5);

        let point = Point { x: 217, y: 196 };
        let serial = 39;
        let power = Grid::power_level(point, serial);
        assert_eq!(power, 0);

        let point = Point { x: 101, y: 153 };
        let serial = 71;
        let power = Grid::power_level(point, serial);
        assert_eq!(power, 4);
    }
}
