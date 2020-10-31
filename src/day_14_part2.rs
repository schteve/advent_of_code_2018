/*
    --- Part Two ---
    As it turns out, you got the Elves' plan backwards. They actually want to know how many recipes appear on the scoreboard to the left of the first recipes whose scores are the digits from your puzzle input.

    51589 first appears after 9 recipes.
    01245 first appears after 5 recipes.
    92510 first appears after 18 recipes.
    59414 first appears after 2018 recipes.
    How many recipes appear on the scoreboard to the left of the score sequence in your puzzle input?
*/

use crate::common::modulo;

fn recipes_before_target(target: &str) -> usize {
    let mut recipes: Vec<u8> = vec![3, 7];
    let mut elf1: usize = 0;
    let mut elf2: usize = 1;

    let target_vec: Vec<u8> = target
        .chars()
        .map(|c| c.to_digit(10).expect("Non-digit character") as u8)
        .collect();
    let mut target_idx = 0;

    loop {
        let new_recipe = recipes[elf1] + recipes[elf2];

        let ones_digit = new_recipe % 10;
        let digits = if new_recipe >= 10 {
            vec![1, ones_digit]
        } else {
            vec![ones_digit]
        };

        for d in digits {
            recipes.push(d);

            if target_vec[target_idx] == d {
                target_idx += 1;
                if target_idx == target_vec.len() {
                    return recipes.len() - target_vec.len();
                }
            } else {
                target_idx = 0;
                if target_vec[target_idx] == d {
                    target_idx += 1;
                }
            }
        }

        elf1 = modulo(elf1 + 1 + recipes[elf1] as usize, recipes.len());
        elf2 = modulo(elf2 + 1 + recipes[elf2] as usize, recipes.len());
    }
}

#[aoc(day14, part2)]
pub fn solve(input: &str) -> usize {
    let target_number = input.trim();
    let num_recipes = recipes_before_target(target_number);
    println!("Recipes: {}", num_recipes);
    assert_eq!(num_recipes, 20322683);
    num_recipes
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_recipes_before_target() {
        assert_eq!(recipes_before_target("51589"), 9);
        assert_eq!(recipes_before_target("01245"), 5);
        assert_eq!(recipes_before_target("92510"), 18);
        assert_eq!(recipes_before_target("59414"), 2018);
    }
}
