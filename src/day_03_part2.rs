/*
    --- Part Two ---
    Amidst the chaos, you notice that exactly one claim doesn't overlap by even a single square inch of fabric with any other claim. If you can somehow draw attention to it, maybe the Elves will be able to make Santa's suit after all!

    For example, in the claims above, only claim 3 is intact after all claims are made.

    What is the ID of the only claim that doesn't overlap?
*/

use crate::common::Point;
use nom::{
    character::complete::{char, digit1, space1},
    combinator::map_res,
    sequence::{preceded, separated_pair, tuple},
    IResult,
};
use std::collections::HashMap;

struct Claim {
    id: u32,
    location: Point,
    size: (u32, u32),
}

impl Claim {
    fn from_string(input: &str) -> Self {
        Self::parser(input).unwrap().1
    }

    fn parser(input: &str) -> IResult<&str, Self> {
        let (input, (_, id, _, _, location, _, size)) = tuple((
            char('#'),
            map_res(digit1, |id: &str| id.parse::<u32>()),
            space1,
            char('@'),
            Point::parser,
            char(':'),
            preceded(
                space1,
                separated_pair(
                    map_res(digit1, |size0: &str| size0.parse::<u32>()),
                    char('x'),
                    map_res(digit1, |size1: &str| size1.parse::<u32>()),
                ),
            ),
        ))(input)?;

        Ok((input, Self { id, location, size }))
    }
}

struct Fabric {
    area: HashMap<Point, u32>,
}

impl Fabric {
    fn from_claims(claims: &[Claim]) -> Self {
        let mut area = HashMap::new();

        for claim in claims {
            for y in 0..claim.size.1 as i32 {
                for x in 0..claim.size.0 as i32 {
                    let location = claim.location + Point { x, y };
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

    fn is_claim_intact(&self, claim: &Claim) -> bool {
        for y in 0..claim.size.1 as i32 {
            for x in 0..claim.size.0 as i32 {
                let location = claim.location + Point { x, y };
                if self.area[&location] != 1 {
                    return false;
                }
            }
        }
        true
    }

    fn find_intact_claim(&self, claims: &[Claim]) -> u32 {
        for claim in claims {
            let intact = self.is_claim_intact(claim);
            if intact == true {
                return claim.id;
            }
        }
        panic!("No intact claim found");
    }
}

#[aoc(day3, part2)]
pub fn solve(input: &str) -> u32 {
    let claims: Vec<Claim> = input.lines().map(Claim::from_string).collect();

    let fabric = Fabric::from_claims(&claims);
    let intact = fabric.find_intact_claim(&claims);
    println!("Intact claim ID: {}", intact);
    assert_eq!(intact, 806);
    intact
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

    #[test]
    fn test_find_intact_claim() {
        let claims = vec![
            Claim::from_string("#1 @ 1,3: 4x4"),
            Claim::from_string("#2 @ 3,1: 4x4"),
            Claim::from_string("#3 @ 5,5: 2x2"),
        ];
        let fabric = Fabric::from_claims(&claims);
        let intact = fabric.find_intact_claim(&claims);
        assert_eq!(intact, 3);
    }
}
