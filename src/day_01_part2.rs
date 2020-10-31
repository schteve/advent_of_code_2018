/*
    --- Part Two ---
    You notice that the device repeats the same frequency change list over and over. To calibrate the device, you need to find the first frequency it reaches twice.

    For example, using the same list of changes above, the device would loop as follows:

    Current frequency  0, change of +1; resulting frequency  1.
    Current frequency  1, change of -2; resulting frequency -1.
    Current frequency -1, change of +3; resulting frequency  2.
    Current frequency  2, change of +1; resulting frequency  3.
    (At this point, the device continues from the start of the list.)
    Current frequency  3, change of +1; resulting frequency  4.
    Current frequency  4, change of -2; resulting frequency  2, which has already been seen.
    In this example, the first frequency reached twice is 2. Note that your device might need to repeat its list of frequency changes many times before a duplicate frequency is found, and that duplicates might be found while in the middle of processing the list.

    Here are other examples:

    +1, -1 first reaches 0 twice.
    +3, +3, +4, -2, -4 first reaches 10 twice.
    -6, +3, +8, +5, -6 first reaches 5 twice.
    +7, +7, -2, -7, -4 first reaches 14 twice.
    What is the first frequency your device reaches twice?
*/

use std::collections::HashSet;

fn find_first_repeat(frequencies: &[i32]) -> i32 {
    let mut result = 0;
    let mut result_set: HashSet<i32> = HashSet::new();
    result_set.insert(result);

    for f in frequencies.iter().cycle() {
        result += f;
        if result_set.insert(result) == false {
            // Value was already present -- this is a repeat
            return result;
        }
    }
    panic!("Impossible to reach");
}

#[aoc(day1, part2)]
pub fn solve(input: &str) -> i32 {
    let frequencies: Vec<i32> = input
        .lines()
        .map(|line| line.trim().parse::<i32>().unwrap())
        .collect();

    let result = find_first_repeat(&frequencies);
    println!("First repeat frequency: {}", result);
    assert_eq!(result, 413);
    result
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_find_first_repeat() {
        let frequencies = vec![1, -2, 3, 1];
        let result = find_first_repeat(&frequencies);
        assert_eq!(result, 2);

        let frequencies = vec![1, -1];
        let result = find_first_repeat(&frequencies);
        assert_eq!(result, 0);

        let frequencies = vec![3, 3, 4, -2, -4];
        let result = find_first_repeat(&frequencies);
        assert_eq!(result, 10);

        let frequencies = vec![-6, 3, 8, 5, -6];
        let result = find_first_repeat(&frequencies);
        assert_eq!(result, 5);

        let frequencies = vec![7, 7, -2, -7, -4];
        let result = find_first_repeat(&frequencies);
        assert_eq!(result, 14);
    }
}
