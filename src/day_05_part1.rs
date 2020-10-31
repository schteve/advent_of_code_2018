/*
    --- Day 5: Alchemical Reduction ---
    You've managed to sneak in to the prototype suit manufacturing lab. The Elves are making decent progress, but are still struggling with the suit's size reduction capabilities.

    While the very latest in 1518 alchemical technology might have solved their problem eventually, you can do better. You scan the chemical composition of the suit's material and discover that it is formed by extremely long polymers (one of which is available as your puzzle input).

    The polymer is formed by smaller units which, when triggered, react with each other such that two adjacent units of the same type and opposite polarity are destroyed. Units' types are represented by letters; units' polarity is represented by capitalization. For instance, r and R are units with the same type but opposite polarity, whereas r and s are entirely different types and do not react.

    For example:

    In aA, a and A react, leaving nothing behind.
    In abBA, bB destroys itself, leaving aA. As above, this then destroys itself, leaving nothing.
    In abAB, no two adjacent units are of the same type, and so nothing happens.
    In aabAAB, even though aa and AA are of the same type, their polarities match, and so nothing happens.
    Now, consider a larger example, dabAcCaCBAcCcaDA:

    dabAcCaCBAcCcaDA  The first 'cC' is removed.
    dabAaCBAcCcaDA    This creates 'Aa', which is removed.
    dabCBAcCcaDA      Either 'cC' or 'Cc' are removed (the result is the same).
    dabCBAcaDA        No further actions can be taken.
    After all possible reactions, the resulting polymer contains 10 units.

    How many units remain after fully reacting the polymer you scanned?
*/

use std::fmt;

fn check_destroy(a: char, b: char) -> bool {
    if (a == b.to_ascii_lowercase() && a.to_ascii_uppercase() == b)
        || (a == b.to_ascii_uppercase() && a.to_ascii_lowercase() == b)
    {
        true
    } else {
        false
    }
}

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
}

impl fmt::Display for Polymer {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for &unit in &self.units {
            write!(f, "{}", unit)?;
        }
        Ok(())
    }
}

#[aoc(day5, part1)]
pub fn solve(input: &str) -> u32 {
    let mut polymer = Polymer::from_string(input);
    polymer.react_all();
    println!("Units remaining: {}", polymer.length());
    assert_eq!(polymer.length(), 10132);
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
}
