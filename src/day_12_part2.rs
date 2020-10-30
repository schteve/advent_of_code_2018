/*
    --- Part Two ---
    You realize that 20 generations aren't enough. After all, these plants will need to last another 1500 years to even reach your timeline, not to mention your future.

    After fifty billion (50000000000) generations, what is the sum of the numbers of all pots which contain a plant?
*/

use regex::Regex;
use std::fmt;

#[derive(Clone)]
struct Pots {
    pots: Vec<bool>,
    first_pot: i64,
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
                pots,
                first_pot: 0,
            }
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

    fn sum(&self) -> i64 {
        self.pots.iter()
            .enumerate()
            .map(|(i, &pot)| if pot == true { self.first_pot + i as i64 } else { 0 })
            .sum()
    }
}

impl fmt::Display for Pots {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let (pots, _) = self.trim();
        for &pot in pots {
            write!(f, "{}", if pot == true { '#' } else { '.' })?;
        }
        Ok(())
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
            rules,
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

    fn step(&mut self, count: u64) {
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
            next.first_pot = current.first_pot + slice_offset as i64 - 2;
            std::mem::swap(&mut current, &mut next); // Swap the references without moving the memory
        }
        self.pots = current.clone();
    }
}

// Currently this takes about 12 hours to run.
// Ideas for future improvement:
// * Make rules process more than 1 pot at a time, by increasing the 'size' of the rule. E.g. instead of 5 bits,
//   make it 15 and it would process 11 pots at a time. This approach is limited by memory usage.
// * Use a more compact data structure than Vec<bool> for the pots such as packed bits
#[aoc(day12, part2)]
pub fn solve(input: &str) -> i64 {
    let mut tunnel = Tunnel::from_string(input);
    tunnel.step(50_000_000_000);

    let sum_pots = tunnel.pots.sum();
    println!("Sum of pots: {}", sum_pots);
    assert_eq!(sum_pots, 4800000001087);
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
