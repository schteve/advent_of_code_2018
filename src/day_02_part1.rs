/*
    --- Day 2: Inventory Management System ---
    You stop falling through time, catch your breath, and check the screen on the device. "Destination reached. Current Year: 1518. Current Location: North Pole Utility Closet 83N10." You made it! Now, to find those anomalies.

    Outside the utility closet, you hear footsteps and a voice. "...I'm not sure either. But now that so many people have chimneys, maybe he could sneak in that way?" Another voice responds, "Actually, we've been working on a new kind of suit that would let him fit through tight spaces like that. But, I heard that a few days ago, they lost the prototype fabric, the design plans, everything! Nobody on the team can even seem to remember important details of the project!"

    "Wouldn't they have had enough fabric to fill several boxes in the warehouse? They'd be stored together, so the box IDs should be similar. Too bad it would take forever to search the warehouse for two similar box IDs..." They walk too far away to hear any more.

    Late at night, you sneak to the warehouse - who knows what kinds of paradoxes you could cause if you were discovered - and use your fancy wrist device to quickly scan every box and produce a list of the likely candidates (your puzzle input).

    To make sure you didn't miss any, you scan the likely candidate boxes again, counting the number that have an ID containing exactly two of any letter and then separately counting those with exactly three of any letter. You can multiply those two counts together to get a rudimentary checksum and compare it to what your device predicts.

    For example, if you see the following box IDs:

    abcdef contains no letters that appear exactly two or three times.
    bababc contains two a and three b, so it counts for both.
    abbcde contains two b, but no letter appears exactly three times.
    abcccd contains three c, but no letter appears exactly two times.
    aabcdd contains two a and two d, but it only counts once.
    abcdee contains two e.
    ababab contains three a and three b, but it only counts once.
    Of these box IDs, four of them contain a letter which appears exactly twice, and three of them contain a letter which appears exactly three times. Multiplying these together produces a checksum of 4 * 3 = 12.

    What is the checksum for your list of box IDs?
*/

use std::collections::HashMap;

fn count_duplicates(box_id: &str) -> (bool, bool) {
    let mut counts: HashMap<char, u32> = HashMap::new();
    for c in box_id.chars() {
        let count = counts.entry(c).or_insert(0);
        *count += 1;
    }

    let has_2 = counts.values().any(|&v| v == 2);
    let has_3 = counts.values().any(|&v| v == 3);
    (has_2, has_3)
}

fn get_all_duplicates(box_ids: &[&str]) -> Vec<(bool, bool)> {
    box_ids
        .iter()
        .map(|box_id| count_duplicates(box_id))
        .collect()
}

fn checksum(duplicates: &[(bool, bool)]) -> u32 {
    let (twos, threes): (Vec<bool>, Vec<bool>) = duplicates.iter().cloned().unzip();
    let twos_count = twos.iter().filter(|&&b| b == true).count() as u32;
    let threes_count = threes.iter().filter(|&&b| b == true).count() as u32;
    twos_count * threes_count
}

#[aoc(day2, part1)]
pub fn solve(input: &str) -> u32 {
    let box_ids: Vec<&str> = input.lines().map(|line| line.trim()).collect();
    let duplicates = get_all_duplicates(&box_ids);
    let chk = checksum(&duplicates);
    println!("Checksum: {}", chk);
    assert_eq!(chk, 5952);
    chk
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_count_duplicates() {
        let duplicates = count_duplicates("abcdef");
        assert_eq!(duplicates, (false, false));

        let duplicates = count_duplicates("bababc");
        assert_eq!(duplicates, (true, true));

        let duplicates = count_duplicates("abbcde");
        assert_eq!(duplicates, (true, false));

        let duplicates = count_duplicates("abcccd");
        assert_eq!(duplicates, (false, true));

        let duplicates = count_duplicates("aabcdd");
        assert_eq!(duplicates, (true, false));

        let duplicates = count_duplicates("abcdee");
        assert_eq!(duplicates, (true, false));

        let duplicates = count_duplicates("ababab");
        assert_eq!(duplicates, (false, true));
    }

    #[test]
    fn test_checksum() {
        let box_ids = vec![
            "abcdef", "bababc", "abbcde", "abcccd", "aabcdd", "abcdee", "ababab",
        ];
        let duplicates = get_all_duplicates(&box_ids);
        let chk = checksum(&duplicates);
        assert_eq!(chk, 12);
    }
}
