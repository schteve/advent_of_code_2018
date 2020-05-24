/*
    --- Part Two ---
    Confident that your list of box IDs is complete, you're ready to find the boxes full of prototype fabric.

    The boxes will have IDs which differ by exactly one character at the same position in both strings. For example, given the following box IDs:

    abcde
    fghij
    klmno
    pqrst
    fguij
    axcye
    wvxyz
    The IDs abcde and axcye are close, but they differ by two characters (the second and fourth). However, the IDs fghij and fguij differ by exactly one character, the third (h and u). Those must be the correct boxes.

    What letters are common between the two correct box IDs? (In the example above, this is found by removing the differing character from either ID, producing fgij.)
*/

fn calculate_difference(id1: &str, id2: &str) -> u32 {
    let count_same = id1.chars().zip(id2.chars()).filter(|(c1, c2)| c1 == c2).count();
    (id1.len() - count_same) as u32
}

fn find_correct_boxes(box_ids: &Vec<&str>) -> (usize, usize) {
    for i in 0..box_ids.len() {
        for j in 0..box_ids.len() {
            let difference = calculate_difference(box_ids[i], box_ids[j]);
            if difference == 1 {
                return (i, j);
            }
        }
    }
    panic!("Correct boxes not found");
}

fn common_letters(id1: &str, id2: &str) -> String {
    id1.chars().zip(id2.chars())
        .filter(|(c1, c2)| c1 == c2)
        .map(|(c1, _c2)| c1)
        .collect()
}

#[aoc(day2, part2)]
pub fn solve(input: &str) -> String {
    let box_ids: Vec<&str> = input.lines()
                                .map(|line| line.trim())
                                .collect();

    let correct_idxs = find_correct_boxes(&box_ids);
    let common_letters = common_letters(box_ids[correct_idxs.0], box_ids[correct_idxs.1]);
    println!("Common letters: {}", common_letters);
    assert_eq!(common_letters, "krdmtuqjgwfoevnaboxglzjph");
    common_letters
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_calculate_difference() {
        let difference = calculate_difference("abcde", "axcye");
        assert_eq!(difference, 2);

        let difference = calculate_difference("fghij", "fguij");
        assert_eq!(difference, 1);
    }

    #[test]
    fn test_find_correct_boxes() {
        let box_ids = vec!["abcde",
                           "fghij",
                           "klmno",
                           "pqrst",
                           "fguij",
                           "axcye",
                           "wvxyz"];
        let correct_idxs = find_correct_boxes(&box_ids);
        assert_eq!(correct_idxs, (1, 4));
    }

    #[test]
    fn test_common_letters() {
        let common_letters = common_letters("fghij", "fguij");
        assert_eq!(common_letters, "fgij");
    }
}
