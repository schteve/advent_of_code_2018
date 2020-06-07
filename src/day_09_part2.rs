/*
    --- Part Two ---
    Amused by the speed of your answer, the Elves are curious:

    What would the new winning Elf's score be if the number of the last marble were 100 times larger?
*/

use regex::Regex;

struct LLNode {
    prev: usize,
    next: usize,
    value: u32,
}

struct LinkedListCirc {
    data: Vec<LLNode>,
    head: Option<usize>,
    free_list: Option<usize>,
    current_idx: Option<usize>,
}

impl LinkedListCirc {
    fn new() -> Self {
        Self {
            data: Vec::new(),
            head: None,
            free_list: None,
            current_idx: None,
        }
    }

    fn allocate_node(&mut self) -> usize {
        // If there are any free indexes, use those. Otherwise create a new index.
        if let Some(free_idx) = self.free_list {
            if self.data[free_idx].next == free_idx {
                // There is only one item in the list
                self.free_list = None;
            } else {
                let before_idx = self.data[free_idx].prev;
                let after_idx = self.data[free_idx].next;
                self.data[before_idx].next = after_idx;
                self.data[after_idx].prev = before_idx;

                // Free list should now start with the next item
                self.free_list = Some(self.data[free_idx].next);
            }
            free_idx
        } else {
            let new_node_idx = self.data.len();
            let new_node = LLNode { // Dummy values
                prev: 0,
                next: 0,
                value: 0,
            };
            self.data.push(new_node);
            new_node_idx
        }
    }

    fn free_node(&mut self, index: usize) {
        // If there is an existing free list, append to it. Otherwise create a new one.
        if let Some(free_idx) = self.free_list {
            let before_idx = self.data[free_idx].prev;
            let after_idx = free_idx;
            self.data[before_idx].next = index;
            self.data[after_idx].prev = index;
            self.data[index].prev = before_idx;
            self.data[index].next = after_idx;
        } else {
            self.free_list = Some(index);
            self.data[index].prev = index;
            self.data[index].next = index;
        }
    }

    fn insert(&mut self, value: u32, offset: i32) {
        // Special case - list is empty
        if self.head == None {
            let new_node_idx = self.allocate_node();

            self.data[new_node_idx].prev = new_node_idx;
            self.data[new_node_idx].next = new_node_idx;
            self.data[new_node_idx].value = value;

            self.head = Some(new_node_idx);
            self.current_idx = Some(new_node_idx);
            return;
        }

        // Move to the target node's index - here we use the "after" node as the target
        let mut target_idx = self.current_idx.unwrap();
        if offset > 0 {
            for _ in 0..offset {
                target_idx = self.data[target_idx].next;
            }
        } else if offset < 0 {
            for _ in offset..0 {
                target_idx = self.data[target_idx].prev;
            }
        }

        // Create the new node, then fixup the previous and next nodes
        let before_idx = self.data[target_idx].prev;
        let after_idx = target_idx;
        let new_node_idx = self.allocate_node();

        self.data[new_node_idx].prev = before_idx;
        self.data[new_node_idx].next = after_idx;
        self.data[new_node_idx].value = value;

        self.data[before_idx].next = new_node_idx;
        self.data[after_idx].prev = new_node_idx;

        self.current_idx = Some(new_node_idx);
    }

    fn remove(&mut self, offset: i32) -> u32 {
        if self.head == None {
            // Special case - list is empty
            panic!("Tried to remove from empty list");
        }

        // Move to the target node's index
        let mut target_idx = self.current_idx.unwrap();
        if offset > 0 {
            for _ in 0..offset {
                target_idx = self.data[target_idx].next;
            }
        } else if offset < 0 {
            for _ in offset..0 {
                target_idx = self.data[target_idx].prev;
            }
        }

        // Adjust head (if needed) before modifying the list
        let head_idx = self.head.unwrap();
        if head_idx == target_idx {
            // Only need to adjust head if we are removing the head item
            if self.data[head_idx].next == head_idx {
                // List only had one item
                self.head = None;
            } else {
                // List had multiple items
                self.head = Some(self.data[head_idx].next);
            }
        }

        // Free the node, then fixup the previous and next nodes
        let before_idx = self.data[target_idx].prev;
        let after_idx = self.data[target_idx].next;

        let existing_value = self.data[target_idx].value;
        self.free_node(target_idx);

        self.data[before_idx].next = after_idx;
        self.data[after_idx].prev = before_idx;

        self.current_idx = Some(after_idx);

        existing_value
    }

    fn to_vec(&self) -> Vec<u32> {
        let mut result: Vec<u32> = Vec::new();

        if let Some(head_idx) = self.head {
            let mut traverse_idx = head_idx;
            loop {
                result.push(self.data[traverse_idx].value);
                traverse_idx = self.data[traverse_idx].next;

                if traverse_idx == head_idx {
                    break;
                }
            }
        }

        result
    }
}

struct Game {
    max_player: u32,
    max_marble: u32,
    state: LinkedListCirc,
    current_player: u32,
    player_score: Vec<u32>
}

impl Game {
    fn from_string(input: &str) -> Self {
        let re = Regex::new(r"(\d+) players; last marble is worth (\d+) points").unwrap();
        let caps = re.captures(input).unwrap();

        let max_player = caps[1].parse::<u32>().unwrap();
        let max_marble = caps[2].parse::<u32>().unwrap();

        let mut state = LinkedListCirc::new();
        state.insert(0, 0);

        Self {
            max_player: max_player,
            max_marble: max_marble,
            state: state,
            current_player: 0,
            player_score: vec![0; max_player as usize],
        }
    }

    fn place_marble(&mut self, value: u32, offset: i32) {
        self.state.insert(value, offset);
    }

    fn remove_marble(&mut self, offset: i32) -> u32 {
        self.state.remove(offset)
    }

    fn play(&mut self) -> u32 {
        // Assumes player scores are initialized to 0
        for marble_number in 1..=self.max_marble { // Marble 0 is placed at initialization
            if marble_number % 23 == 0 {
                // Don't place this marble, remove the one 7 to the left, and add both to the current player's score.
                let removed = self.state.remove(-7);
                self.player_score[self.current_player as usize] += marble_number + removed;
            } else {
                // Place this marble 2 to the right
                self.state.insert(marble_number, 2);
            }

            self.current_player = self.current_player + 1;
            if self.current_player >= self.max_player {
                self.current_player = 0;
            }
        }

        *self.player_score.iter().max().unwrap()
    }
}

#[aoc(day9, part2)]
pub fn solve(input: &str) -> u32 {
    let mut game = Game::from_string(input);
    game.max_marble *= 100;
    let high_score = game.play();
    println!("High score: {}" , high_score);
    assert_eq!(high_score, 3009951158);
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
        assert_eq!(game.state.to_vec(), vec![0, 1]);
        game.place_marble(2, 2);
        assert_eq!(game.state.to_vec(), vec![0, 2, 1]);
        game.place_marble(3, 2);
        assert_eq!(game.state.to_vec(), vec![0, 2, 1, 3]);
        game.place_marble(4, 2);
        assert_eq!(game.state.to_vec(), vec![0, 4, 2, 1, 3]);
        game.place_marble(5, 2);
        assert_eq!(game.state.to_vec(), vec![0, 4, 2, 5, 1, 3]);
        game.place_marble(6, 2);
        assert_eq!(game.state.to_vec(), vec![0, 4, 2, 5, 1, 6, 3]);
        game.place_marble(7, 2);
        assert_eq!(game.state.to_vec(), vec![0, 4, 2, 5, 1, 6, 3, 7]);
        game.place_marble(8, 2);
        assert_eq!(game.state.to_vec(), vec![0, 8, 4, 2, 5, 1, 6, 3, 7]);
        game.place_marble(9, 2);
        assert_eq!(game.state.to_vec(), vec![0, 8, 4, 9, 2, 5, 1, 6, 3, 7]);
    }

    #[test]
    fn test_play() {
        let input = "9 players; last marble is worth 25 points";
        let mut game = Game::from_string(input);
        let high_score = game.play();
        assert_eq!(high_score, 32);
        assert_eq!(game.state.to_vec(), vec![0, 16, 8, 17, 4, 18, 19, 2, 24, 20, 25, 10, 21, 5, 22, 11, 1, 12, 6, 13, 3, 14, 7, 15]);

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
