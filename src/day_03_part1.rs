/*
    --- Day 3: No Matter How You Slice It ---
    The Elves managed to locate the chimney-squeeze prototype fabric for Santa's suit (thanks to someone who helpfully wrote its box IDs on the wall of the warehouse in the middle of the night). Unfortunately, anomalies are still affecting them - nobody can even agree on how to cut the fabric.

    The whole piece of fabric they're working on is a very large square - at least 1000 inches on each side.

    Each Elf has made a claim about which area of fabric would be ideal for Santa's suit. All claims have an ID and consist of a single rectangle with edges parallel to the edges of the fabric. Each claim's rectangle is defined as follows:

    The number of inches between the left edge of the fabric and the left edge of the rectangle.
    The number of inches between the top edge of the fabric and the top edge of the rectangle.
    The width of the rectangle in inches.
    The height of the rectangle in inches.
    A claim like #123 @ 3,2: 5x4 means that claim ID 123 specifies a rectangle 3 inches from the left edge, 2 inches from the top edge, 5 inches wide, and 4 inches tall. Visually, it claims the square inches of fabric represented by # (and ignores the square inches of fabric represented by .) in the diagram below:

    ...........
    ...........
    ...#####...
    ...#####...
    ...#####...
    ...#####...
    ...........
    ...........
    ...........
    The problem is that many of the claims overlap, causing two or more claims to cover part of the same areas. For example, consider the following claims:

    #1 @ 1,3: 4x4
    #2 @ 3,1: 4x4
    #3 @ 5,5: 2x2
    Visually, these claim the following areas:

    ........
    ...2222.
    ...2222.
    .11XX22.
    .11XX22.
    .111133.
    .111133.
    ........
    The four square inches marked with X are claimed by both 1 and 2. (Claim 3, while adjacent to the others, does not overlap either of them.)

    If the Elves all proceed with their own plans, none of them will have enough fabric. How many square inches of fabric are within two or more claims?
*/

use regex::Regex;
use std::collections::HashMap;

struct Claim {
    id: u32,
    location: (u32, u32),
    size: (u32, u32),
}

impl Claim {
    fn from_string(input: &str) -> Self {
        let re = Regex::new(r"#(\d+) @ (\d+),(\d+): (\d+)x(\d+)").unwrap();
        let caps = re.captures(input).unwrap();

        Self {
            id: caps[1].parse::<u32>().unwrap(),
            location: (
                caps[2].parse::<u32>().unwrap(),
                caps[3].parse::<u32>().unwrap(),
            ),
            size: (
                caps[4].parse::<u32>().unwrap(),
                caps[5].parse::<u32>().unwrap(),
            ),
        }
    }
}

struct Fabric {
    area: HashMap<(u32, u32), u32>,
}

impl Fabric {
    fn from_claims(claims: &[Claim]) -> Self {
        let mut area = HashMap::new();

        for claim in claims {
            for y in 0..claim.size.1 {
                for x in 0..claim.size.0 {
                    let location = (claim.location.0 + x, claim.location.1 + y);
                    let count = area.entry(location).or_insert(0);
                    *count += 1;
                }
            }
        }

        Self { area }
    }

    fn count_overlap(&self) -> u32 {
        self.area.values().filter(|&&count| count > 1).count() as u32
    }
}

#[aoc(day3, part1)]
pub fn solve(input: &str) -> u32 {
    let claims: Vec<Claim> = input.lines().map(|line| Claim::from_string(line)).collect();

    let fabric = Fabric::from_claims(&claims);
    let overlap = fabric.count_overlap();
    println!("Overlap: {}", overlap);
    assert_eq!(overlap, 104241);
    overlap
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_count_overlap() {
        let claims = vec![
            Claim::from_string("#1 @ 1,3: 4x4"),
            Claim::from_string("#2 @ 3,1: 4x4"),
            Claim::from_string("#3 @ 5,5: 2x2"),
        ];
        let fabric = Fabric::from_claims(&claims);
        let overlap = fabric.count_overlap();
        assert_eq!(overlap, 4);
    }
}
