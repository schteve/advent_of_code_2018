/*
    --- Day 23: Experimental Emergency Teleportation ---
    Using your torch to search the darkness of the rocky cavern, you finally locate the man's friend: a small reindeer.

    You're not sure how it got so far in this cave. It looks sick - too sick to walk - and too heavy for you to carry all the way back. Sleighs won't be invented for another 1500 years, of course.

    The only option is experimental emergency teleportation.

    You hit the "experimental emergency teleportation" button on the device and push I accept the risk on no fewer than 18 different warning messages. Immediately, the device deploys hundreds of tiny nanobots which fly around the cavern, apparently assembling themselves into a very specific formation. The device lists the X,Y,Z position (pos) for each nanobot as well as its signal radius (r) on its tiny screen (your puzzle input).

    Each nanobot can transmit signals to any integer coordinate which is a distance away from it less than or equal to its signal radius (as measured by Manhattan distance). Coordinates a distance away of less than or equal to a nanobot's signal radius are said to be in range of that nanobot.

    Before you start the teleportation process, you should determine which nanobot is the strongest (that is, which has the largest signal radius) and then, for that nanobot, the total number of nanobots that are in range of it, including itself.

    For example, given the following nanobots:

    pos=<0,0,0>, r=4
    pos=<1,0,0>, r=1
    pos=<4,0,0>, r=3
    pos=<0,2,0>, r=1
    pos=<0,5,0>, r=3
    pos=<0,0,3>, r=1
    pos=<1,1,1>, r=1
    pos=<1,1,2>, r=1
    pos=<1,3,1>, r=1
    The strongest nanobot is the first one (position 0,0,0) because its signal radius, 4 is the largest. Using that nanobot's location and signal radius, the following nanobots are in or out of range:

    The nanobot at 0,0,0 is distance 0 away, and so it is in range.
    The nanobot at 1,0,0 is distance 1 away, and so it is in range.
    The nanobot at 4,0,0 is distance 4 away, and so it is in range.
    The nanobot at 0,2,0 is distance 2 away, and so it is in range.
    The nanobot at 0,5,0 is distance 5 away, and so it is not in range.
    The nanobot at 0,0,3 is distance 3 away, and so it is in range.
    The nanobot at 1,1,1 is distance 3 away, and so it is in range.
    The nanobot at 1,1,2 is distance 4 away, and so it is in range.
    The nanobot at 1,3,1 is distance 5 away, and so it is not in range.
    In this example, in total, 7 nanobots are in range of the nanobot with the largest signal radius.

    Find the nanobot with the largest signal radius. How many nanobots are in range of its signals?
*/

use nom::{
    bytes::complete::tag,
    character::complete::{digit1, multispace0, one_of},
    combinator::{map_res, recognize},
    multi::many1,
    sequence::{pair, preceded, tuple},
    IResult,
};

struct NanoBot {
    position: (i32, i32, i32),
    signal_radius: u32,
}

impl NanoBot {
    fn parser(input: &str) -> IResult<&str, Self> {
        let (input, (position, signal_radius)) = preceded(
            multispace0,
            pair(
                tuple((
                    preceded(
                        tag("pos=<"),
                        map_res(recognize(many1(one_of("-01234567890"))), |x: &str| {
                            x.parse::<i32>()
                        }),
                    ),
                    preceded(
                        tag(","),
                        map_res(recognize(many1(one_of("-01234567890"))), |y: &str| {
                            y.parse::<i32>()
                        }),
                    ),
                    preceded(
                        tag(","),
                        map_res(recognize(many1(one_of("-01234567890"))), |z: &str| {
                            z.parse::<i32>()
                        }),
                    ),
                )),
                preceded(tag(">, r="), map_res(digit1, |r: &str| r.parse::<u32>())),
            ),
        )(input)?;

        Ok((
            input,
            Self {
                position,
                signal_radius,
            },
        ))
    }

    fn is_point_in_range(&self, p: (i32, i32, i32)) -> bool {
        let distance = (self.position.0 - p.0).abs()
            + (self.position.1 - p.1).abs()
            + (self.position.2 - p.2).abs();
        distance as u32 <= self.signal_radius
    }
}

struct Swarm {
    bots: Vec<NanoBot>,
}

impl Swarm {
    fn from_string(input: &str) -> Self {
        let bots: Vec<NanoBot> = input
            .lines()
            .map(|line| NanoBot::parser(line).unwrap().1)
            .collect();
        Self { bots }
    }

    fn find_bots_in_range_of_strongest(&self) -> u32 {
        let strongest = self
            .bots
            .iter()
            .max_by_key(|bot| bot.signal_radius)
            .unwrap();
        self.bots
            .iter()
            .filter(|bot| strongest.is_point_in_range(bot.position))
            .count() as u32
    }
}

#[aoc(day23, part1)]
pub fn solve(input: &str) -> u32 {
    let swarm = Swarm::from_string(input);
    let in_range = swarm.find_bots_in_range_of_strongest();
    println!("Bots in range of strongest: {}", in_range);
    assert_eq!(in_range, 294);
    in_range
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn find_bots_in_range_of_strongest() {
        let input = "\
pos=<0,0,0>, r=4
pos=<1,0,0>, r=1
pos=<4,0,0>, r=3
pos=<0,2,0>, r=1
pos=<0,5,0>, r=3
pos=<0,0,3>, r=1
pos=<1,1,1>, r=1
pos=<1,1,2>, r=1
pos=<1,3,1>, r=1";
        let swarm = Swarm::from_string(input);
        let in_range = swarm.find_bots_in_range_of_strongest();
        assert_eq!(in_range, 7);
    }
}
