/*
    --- Day 8: Memory Maneuver ---
    The sleigh is much easier to pull than you'd expect for something its weight. Unfortunately, neither you nor the Elves know which way the North Pole is from here.

    You check your wrist device for anything that might help. It seems to have some kind of navigation system! Activating the navigation system produces more bad news: "Failed to start navigation system. Could not read software license file."

    The navigation system's license file consists of a list of numbers (your puzzle input). The numbers define a data structure which, when processed, produces some kind of tree that can be used to calculate the license number.

    The tree is made up of nodes; a single, outermost node forms the tree's root, and it contains all other nodes in the tree (or contains nodes that contain nodes, and so on).

    Specifically, a node consists of:

    A header, which is always exactly two numbers:
    The quantity of child nodes.
    The quantity of metadata entries.
    Zero or more child nodes (as specified in the header).
    One or more metadata entries (as specified in the header).
    Each child node is itself a node that has its own header, child nodes, and metadata. For example:

    2 3 0 3 10 11 12 1 1 0 1 99 2 1 1 2
    A----------------------------------
        B----------- C-----------
                        D-----
    In this example, each node of the tree is also marked with an underline starting with a letter for easier identification. In it, there are four nodes:

    A, which has 2 child nodes (B, C) and 3 metadata entries (1, 1, 2).
    B, which has 0 child nodes and 3 metadata entries (10, 11, 12).
    C, which has 1 child node (D) and 1 metadata entry (2).
    D, which has 0 child nodes and 1 metadata entry (99).
    The first check done on the license file is to simply add up all of the metadata entries. In this example, that sum is 1+1+2+10+11+12+2+99=138.

    What is the sum of all metadata entries?
*/

struct Node {
    num_children: usize,
    num_metadata: usize,
    children: Vec<Node>,
    metadata: Vec<u32>,
}

impl Node {
    fn from_string(input: &str) -> Self {
        let input_vec: Vec<u32> = input
            .split(' ')
            .map(|s| s.trim().parse::<u32>().unwrap())
            .collect();
        let (node, consumed) = Node::from_slice(&input_vec);
        assert_eq!(consumed, input_vec.len()); // Check that we used every byte. Not strictly necessary but should be valid for this puzzle.
        node
    }

    fn from_slice(input: &[u32]) -> (Self, usize) {
        let mut idx = 0;
        let num_children = input[idx] as usize;
        idx += 1;
        let num_metadata = input[idx] as usize;
        idx += 1;

        let children: Vec<Node> = (0..num_children)
            .map(|_| {
                let (node, consumed) = Node::from_slice(&input[idx..]);
                idx += consumed;
                node
            })
            .collect();

        let metadata: Vec<u32> = input[idx..(idx + num_metadata)].to_vec();
        idx += num_metadata;

        let node = Self {
            num_children,
            num_metadata,
            children,
            metadata,
        };
        (node, idx)
    }

    fn sum_metadata(&self) -> u32 {
        let my_sum: u32 = self.metadata.iter().sum();
        let child_sum: u32 = self.children.iter().map(|child| child.sum_metadata()).sum();
        my_sum + child_sum
    }
}

#[aoc(day8, part1)]
pub fn solve(input: &str) -> u32 {
    let tree = Node::from_string(input);
    let meta_sum = tree.sum_metadata();
    println!("Sum of metadata: {}", meta_sum);
    assert_eq!(meta_sum, 44838);
    meta_sum
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_sum_metadata() {
        let input = "2 3 0 3 10 11 12 1 1 0 1 99 2 1 1 2";
        let tree = Node::from_string(input);
        let meta_sum = tree.sum_metadata();
        assert_eq!(meta_sum, 138);
    }
}
