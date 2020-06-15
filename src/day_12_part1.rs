/*
    --- Day 12: Subterranean Sustainability ---
    The year 518 is significantly more underground than your history books implied. Either that, or you've arrived in a vast cavern network under the North Pole.

    After exploring a little, you discover a long tunnel that contains a row of small pots as far as you can see to your left and right. A few of them contain plants - someone is trying to grow things in these geothermally-heated caves.

    The pots are numbered, with 0 in front of you. To the left, the pots are numbered -1, -2, -3, and so on; to the right, 1, 2, 3.... Your puzzle input contains a list of pots from 0 to the right and whether they do (#) or do not (.) currently contain a plant, the initial state. (No other pots currently contain plants.) For example, an initial state of #..##.... indicates that pots 0, 3, and 4 currently contain plants.

    Your puzzle input also contains some notes you find on a nearby table: someone has been trying to figure out how these plants spread to nearby pots. Based on the notes, for each generation of plants, a given pot has or does not have a plant based on whether that pot (and the two pots on either side of it) had a plant in the last generation. These are written as LLCRR => N, where L are pots to the left, C is the current pot being considered, R are the pots to the right, and N is whether the current pot will have a plant in the next generation. For example:

    A note like ..#.. => . means that a pot that contains a plant but with no plants within two pots of it will not have a plant in it during the next generation.
    A note like ##.## => . means that an empty pot with two plants on each side of it will remain empty in the next generation.
    A note like .##.# => # means that a pot has a plant in a given generation if, in the previous generation, there were plants in that pot, the one immediately to the left, and the one two pots to the right, but not in the ones immediately to the right and two to the left.
    It's not clear what these plants are for, but you're sure it's important, so you'd like to make sure the current configuration of plants is sustainable by determining what will happen after 20 generations.

    For example, given the following input:

    initial state: #..#.#..##......###...###

    ...## => #
    ..#.. => #
    .#... => #
    .#.#. => #
    .#.## => #
    .##.. => #
    .#### => #
    #.#.# => #
    #.### => #
    ##.#. => #
    ##.## => #
    ###.. => #
    ###.# => #
    ####. => #
    For brevity, in this example, only the combinations which do produce a plant are listed. (Your input includes all possible combinations.) Then, the next 20 generations will look like this:

                    1         2         3
        0         0         0         0
    0: ...#..#.#..##......###...###...........
    1: ...#...#....#.....#..#..#..#...........
    2: ...##..##...##....#..#..#..##..........
    3: ..#.#...#..#.#....#..#..#...#..........
    4: ...#.#..#...#.#...#..#..##..##.........
    5: ....#...##...#.#..#..#...#...#.........
    6: ....##.#.#....#...#..##..##..##........
    7: ...#..###.#...##..#...#...#...#........
    8: ...#....##.#.#.#..##..##..##..##.......
    9: ...##..#..#####....#...#...#...#.......
    10: ..#.#..#...#.##....##..##..##..##......
    11: ...#...##...#.#...#.#...#...#...#......
    12: ...##.#.#....#.#...#.#..##..##..##.....
    13: ..#..###.#....#.#...#....#...#...#.....
    14: ..#....##.#....#.#..##...##..##..##....
    15: ..##..#..#.#....#....#..#.#...#...#....
    16: .#.#..#...#.#...##...#...#.#..##..##...
    17: ..#...##...#.#.#.#...##...#....#...#...
    18: ..##.#.#....#####.#.#.#...##...##..##..
    19: .#..###.#..#.#.#######.#.#.#..#.#...#..
    20: .#....##....#####...#######....#.#..##.
    The generation is shown along the left, where 0 is the initial state. The pot numbers are shown along the top, where 0 labels the center pot, negative-numbered pots extend to the left, and positive pots extend toward the right. Remember, the initial state begins at pot 0, which is not the leftmost pot used in this example.

    After one generation, only seven plants remain. The one in pot 0 matched the rule looking for ..#.., the one in pot 4 matched the rule looking for .#.#., pot 9 matched .##.., and so on.

    In this example, after 20 generations, the pots shown as # contain plants, the furthest left of which is pot -2, and the furthest right of which is pot 34. Adding up all the numbers of plant-containing pots after the 20th generation produces 325.

    After 20 generations, what is the sum of the numbers of all pots which contain a plant?
*/

use regex::Regex;

#[derive(Clone)]
struct Pots {
    pots: Vec<bool>,
    first_pot: i32,
}

impl Pots {
    fn new() -> Self {
        Self {
            pots: Vec::new(),
            first_pot: 0,
        }
    }

    fn from_string(input: &str) -> Self {
        let pots: Vec<bool> = input.chars()
            .map(|c| c == '#')
            .collect();

            Self {
                pots: pots,
                first_pot: 0,
            }
    }

    fn to_string(&self) -> String {
        let (pots, _) = self.trim();
        pots.iter()
            .map(|&pot| if pot == true { '#' } else { '.' })
            .collect()
    }

    fn trim(&self) -> (&[bool], usize) {
        let mut begin = 0;
        let mut end = self.pots.len();

        for &pot in self.pots.iter() {
            if pot == false {
                begin += 1;
            } else {
                break;
            }
        }

        for &pot in self.pots.iter().rev() {
            if pot == false {
                end -= 1;
            } else {
                break;
            }
        }

        (&self.pots[begin..end], begin)
    }

    fn sum(&self) -> i32 {
        self.pots.iter()
            .enumerate()
            .map(|(i, &pot)| if pot == true { self.first_pot + i as i32 } else { 0 })
            .sum()
    }
}

struct Tunnel {
    pots: Pots,
    rules: Vec<bool>
}

impl Tunnel {
    fn from_string(input: &str) -> Self {
        let re = Regex::new(r"initial state: ([#\.]+)").unwrap();
        let caps = re.captures(input).unwrap();

        let re = Regex::new(r"([#\.]+) => ([#\.])").unwrap();
        let rules_list: Vec<i32> = re.captures_iter(input)
            .filter(|cap| &cap[2] == "#")
            .map(|cap| Tunnel::rule_id_from_string(&cap[1]))
            .collect();
        let rules: Vec<bool> = (0..32)
            .map(|i| rules_list.contains(&i))
            .collect();

        Self {
            pots: Pots::from_string(&caps[1]),
            rules: rules,
        }
    }

    fn rule_id_from_string(input: &str) -> i32 {
        let mut output = 0;
        for c in input.chars() {
            output <<= 1;
            if c == '#' {
                output |= 1;
            }
        }
        output
    }

    fn step(&mut self, count: u32) {
        // Use double buffering to reduce memory allocation overhead
        let mut current: &mut Pots = &mut self.pots.clone();
        let mut next: &mut Pots = &mut Pots::new();

        for _ in 0..count {
            // For each pot that could be a plant, calculate which rule ID to use and then apply it.
            next.pots.clear();

            // ID always starts at 0 by definition (first pot we check is ....# and the last bit is added in the loop)
            let mut rule_id = 0;

            // We will iterate over all existing pots but need to account for 2 pots before and 2 after.
            // Each of those pots looks 2 left and 2 right so we need to produce 8 additional pots in the iterator.
            // However, we skip the first 4 since they will always be false and rule_id can start at 0 by definition.
            // So we only need 4 extra pots total.
            let (pots, slice_offset) = current.trim();
            for &pot in pots.iter().chain(std::iter::repeat(&false).take(4)) {
                rule_id = (rule_id << 1) & 0x1F; // Keep only 5 bits
                if pot == true { // The new pot is the rightmost (2 from the center)
                    rule_id |= 1;
                }
                next.pots.push(self.rules[rule_id]);
            }

            // Record the change in the first pot offset, if any. Swap buffers.
            next.first_pot = current.first_pot + slice_offset as i32 - 2;
            std::mem::swap(&mut current, &mut next); // Swap the references without moving the memory
        }
        self.pots = current.clone();
    }
}

#[aoc(day12, part1)]
pub fn solve(input: &str) -> i32 {
    let mut tunnel = Tunnel::from_string(input);
    tunnel.step(20);
    //println!("Tunnel: {}", tunnel.to_string());

    let sum_pots = tunnel.pots.sum();
    //println!("Sum of pots: {}", sum_pots);
    assert_eq!(sum_pots, 3890);
    sum_pots
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_step() {
        let input = "
initial state: #..#.#..##......###...###

...## => #
..#.. => #
.#... => #
.#.#. => #
.#.## => #
.##.. => #
.#### => #
#.#.# => #
#.### => #
##.#. => #
##.## => #
###.. => #
###.# => #
####. => #";
        let mut tunnel = Tunnel::from_string(input);
        assert_eq!(tunnel.pots.to_string(), "#..#.#..##......###...###");

        let expect_strings = vec![
            "#...#....#.....#..#..#..#",
            "##..##...##....#..#..#..##",
            "#.#...#..#.#....#..#..#...#",
            "#.#..#...#.#...#..#..##..##",
            "#...##...#.#..#..#...#...#",
            "##.#.#....#...#..##..##..##",
            "#..###.#...##..#...#...#...#",
            "#....##.#.#.#..##..##..##..##",
            "##..#..#####....#...#...#...#",
            "#.#..#...#.##....##..##..##..##",
            "#...##...#.#...#.#...#...#...#",
            "##.#.#....#.#...#.#..##..##..##",
            "#..###.#....#.#...#....#...#...#",
            "#....##.#....#.#..##...##..##..##",
            "##..#..#.#....#....#..#.#...#...#",
            "#.#..#...#.#...##...#...#.#..##..##",
            "#...##...#.#.#.#...##...#....#...#",
            "##.#.#....#####.#.#.#...##...##..##",
            "#..###.#..#.#.#######.#.#.#..#.#...#",
            "#....##....#####...#######....#.#..##",
        ];
        for s in expect_strings {
            tunnel.step(1);
            assert_eq!(tunnel.pots.to_string(), s);
        }
    }

    #[test]
    fn test_pots_sum() {
        let input = "
initial state: #..#.#..##......###...###

...## => #
..#.. => #
.#... => #
.#.#. => #
.#.## => #
.##.. => #
.#### => #
#.#.# => #
#.### => #
##.#. => #
##.## => #
###.. => #
###.# => #
####. => #";
        let mut tunnel = Tunnel::from_string(input);
        tunnel.step(20);
        assert_eq!(tunnel.pots.sum(), 325);
    }
}
