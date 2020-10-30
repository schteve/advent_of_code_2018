/*
    --- Part Two ---
    The second check is slightly more complicated: you need to find the value of the root node (A in the example above).

    The value of a node depends on whether it has child nodes.

    If a node has no child nodes, its value is the sum of its metadata entries. So, the value of node B is 10+11+12=33, and the value of node D is 99.

    However, if a node does have child nodes, the metadata entries become indexes which refer to those child nodes. A metadata entry of 1 refers to the first child node, 2 to the second, 3 to the third, and so on. The value of this node is the sum of the values of the child nodes referenced by the metadata entries. If a referenced child node does not exist, that reference is skipped. A child node can be referenced multiple time and counts each time it is referenced. A metadata entry of 0 does not refer to any child node.

    For example, again using the above nodes:

    Node C has one metadata entry, 2. Because node C has only one child node, 2 references a child node which does not exist, and so the value of node C is 0.
    Node A has three metadata entries: 1, 1, and 2. The 1 references node A's first child node, B, and the 2 references node A's second child node, C. Because node B has a value of 33 and node C has a value of 0, the value of node A is 33+33+0=66.
    So, in this example, the value of the root node is 66.

    What is the value of the root node?
*/

struct Node {
    num_children: usize,
    num_metadata: usize,
    children: Vec<Node>,
    metadata: Vec<u32>,
}

impl Node {
    fn from_string(input: &str) -> Self {
        let input_vec: Vec<u32> = input.split(' ').map(|s| s.trim().parse::<u32>().unwrap()).collect();
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

        let metadata: Vec<u32> = input[idx .. (idx + num_metadata)].to_vec();
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

    fn value(&self) -> u32 {
        if self.children.is_empty() == true {
            return self.metadata.iter().sum();
        }

        let mut value = 0;
        for &meta in &self.metadata {
            let entry = (meta - 1) as usize; // Metadata entries are 1-based. 0 will panic in debug and correctly be skipped in release.
            if entry < self.children.len() {
                value += self.children[entry].value();
            }
        }
        value
    }
}

#[aoc(day8, part2)]
pub fn solve(input: &str) -> u32 {
    let tree = Node::from_string(input);
    let value = tree.value();
    println!("Root value: {}", value);
    assert_eq!(value, 22198);
    value
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

    #[test]
    fn test_value() {
        let input = "2 3 0 3 10 11 12 1 1 0 1 99 2 1 1 2";
        let tree = Node::from_string(input);
        let value = tree.value();
        assert_eq!(value, 66);
    }
}
