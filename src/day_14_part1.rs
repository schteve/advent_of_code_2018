/*
    --- Day 14: Chocolate Charts ---
    You finally have a chance to look at all of the produce moving around. Chocolate, cinnamon, mint, chili peppers, nutmeg, vanilla... the Elves must be growing these plants to make hot chocolate! As you realize this, you hear a conversation in the distance. When you go to investigate, you discover two Elves in what appears to be a makeshift underground kitchen/laboratory.

    The Elves are trying to come up with the ultimate hot chocolate recipe; they're even maintaining a scoreboard which tracks the quality score (0-9) of each recipe.

    Only two recipes are on the board: the first recipe got a score of 3, the second, 7. Each of the two Elves has a current recipe: the first Elf starts with the first recipe, and the second Elf starts with the second recipe.

    To create new recipes, the two Elves combine their current recipes. This creates new recipes from the digits of the sum of the current recipes' scores. With the current recipes' scores of 3 and 7, their sum is 10, and so two new recipes would be created: the first with score 1 and the second with score 0. If the current recipes' scores were 2 and 3, the sum, 5, would only create one recipe (with a score of 5) with its single digit.

    The new recipes are added to the end of the scoreboard in the order they are created. So, after the first round, the scoreboard is 3, 7, 1, 0.

    After all new recipes are added to the scoreboard, each Elf picks a new current recipe. To do this, the Elf steps forward through the scoreboard a number of recipes equal to 1 plus the score of their current recipe. So, after the first round, the first Elf moves forward 1 + 3 = 4 times, while the second Elf moves forward 1 + 7 = 8 times. If they run out of recipes, they loop back around to the beginning. After the first round, both Elves happen to loop around until they land on the same recipe that they had in the beginning; in general, they will move to different recipes.

    Drawing the first Elf as parentheses and the second Elf as square brackets, they continue this process:

    (3)[7]
    (3)[7] 1  0
    3  7  1 [0](1) 0
    3  7  1  0 [1] 0 (1)
    (3) 7  1  0  1  0 [1] 2
    3  7  1  0 (1) 0  1  2 [4]
    3  7  1 [0] 1  0 (1) 2  4  5
    3  7  1  0 [1] 0  1  2 (4) 5  1
    3 (7) 1  0  1  0 [1] 2  4  5  1  5
    3  7  1  0  1  0  1  2 [4](5) 1  5  8
    3 (7) 1  0  1  0  1  2  4  5  1  5  8 [9]
    3  7  1  0  1  0  1 [2] 4 (5) 1  5  8  9  1  6
    3  7  1  0  1  0  1  2  4  5 [1] 5  8  9  1 (6) 7
    3  7  1  0 (1) 0  1  2  4  5  1  5 [8] 9  1  6  7  7
    3  7 [1] 0  1  0 (1) 2  4  5  1  5  8  9  1  6  7  7  9
    3  7  1  0 [1] 0  1  2 (4) 5  1  5  8  9  1  6  7  7  9  2
    The Elves think their skill will improve after making a few recipes (your puzzle input). However, that could take ages; you can speed this up considerably by identifying the scores of the ten recipes after that. For example:

    If the Elves think their skill will improve after making 9 recipes, the scores of the ten recipes after the first nine on the scoreboard would be 5158916779 (highlighted in the last line of the diagram).
    After 5 recipes, the scores of the next ten would be 0124515891.
    After 18 recipes, the scores of the next ten would be 9251071085.
    After 2018 recipes, the scores of the next ten would be 5941429882.
    What are the scores of the ten recipes immediately after the number of recipes in your puzzle input?
*/

use crate::common::modulo;

fn score_after_target(target: usize) -> u64 {
    let mut recipes: Vec<u8> = vec![3, 7];
    let mut elf1: usize = 0;
    let mut elf2: usize = 1;

    while recipes.len() < target + 10 {
        let new_recipe = recipes[elf1] + recipes[elf2];
        if new_recipe >= 10 {
            recipes.push(1);
        }
        recipes.push(new_recipe % 10);

        elf1 = modulo(elf1 + 1 + recipes[elf1] as usize, recipes.len());
        elf2 = modulo(elf2 + 1 + recipes[elf2] as usize, recipes.len());
    }

    let score_slice = &recipes[target..target + 10];
    score_slice.iter().fold(0, |acc, &s| acc * 10 + s as u64)
}

#[aoc(day14, part1)]
pub fn solve(input: &str) -> u64 {
    let target_number = input.trim().parse::<usize>().unwrap();
    let score = score_after_target(target_number);
    println!("Score: {}", score);
    assert_eq!(score, 1741551073);
    score
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_score_after_target() {
        assert_eq!(score_after_target(9), 5158916779);
        assert_eq!(score_after_target(5), 0124515891);
        assert_eq!(score_after_target(18), 9251071085);
        assert_eq!(score_after_target(2018), 5941429882);
    }
}
