/*
    --- Part Two ---
    Time to improve the polymer.

    One of the unit types is causing problems; it's preventing the polymer from collapsing as much as it should. Your goal is to figure out which unit type is causing the most problems, remove all instances of it (regardless of polarity), fully react the remaining polymer, and measure its length.

    For example, again using the polymer dabAcCaCBAcCcaDA from above:

    Removing all A/a units produces dbcCCBcCcD. Fully reacting this polymer produces dbCBcD, which has length 6.
    Removing all B/b units produces daAcCaCAcCcaDA. Fully reacting this polymer produces daCAcaDA, which has length 8.
    Removing all C/c units produces dabAaBAaDA. Fully reacting this polymer produces daDA, which has length 4.
    Removing all D/d units produces abAcCaCBAcCcaA. Fully reacting this polymer produces abCBAc, which has length 6.
    In this example, removing all C/c units was best, producing the answer 4.

    What is the length of the shortest polymer you can produce by removing all units of exactly one type and fully reacting the result?
*/

use std::fmt;

fn check_destroy(a: char, b: char) -> bool {
    if (a == b.to_ascii_lowercase() && a.to_ascii_uppercase() == b) ||
       (a == b.to_ascii_uppercase() && a.to_ascii_lowercase() == b) {
        true
    } else {
        false
    }
}

#[derive(Clone)]
struct Polymer {
    units: Vec<char>,
}

impl Polymer {
    fn from_string(input: &str) -> Self {
        Self {
            units: input.trim().chars().collect(),
        }
    }

    fn length(&self) -> u32 {
        self.units.len() as u32
    }

    fn react(&mut self) {
        // Decide which indexes to keep
        let mut keep: Vec<bool> = Vec::new();
        let mut skip_next = false;
        for (i, _c) in self.units.iter().enumerate() {
            if skip_next == false {
                if i + 1 < self.units.len() {
                    if check_destroy(self.units[i], self.units[i + 1]) == true {
                        // Pair is destroyed - mark both for removal now and set flag to skip next index
                        keep.push(false);
                        keep.push(false);
                        skip_next = true;
                    } else {
                        // Pair is not destroyed - keep first one (second will be checked next time)
                        keep.push(true);
                    }
                } else {
                    // Last element in the vector - keep it (nothing to pair with)
                    keep.push(true);
                }
            } else {
                skip_next = false;
            }
        }

        let mut i = 0;
        self.units.retain(|_| {
                let tmp = i;
                i += 1;
                keep[tmp]
            });
    }

    fn react_all(&mut self) {
        let mut prev_len = self.units.len();
        loop {
            self.react();

            if prev_len == self.units.len() {
                return;
            } else {
                prev_len = self.units.len();
            }
        }
    }

    fn remove_unit(&mut self, unit: char) {
        self.units.retain(|&c| c != unit && c != unit.to_ascii_uppercase());
    }

    fn improve(&mut self) {
        let best = "abcdefghijklmnopqrstuvwxyz".chars()
                                            .map(|c| {
                                                    let mut p = self.clone();
                                                    p.remove_unit(c);
                                                    p.react_all();
                                                    p
                                                })
                                            .min_by_key(|p| p.length())
                                            .unwrap();
        self.units = best.units;
    }
}

impl fmt::Display for Polymer {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for &unit in &self.units {
            write!(f, "{}", unit)?;
        }
        Ok(())
    }
}


#[aoc(day5, part2)]
pub fn solve(input: &str) -> u32 {
    let mut polymer = Polymer::from_string(input);
    polymer.improve();
    println!("Shortest length: {}", polymer.length());
    assert_eq!(polymer.length(), 4572);
    polymer.length()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_react_all() {
        let input = "aA";
        let mut polymer = Polymer::from_string(input);
        polymer.react_all();
        assert_eq!(polymer.to_string(), "");

        let input = "abBA";
        let mut polymer = Polymer::from_string(input);
        polymer.react_all();
        assert_eq!(polymer.to_string(), "");

        let input = "abAB";
        let mut polymer = Polymer::from_string(input);
        polymer.react_all();
        assert_eq!(polymer.to_string(), "abAB");

        let input = "aabAAB";
        let mut polymer = Polymer::from_string(input);
        polymer.react_all();
        assert_eq!(polymer.to_string(), "aabAAB");

        let input = "dabAcCaCBAcCcaDA";
        let mut polymer = Polymer::from_string(input);
        polymer.react_all();
        assert_eq!(polymer.to_string(), "dabCBAcaDA");
    }

    #[test]
    fn test_remove_unit() {
        let input = "dabAcCaCBAcCcaDA";
        let mut polymer = Polymer::from_string(input);
        polymer.remove_unit('a');
        assert_eq!(polymer.to_string(), "dbcCCBcCcD");

        let mut polymer = Polymer::from_string(input);
        polymer.remove_unit('b');
        assert_eq!(polymer.to_string(), "daAcCaCAcCcaDA");

        let mut polymer = Polymer::from_string(input);
        polymer.remove_unit('c');
        assert_eq!(polymer.to_string(), "dabAaBAaDA");

        let mut polymer = Polymer::from_string(input);
        polymer.remove_unit('d');
        assert_eq!(polymer.to_string(), "abAcCaCBAcCcaA");
    }

    #[test]
    fn test_improve() {
        let input = "dabAcCaCBAcCcaDA";
        let mut polymer = Polymer::from_string(input);
        polymer.improve();
        assert_eq!(polymer.to_string(), "daDA");
        assert_eq!(polymer.length(), 4);
    }
}
