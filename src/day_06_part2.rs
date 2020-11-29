/*
    --- Day 6: Chronal Coordinates ---
    The device on your wrist beeps several times, and once again you feel like you're falling.

    "Situation critical," the device announces. "Destination indeterminate. Chronal interference detected. Please specify new target coordinates."

    The device then produces a list of coordinates (your puzzle input). Are they places it thinks are safe or dangerous? It recommends you check manual page 729. The Elves did not give you a manual.

    If they're dangerous, maybe you can minimize the danger by finding the coordinate that gives the largest distance from the other points.

    Using only the Manhattan distance, determine the area around each coordinate by counting the number of integer X,Y locations that are closest to that coordinate (and aren't tied in distance to any other coordinate).

    Your goal is to find the size of the largest area that isn't infinite. For example, consider the following list of coordinates:

    1, 1
    1, 6
    8, 3
    3, 4
    5, 5
    8, 9
    If we name these coordinates A through F, we can draw them on a grid, putting 0,0 at the top left:

    ..........
    .A........
    ..........
    ........C.
    ...D......
    .....E....
    .B........
    ..........
    ..........
    ........F.
    This view is partial - the actual grid extends infinitely in all directions. Using the Manhattan distance, each location's closest coordinate can be determined, shown here in lowercase:

    aaaaa.cccc
    aAaaa.cccc
    aaaddecccc
    aadddeccCc
    ..dDdeeccc
    bb.deEeecc
    bBb.eeee..
    bbb.eeefff
    bbb.eeffff
    bbb.ffffFf
    Locations shown as . are equally far from two or more coordinates, and so they don't count as being closest to any.

    In this example, the areas of coordinates A, B, C, and F are infinite - while not shown here, their areas extend forever outside the visible grid. However, the areas of coordinates D and E are finite: D is closest to 9 locations, and E is closest to 17 (both including the coordinate's location itself). Therefore, in this example, the size of the largest area is 17.

    What is the size of the largest area that isn't infinite?
*/

use super::common::Point;
use std::collections::HashMap;

struct LandingZone {
    coordinates: Vec<Point>,
    area: HashMap<Point, u32>,
}

impl LandingZone {
    fn from_string(input: &str) -> Self {
        let coordinates: Vec<Point> = input.lines().map(Point::from_string).collect();
        Self {
            coordinates,
            area: HashMap::new(),
        }
    }

    fn total_distance(&self, point: Point) -> u32 {
        self.coordinates
            .iter()
            .map(|&coord| Point::manhattan(point, coord))
            .sum()
    }

    fn get_range(&self) -> ((i32, i32), (i32, i32)) {
        let mut x_range = (0, 0);
        let mut y_range = (0, 0);

        for p in &self.coordinates {
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

    fn scan(&mut self) {
        let (x_range, y_range) = self.get_range();
        for y in y_range.0..=y_range.1 {
            for x in x_range.0..=x_range.1 {
                let p = Point { x, y };
                let total_distance = self.total_distance(p);
                self.area.insert(p, total_distance);
            }
        }
    }

    fn count_points_under_size(&self, size: u32) -> usize {
        self.area.values().filter(|&&dist| dist < size).count()
    }
}

#[aoc(day6, part2)]
pub fn solve(input: &str) -> usize {
    let mut landing_zone = LandingZone::from_string(input);
    landing_zone.scan();
    let max_size = 10000;
    let region_size = landing_zone.count_points_under_size(max_size);
    println!("Region under {}: {}", max_size, region_size);
    assert_eq!(region_size, 43852);
    region_size
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_count_points_under_size() {
        let input = "\
1, 1
1, 6
8, 3
3, 4
5, 5
8, 9";
        let mut landing_zone = LandingZone::from_string(input);
        landing_zone.scan();
        let region_size = landing_zone.count_points_under_size(32);
        assert_eq!(region_size, 16);
    }
}
