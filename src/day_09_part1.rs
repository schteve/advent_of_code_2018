/*
    --- Day 9: Marble Mania ---
    You talk to the Elves while you wait for your navigation system to initialize. To pass the time, they introduce you to their favorite marble game.

    The Elves play this game by taking turns arranging the marbles in a circle according to very particular rules. The marbles are numbered starting with 0 and increasing by 1 until every marble has a number.

    First, the marble numbered 0 is placed in the circle. At this point, while it contains only a single marble, it is still a circle: the marble is both clockwise from itself and counter-clockwise from itself. This marble is designated the current marble.

    Then, each Elf takes a turn placing the lowest-numbered remaining marble into the circle between the marbles that are 1 and 2 marbles clockwise of the current marble. (When the circle is large enough, this means that there is one marble between the marble that was just placed and the current marble.) The marble that was just placed then becomes the current marble.

    However, if the marble that is about to be placed has a number which is a multiple of 23, something entirely different happens. First, the current player keeps the marble they would have placed, adding it to their score. In addition, the marble 7 marbles counter-clockwise from the current marble is removed from the circle and also added to the current player's score. The marble located immediately clockwise of the marble that was removed becomes the new current marble.

    For example, suppose there are 9 players. After the marble with value 0 is placed in the middle, each player (shown in square brackets) takes a turn. The result of each of those turns would produce circles of marbles like this, where clockwise is to the right and the resulting current marble is in parentheses:

    [-] (0)
    [1]  0 (1)
    [2]  0 (2) 1
    [3]  0  2  1 (3)
    [4]  0 (4) 2  1  3
    [5]  0  4  2 (5) 1  3
    [6]  0  4  2  5  1 (6) 3
    [7]  0  4  2  5  1  6  3 (7)
    [8]  0 (8) 4  2  5  1  6  3  7
    [9]  0  8  4 (9) 2  5  1  6  3  7
    [1]  0  8  4  9  2(10) 5  1  6  3  7
    [2]  0  8  4  9  2 10  5(11) 1  6  3  7
    [3]  0  8  4  9  2 10  5 11  1(12) 6  3  7
    [4]  0  8  4  9  2 10  5 11  1 12  6(13) 3  7
    [5]  0  8  4  9  2 10  5 11  1 12  6 13  3(14) 7
    [6]  0  8  4  9  2 10  5 11  1 12  6 13  3 14  7(15)
    [7]  0(16) 8  4  9  2 10  5 11  1 12  6 13  3 14  7 15
    [8]  0 16  8(17) 4  9  2 10  5 11  1 12  6 13  3 14  7 15
    [9]  0 16  8 17  4(18) 9  2 10  5 11  1 12  6 13  3 14  7 15
    [1]  0 16  8 17  4 18  9(19) 2 10  5 11  1 12  6 13  3 14  7 15
    [2]  0 16  8 17  4 18  9 19  2(20)10  5 11  1 12  6 13  3 14  7 15
    [3]  0 16  8 17  4 18  9 19  2 20 10(21) 5 11  1 12  6 13  3 14  7 15
    [4]  0 16  8 17  4 18  9 19  2 20 10 21  5(22)11  1 12  6 13  3 14  7 15
    [5]  0 16  8 17  4 18(19) 2 20 10 21  5 22 11  1 12  6 13  3 14  7 15
    [6]  0 16  8 17  4 18 19  2(24)20 10 21  5 22 11  1 12  6 13  3 14  7 15
    [7]  0 16  8 17  4 18 19  2 24 20(25)10 21  5 22 11  1 12  6 13  3 14  7 15
    The goal is to be the player with the highest score after the last marble is used up. Assuming the example above ends after the marble numbered 25, the winning score is 23+9=32 (because player 5 kept marble 23 and removed marble 9, while no other player got any points in this very short example game).

    Here are a few more examples:

    10 players; last marble is worth 1618 points: high score is 8317
    13 players; last marble is worth 7999 points: high score is 146373
    17 players; last marble is worth 1104 points: high score is 2764
    21 players; last marble is worth 6111 points: high score is 54718
    30 players; last marble is worth 5807 points: high score is 37305
    What is the winning Elf's score?
*/

use super::common::modulo;
use regex::Regex;

struct Game {
    max_player: u32,
    max_marble: u32,
    state: Vec<u32>,
    current_player: u32,
    current_marble: u32,
    player_score: Vec<u32>,
}

impl Game {
    fn from_string(input: &str) -> Self {
        let re = Regex::new(r"(\d+) players; last marble is worth (\d+) points").unwrap();
        let caps = re.captures(input).unwrap();

        let max_player = caps[1].parse::<u32>().unwrap();
        let max_marble = caps[2].parse::<u32>().unwrap();

        let mut state = Vec::new();
        state.push(0);

        Self {
            max_player,
            max_marble,
            state,
            current_player: 0,
            current_marble: 0,
            player_score: vec![0; max_player as usize],
        }
    }

    fn place_marble(&mut self, value: u32, offset: i32) {
        //let index = modulo(self.current_marble as i32 + (offset - 1), self.state.len() as i32) + 1;
        let index = modulo(
            self.current_marble as i32 + (offset - 1),
            self.state.len() as i32,
        ) + 1;
        self.state.insert(index as usize, value);
        self.current_marble = index as u32;
    }

    fn remove_marble(&mut self, offset: i32) -> u32 {
        let index = modulo(self.current_marble as i32 + offset, self.state.len() as i32);
        let value = self.state.remove(index as usize);
        self.current_marble = index as u32;
        value
    }

    fn play(&mut self) -> u32 {
        // Assumes player scores are initialized to 0
        for marble_number in 1..=self.max_marble {
            // Marble 0 is placed at initialization
            if marble_number % 23 == 0 {
                // Don't place this marble, remove the one 7 to the left, and add both to the current player's score.
                let removed = self.remove_marble(-7);
                self.player_score[self.current_player as usize] += marble_number + removed;
            } else {
                // Place this marble 2 to the right
                self.place_marble(marble_number, 2);
            }

            self.current_player =
                modulo(self.current_player as i32 + 1, self.max_player as i32) as u32;
        }

        *self.player_score.iter().max().unwrap()
    }
}

#[aoc(day9, part1)]
pub fn solve(input: &str) -> u32 {
    let mut game = Game::from_string(input);
    let high_score = game.play();
    println!("High score: {}", high_score);
    assert_eq!(high_score, 374690);
    high_score
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_place_marble() {
        let input = "9 players; last marble is worth 25 points";
        let mut game = Game::from_string(input);
        game.place_marble(1, 2);
        assert_eq!(game.state, vec![0, 1]);
        game.place_marble(2, 2);
        assert_eq!(game.state, vec![0, 2, 1]);
        game.place_marble(3, 2);
        assert_eq!(game.state, vec![0, 2, 1, 3]);
        game.place_marble(4, 2);
        assert_eq!(game.state, vec![0, 4, 2, 1, 3]);
        game.place_marble(5, 2);
        assert_eq!(game.state, vec![0, 4, 2, 5, 1, 3]);
        game.place_marble(6, 2);
        assert_eq!(game.state, vec![0, 4, 2, 5, 1, 6, 3]);
        game.place_marble(7, 2);
        assert_eq!(game.state, vec![0, 4, 2, 5, 1, 6, 3, 7]);
        game.place_marble(8, 2);
        assert_eq!(game.state, vec![0, 8, 4, 2, 5, 1, 6, 3, 7]);
        game.place_marble(9, 2);
        assert_eq!(game.state, vec![0, 8, 4, 9, 2, 5, 1, 6, 3, 7]);
    }

    #[test]
    fn test_play() {
        let input = "9 players; last marble is worth 25 points";
        let mut game = Game::from_string(input);
        let high_score = game.play();
        assert_eq!(high_score, 32);
        assert_eq!(
            game.state,
            vec![
                0, 16, 8, 17, 4, 18, 19, 2, 24, 20, 25, 10, 21, 5, 22, 11, 1, 12, 6, 13, 3, 14, 7,
                15
            ]
        );

        let input = "10 players; last marble is worth 1618 points";
        let mut game = Game::from_string(input);
        let high_score = game.play();
        assert_eq!(high_score, 8317);

        let input = "13 players; last marble is worth 7999 points";
        let mut game = Game::from_string(input);
        let high_score = game.play();
        assert_eq!(high_score, 146373);

        let input = "17 players; last marble is worth 1104 points";
        let mut game = Game::from_string(input);
        let high_score = game.play();
        assert_eq!(high_score, 2764);

        let input = "21 players; last marble is worth 6111 points";
        let mut game = Game::from_string(input);
        let high_score = game.play();
        assert_eq!(high_score, 54718);

        let input = "30 players; last marble is worth 5807 points";
        let mut game = Game::from_string(input);
        let high_score = game.play();
        assert_eq!(high_score, 37305);
    }
}
