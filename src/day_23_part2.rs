/*
    --- Part Two ---
    Now, you just need to figure out where to position yourself so that you're actually teleported when the nanobots activate.

    To increase the probability of success, you need to find the coordinate which puts you in range of the largest number of nanobots. If there are multiple, choose one closest to your position (0,0,0, measured by manhattan distance).

    For example, given the following nanobot formation:

    pos=<10,12,12>, r=2
    pos=<12,14,12>, r=2
    pos=<16,12,12>, r=4
    pos=<14,14,14>, r=6
    pos=<50,50,50>, r=200
    pos=<10,10,10>, r=5
    Many coordinates are in range of some of the nanobots in this formation. However, only the coordinate 12,12,12 is in range of the most nanobots: it is in range of the first five, but is not in range of the nanobot at 10,10,10. (All other coordinates are in range of fewer than five nanobots.) This coordinate's distance from 0,0,0 is 36.

    Find the coordinates that are in range of the largest number of nanobots. What is the shortest manhattan distance between any of those points and 0,0,0?
*/

use lazy_static::lazy_static;
use regex::Regex;
use std::cmp;

fn manhattan(a: (i32, i32, i32), b: (i32, i32, i32)) -> u32 {
    ((a.0 - b.0).abs() + (a.1 - b.1).abs() + (a.2 - b.2).abs()) as u32
}

#[derive(Clone, Copy, Debug, PartialEq)]
struct NanoBot {
    position: (i32, i32, i32),
    signal_radius: u32,
}

impl NanoBot {
    fn from_string(input: &str) -> Self {
        lazy_static! {
            static ref RE: Regex = Regex::new(r"pos=<(-?\d+),(-?\d+),(-?\d+)>, r=(\d+)").unwrap();
        }
        let caps = RE.captures(input.trim()).unwrap();

        let x = caps[1].parse::<i32>().unwrap();
        let y = caps[2].parse::<i32>().unwrap();
        let z = caps[3].parse::<i32>().unwrap();
        let r = caps[4].parse::<u32>().unwrap();

        Self {
            position: (x, y, z),
            signal_radius: r,
        }
    }

    fn is_point_in_range(&self, p: (i32, i32, i32)) -> bool {
        manhattan(self.position, p) <= self.signal_radius
    }

    fn vertices(&self) -> [(i32, i32, i32); 6] {
        let p = self.position; // Alias
        let r = self.signal_radius as i32; // Alias
        [
            (p.0 - r, p.1, p.2),
            (p.0 + r, p.1, p.2),
            (p.0, p.1 - r, p.2),
            (p.0, p.1 + r, p.2),
            (p.0, p.1, p.2 - r),
            (p.0, p.1, p.2 + r),
        ]
    }

    fn to_octahedron(&self) -> Octahedron {
        let p = self.position; // Alias
        let r = self.signal_radius as i32; // Alias

        // Side length is equal in each direction, making this a regular octahedron.
        // The value for each axis is defined in the Octahedron struct, +/- the signal radius (side length).
        let a = p.0 + p.1 + p.2;
        let b = p.0 - p.1 + p.2;
        let c = -p.0 + p.1 + p.2;
        let d = -p.0 - p.1 + p.2;

        Octahedron {
            a: (a - r, a + r),
            b: (b - r, b + r),
            c: (c - r, c + r),
            d: (d - r, d + r),
        }
    }
}

#[derive(Debug, PartialEq)]
struct Octahedron {
    a: (i32, i32), // axis along +x, +y (min, max)
    b: (i32, i32), // axis along +x, -y (min, max)
    c: (i32, i32), // axis along -x, +y (min, max)
    d: (i32, i32), // axis along -x, -y (min, max)
}

impl Octahedron {
    fn intersection(&self, other: &Self) -> Option<Self> {
        // I'm assuming the intersection of two octahedrons is always an octahedron, and the easiest way to define one is
        // by four sets of parallel planes (each plane defined as a fixed manhattan distance in a certain direction).
        // Therefore the intersection of two octahedrons is the 'inner' of the two corresponding planes for each value.
        // See https://github.com/bryanmcnett/aabo
        if self.a.0 > other.a.1
            || other.a.0 > self.a.1
            || self.b.0 > other.b.1
            || other.b.0 > self.b.1
            || self.c.0 > other.c.1
            || other.c.0 > self.c.1
            || self.d.0 > other.d.1
            || other.d.0 > self.d.1
        {
            // The two octahedrons do not intersect if there is no overlap between them on any axis
            return None;
        }

        let a0 = cmp::max(self.a.0, other.a.0);
        let a1 = cmp::min(self.a.1, other.a.1);
        let b0 = cmp::max(self.b.0, other.b.0);
        let b1 = cmp::min(self.b.1, other.b.1);
        let c0 = cmp::max(self.c.0, other.c.0);
        let c1 = cmp::min(self.c.1, other.c.1);
        let d0 = cmp::max(self.d.0, other.d.0);
        let d1 = cmp::min(self.d.1, other.d.1);

        Some(Self {
            a: (a0, a1),
            b: (b0, b1),
            c: (c0, c1),
            d: (d0, d1),
        })
    }

    fn distance_to_origin(&self) -> u32 {
        // The closest point in an octahedron to the origin is always going to be composed of the closest coordinate
        // on each axis. If the plane pair for an axis straddles the origin, then 0 is the closest coordinate.
        // The distance to this point is then the value of the coordinate with the largest absolute value (since the
        // other coordinates are already reached within that same distance).
        fn axis_dist(planes: (i32, i32)) -> u32 {
            if planes.0 <= 0 && planes.1 >= 0 {
                0
            } else {
                cmp::min(planes.0.abs(), planes.1.abs()) as u32
            }
        }

        let a_abs = axis_dist(self.a);
        let b_abs = axis_dist(self.b);
        let c_abs = axis_dist(self.c);
        let d_abs = axis_dist(self.d);

        cmp::max(cmp::max(cmp::max(a_abs, b_abs), c_abs), d_abs)
    }
}

#[derive(Clone, Debug, PartialEq)]
struct Region {
    x: (i32, i32),
    y: (i32, i32),
    z: (i32, i32),
    bots: Vec<NanoBot>,
}

impl Region {
    fn split(&self) -> Vec<Self> {
        fn split_range(range: (i32, i32)) -> [(i32, i32); 2] {
            if range.0 == range.1 {
                [(range.0, range.0), (range.1, range.1)]
            } else {
                let mid = (range.0 + range.1) / 2;
                [(range.0, mid), (mid + 1, range.1)]
            }
        }

        let x_range = split_range(self.x);
        let y_range = split_range(self.y);
        let z_range = split_range(self.z);

        let mut output: Vec<Region> = Vec::new();
        for &x in &x_range {
            for &y in &y_range {
                for &z in &z_range {
                    output.push(Region {
                        x,
                        y,
                        z,
                        bots: Vec::new(),
                    });
                }
            }
        }
        output.sort_unstable_by(|a, b| (a.x, a.y, a.z).cmp(&(b.x, b.y, b.z)));
        output.dedup();

        for &bot in &self.bots {
            for r in output.iter_mut() {
                if r.intersects(&bot) == true {
                    r.bots.push(bot);
                }
            }
        }
        output
    }

    fn is_point_in_range(&self, p: (i32, i32, i32)) -> bool {
        p.0 >= self.x.0
            && p.0 <= self.x.1
            && p.1 >= self.y.0
            && p.1 <= self.y.1
            && p.2 >= self.z.0
            && p.2 <= self.z.1
    }

    fn vertices(&self) -> [(i32, i32, i32); 8] {
        [
            (self.x.0, self.y.0, self.z.0),
            (self.x.0, self.y.0, self.z.1),
            (self.x.0, self.y.1, self.z.0),
            (self.x.0, self.y.1, self.z.1),
            (self.x.1, self.y.0, self.z.0),
            (self.x.1, self.y.0, self.z.1),
            (self.x.1, self.y.1, self.z.0),
            (self.x.1, self.y.1, self.z.1),
        ]
    }

    fn intersects(&self, bot: &NanoBot) -> bool {
        // If any bot vertex (octahedron) is inside the region (rectangular prism), they intersect
        for &v in &bot.vertices() {
            if self.is_point_in_range(v) == true {
                return true;
            }
        }

        // If any region vertex is within range of the bot, they intersect
        for &v in &self.vertices() {
            if bot.is_point_in_range(v) == true {
                return true;
            }
        }

        false
    }
}

struct Swarm {
    bots: Vec<NanoBot>,
}

impl Swarm {
    fn from_string(input: &str) -> Self {
        let bots: Vec<NanoBot> = input.lines().map(NanoBot::from_string).collect();
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

    fn bounding_box(&self) -> Region {
        let min = self.bots.iter().fold((0, 0, 0), |best, bot| {
            (
                cmp::min(best.0, bot.position.0),
                cmp::min(best.1, bot.position.1),
                cmp::min(best.2, bot.position.2),
            )
        });
        let max = self.bots.iter().fold((0, 0, 0), |best, bot| {
            (
                cmp::max(best.0, bot.position.0),
                cmp::max(best.1, bot.position.1),
                cmp::max(best.2, bot.position.2),
            )
        });

        Region {
            x: (min.0, max.0),
            y: (min.1, max.1),
            z: (min.2, max.2),
            bots: self.bots.clone(),
        }
    }

    fn find_distance_to_points_in_range_of_most(&self) -> u32 {
        let mut regions: Vec<Region> = vec![self.bounding_box()]; // The region with the most bots is always at the end
        let mut best_region: Option<Region> = None; // In this region, all bots mutually intersect and it is the smallest such region

        // Always look at the region with the most bots
        while let Some(r) = regions.pop() {
            // First check if the candidate region even contains enough bots to contend with the best region we've seen so far
            if let Some(br) = &best_region {
                if r.bots.len() < br.bots.len() {
                    // The candidate region has fewer bots than the best we've seen so far, so we can just discard it now
                    continue;
                }
            }

            // Next check what it looks like when we intersect all bots in the region
            if let Some(_oct) = Swarm::intersect_all(&r.bots) {
                // There is an intersection of all bots in this region
                if let Some(br) = &best_region {
                    if r.bots.len() >= br.bots.len() {
                        // Found an even better region (more bots or smaller octahedron)
                        best_region = Some(r.clone());
                    } else {
                        panic!("Shouldn't reach here if smaller regions are properly filtered out");
                    }
                } else {
                    // This is the first region we've found in which all bots intersect
                    best_region = Some(r.clone());
                }
            }

            // Region should be split further: either it contains multiple groups of bots, or it's a single group that we want to find a more specific region for
            let sub_regions = r.split();
            for sr in sub_regions {
                if let Some(br) = &best_region {
                    if sr != r && sr.bots.len() >= br.bots.len() {
                        regions.push(sr);
                    }
                } else {
                    regions.push(sr);
                }
            }

            // Remove any duplicate regions
            regions.sort_unstable_by(|a, b| {
                (a.bots.len(), a.x, a.y, a.z).cmp(&(b.bots.len(), b.x, b.y, b.z))
            }); // Sort first by number of bots, then x,y,z
            regions.dedup();
        }

        // We have the best region, now find the distance to it from the origin
        let oct = Swarm::intersect_all(&best_region.unwrap().bots).unwrap();
        oct.distance_to_origin()
    }

    fn intersect_all(bots: &[NanoBot]) -> Option<Octahedron> {
        let mut bots_iter = bots.iter();
        let mut oct = bots_iter.next().unwrap().to_octahedron();

        for bot in bots_iter {
            if let Some(new_oct) = oct.intersection(&bot.to_octahedron()) {
                // There is still a valid volume of intersection, store it and continue
                oct = new_oct;
            } else {
                // If any intersection fails, then there is no overall intersection
                return None;
            }
        }

        Some(oct)
    }
}

#[aoc(day23, part2)]
pub fn solve(input: &str) -> u32 {
    let swarm = Swarm::from_string(input);
    let best_dist = swarm.find_distance_to_points_in_range_of_most();
    println!("Distance to best point: {}", best_dist);
    assert_eq!(best_dist, 88894457);
    best_dist
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

    #[test]
    fn test_swarm_intersection() {
        let input = "\
            pos=<10,12,12>, r=2
            pos=<12,14,12>, r=2
            pos=<16,12,12>, r=4
            pos=<14,14,14>, r=6
            pos=<50,50,50>, r=200
            pos=<10,10,10>, r=5";
        let swarm = Swarm::from_string(input);

        // All bots do not intersect
        let volume = Swarm::intersect_all(&swarm.bots);
        assert_eq!(volume, None);

        // However remove the last one and they do
        let mut bots = swarm.bots.clone();
        bots.pop();
        let volume = Swarm::intersect_all(&bots);
        assert_eq!(
            volume,
            Some(Octahedron {
                a: (36, 36),
                b: (12, 12),
                c: (12, 12),
                d: (-12, -12),
            })
        )
    }

    #[test]
    fn find_distance_to_points_in_range_of_most() {
        let input = "\
            pos=<10,12,12>, r=2
            pos=<12,14,12>, r=2
            pos=<16,12,12>, r=4
            pos=<14,14,14>, r=6
            pos=<50,50,50>, r=200
            pos=<10,10,10>, r=5";
        let swarm = Swarm::from_string(input);
        let best_dist = swarm.find_distance_to_points_in_range_of_most();
        assert_eq!(best_dist, 36);
    }
}
