/*
    --- Day 25: Four-Dimensional Adventure ---
    The reindeer's symptoms are getting worse, and neither you nor the white-bearded man have a solution. At least the reindeer has a warm place to rest: a small bed near where you're sitting.

    As you reach down, the reindeer looks up at you, accidentally bumping a button on your wrist-mounted device with its nose in the process - a button labeled "help".

    "Hello, and welcome to the Time Travel Support Hotline! If you are lost in time and space, press 1. If you are trapped in a time paradox, press 2. If you need help caring for a sick reindeer, press 3. If you--"

    Beep.

    A few seconds later, you hear a new voice. "Hello; please state the nature of your reindeer." You try to describe the situation.

    "Just a moment, I think I can remotely run a diagnostic scan." A beam of light projects from the device and sweeps over the reindeer a few times.

    "Okay, it looks like your reindeer is very low on magical energy; it should fully recover if we can fix that. Let me check your timeline for a source.... Got one. There's actually a powerful source of magical energy about 1000 years forward from you, and at roughly your position, too! It looks like... hot chocolate? Anyway, you should be able to travel there to pick some up; just don't forget a mug! Is there anything else I can help you with today?"

    You explain that your device isn't capable of going forward in time. "I... see. That's tricky. Well, according to this information, your device should have the necessary hardware to open a small portal and send some hot chocolate back to you. You'll need a list of fixed points in spacetime; I'm transmitting it to you now."

    "You just need to align your device to the constellations of fixed points so that it can lock on to the destination and open the portal. Let me look up how much hot chocolate that breed of reindeer needs."

    "It says here that your particular reindeer is-- this can't be right, it says there's only one like that in the universe! But THAT means that you're--" You disconnect the call.

    The list of fixed points in spacetime (your puzzle input) is a set of four-dimensional coordinates. To align your device, acquire the hot chocolate, and save the reindeer, you just need to find the number of constellations of points in the list.

    Two points are in the same constellation if their manhattan distance apart is no more than 3 or if they can form a chain of points, each a manhattan distance no more than 3 from the last, between the two of them. (That is, if a point is close enough to a constellation, it "joins" that constellation.) For example:

    0,0,0,0
    3,0,0,0
    0,3,0,0
    0,0,3,0
    0,0,0,3
    0,0,0,6
    9,0,0,0
    12,0,0,0
    In the above list, the first six points form a single constellation: 0,0,0,0 is exactly distance 3 from the next four, and the point at 0,0,0,6 is connected to the others by being 3 away from 0,0,0,3, which is already in the constellation. The bottom two points, 9,0,0,0 and 12,0,0,0 are in a separate constellation because no point is close enough to connect them to the first constellation. So, in the above list, the number of constellations is 2. (If a point at 6,0,0,0 were present, it would connect 3,0,0,0 and 9,0,0,0, merging all of the points into a single giant constellation instead.)

    In this example, the number of constellations is 4:

    -1,2,2,0
    0,0,2,-2
    0,0,0,-2
    -1,2,0,0
    -2,-2,-2,2
    3,0,2,-1
    -1,3,2,2
    -1,0,-1,0
    0,2,1,-2
    3,0,0,0
    In this one, it's 3:

    1,-1,0,1
    2,0,-1,0
    3,2,-1,0
    0,0,3,1
    0,0,-1,-1
    2,3,-2,0
    -2,2,0,0
    2,-2,0,-1
    1,-1,0,-1
    3,2,0,2
    Finally, in this one, it's 8:

    1,-1,-1,-2
    -2,-2,0,1
    0,2,1,3
    -2,3,-2,1
    0,2,3,-2
    -1,-1,1,-2
    0,-2,-1,0
    -2,2,3,-1
    1,2,2,0
    -1,-2,0,-2
    The portly man nervously strokes his white beard. It's time to get that hot chocolate.

    How many constellations are formed by the fixed points in spacetime?
*/

fn manhattan(a: &Point4D, b: &Point4D) -> u32 {
    ((a.0 - b.0).abs() + (a.1 - b.1).abs() + (a.2 - b.2).abs() + (a.3 - b.3).abs()) as u32
}

#[derive(Clone, Copy, Debug)]
struct Point4D(i32, i32, i32, i32);

impl Point4D {
    fn from_string(input: &str) -> Self {
        let mut parts = input.split(',');
        Self(
            parts.next().unwrap().trim().parse::<i32>().unwrap(),
            parts.next().unwrap().trim().parse::<i32>().unwrap(),
            parts.next().unwrap().trim().parse::<i32>().unwrap(),
            parts.next().unwrap().trim().parse::<i32>().unwrap(),
        )
    }

    fn many_from_string(input: &str) -> Vec<Self> {
        input.lines().map(Self::from_string).collect()
    }
}

#[derive(Debug)]
struct Constellation {
    stars: Vec<Point4D>,
}

impl Constellation {
    fn new() -> Self {
        Self { stars: Vec::new() }
    }

    fn touches_point(&self, p: &Point4D) -> bool {
        for star in &self.stars {
            if manhattan(p, star) <= 3 {
                return true;
            }
        }
        false
    }

    fn add_point(&mut self, p: Point4D) {
        self.stars.push(p);
    }

    fn add_points(&mut self, points: &[Point4D]) {
        self.stars.extend(points);
    }
}

fn form_constellations(points: &[Point4D]) -> Vec<Constellation> {
    let mut constellations: Vec<Constellation> = Vec::new();
    for p in points {
        // First check which existing constellations this point might belong to
        let mut marked: Vec<usize> = Vec::new();
        for (i, c) in constellations.iter().enumerate() {
            if c.touches_point(p) == true {
                marked.push(i);
            }
        }

        // Then modify or create constellations
        match marked.len() {
            0 => {
                // No overlap, make a new constellation
                let mut c = Constellation::new();
                c.add_point(*p);
                constellations.push(c);
            }
            1 => {
                // Overlap with exactly one constellation, add the point to it
                constellations[marked[0]].add_point(*p);
            }
            _ => {
                // Overlap with multiple constellations, add the point to the first one then combine all others into it
                let origin_idx = marked[0];
                constellations[origin_idx].add_point(*p);
                for &m in marked.iter().skip(1).rev() {
                    let stars_to_add = constellations[m].stars.clone();
                    constellations[origin_idx].add_points(&stars_to_add);
                    constellations.remove(m); // Remove this constellation since it has been combined into another one
                }
            }
        }
    }
    constellations
}

#[aoc(day25, part1)]
pub fn solve(input: &str) -> usize {
    let points: Vec<Point4D> = Point4D::many_from_string(input);
    let constellations = form_constellations(&points);
    let count = constellations.len();
    println!("Total constellations: {}", count);
    assert_eq!(count, 388);
    count
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_form_constellations() {
        let input = "\
0,0,0,0
3,0,0,0
0,3,0,0
0,0,3,0
0,0,0,3
0,0,0,6
9,0,0,0
12,0,0,0";
        let points: Vec<Point4D> = Point4D::many_from_string(input);
        let constellations = form_constellations(&points);
        assert_eq!(constellations.len(), 2);
    }
}
