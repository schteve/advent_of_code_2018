
struct LLNode {
    prev: usize,
    next: usize,
    value: u32,
}

pub struct LLIter<'a> {
    list: &'a LinkedListCirc,
    index: Option<usize>,
}

impl<'a> LLIter<'a> {
    fn new(list: &'a LinkedListCirc) -> Self {
        Self {
            list: list,
            index: list.head,
        }
    }
}

impl<'a> Iterator for LLIter<'a> {
    type Item = u32;

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(head_idx) = self.list.head {
            if let Some(idx) = self.index {
                // Traverse to the next node, or stop if the head has been reached
                if self.list.data[idx].next == head_idx {
                    self.index = None;
                } else {
                    self.index = Some(self.list.data[idx].next);
                }

                Some(self.list.data[idx].value)
            } else {
                // List has been completely traversed
                None
            }
        } else {
            // List is empty
            None
        }
    }
}

pub struct LinkedListCirc {
    data: Vec<LLNode>,
    head: Option<usize>,
    free_list: Option<usize>,
    current_idx: Option<usize>,
}

impl LinkedListCirc {
    pub fn new() -> Self {
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
                // There is only one node in the list
                self.free_list = None;
            } else {
                let before_idx = self.data[free_idx].prev;
                let after_idx = self.data[free_idx].next;
                self.data[before_idx].next = after_idx;
                self.data[after_idx].prev = before_idx;

                // Free list should now start with the next node
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

    pub fn insert(&mut self, value: u32, offset: i32) {
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

    pub fn remove(&mut self, offset: i32) -> u32 {
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
            // Only need to adjust head if we are removing the head node
            if self.data[head_idx].next == head_idx {
                // List only had one node
                self.head = None;
            } else {
                // List had multiple nodes
                self.head = Some(self.data[head_idx].next);
            }
        }

        // Free the node, then fixup the previous and next nodes
        let before_idx = self.data[target_idx].prev;
        let after_idx = self.data[target_idx].next;

        let existing_value = self.data[target_idx].value;
        self.free_node(target_idx);

        // If we are removing the last node,
        if self.head == None {
            self.current_idx = None;
        } else {
            self.data[before_idx].next = after_idx;
            self.data[after_idx].prev = before_idx;

            self.current_idx = Some(after_idx);
        }

        existing_value
    }

    pub fn to_vec(&self) -> Vec<u32> {
        self.iter().collect()
    }

    pub fn iter(&self) -> LLIter {
        LLIter::new(self)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_nodes() {
        let mut list = LinkedListCirc::new();
        let node_idx = list.allocate_node();
        assert_eq!(node_idx, 0);
        assert_eq!(list.free_list, None);

        let node_idx = list.allocate_node();
        assert_eq!(node_idx, 1);

        let node_idx = list.allocate_node();
        assert_eq!(node_idx, 2);

        let node_idx = list.allocate_node();
        assert_eq!(node_idx, 3);

        list.free_node(0);
        assert_eq!(list.free_list, Some(0));

        list.free_node(2);
        assert_eq!(list.free_list, Some(0));
        assert_eq!(list.data[0].next, 2);
        assert_eq!(list.data[2].next, 0);
    }

    #[test]
    fn test_insert_remove() {
        let mut list = LinkedListCirc::new();
        list.insert(0, 0);
        assert_eq!(list.head, Some(0));
        assert_eq!(list.current_idx, Some(0));

        list.remove(0);
        assert_eq!(list.head, None);
        assert_eq!(list.current_idx, None);

        for i in 0..10 {
            list.insert(i, 1);
        }
        assert_eq!(list.to_vec(), vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9]);

        let mut list = LinkedListCirc::new();
        list.insert(0, 0);
        list.insert(1, 0);
        assert_eq!(list.to_vec(), vec![0, 1]);
        assert_eq!(list.head, Some(0));
        assert_eq!(list.current_idx, Some(1));

        list.insert(2, 0);
        assert_eq!(list.to_vec(), vec![0, 2, 1]);
        assert_eq!(list.current_idx, Some(2));

        list.insert(3, -1);
        assert_eq!(list.to_vec(), vec![0, 2, 1, 3]);
        assert_eq!(list.current_idx, Some(3));

        list.insert(4, -10);
        assert_eq!(list.to_vec(), vec![0, 4, 2, 1, 3]);
        assert_eq!(list.current_idx, Some(4));

        let item = list.remove(1);
        assert_eq!(item, 2);
        assert_eq!(list.to_vec(), vec![0, 4, 1, 3]);
        assert_eq!(list.head, Some(0));
        assert_eq!(list.current_idx, Some(1));

        let item = list.remove(-2);
        assert_eq!(item, 0);
        assert_eq!(list.to_vec(), vec![4, 1, 3]);
        assert_eq!(list.head, Some(4));
        assert_eq!(list.current_idx, Some(4));

        let item = list.remove(1);
        assert_eq!(item, 1);
        assert_eq!(list.to_vec(), vec![4, 3]);
        assert_eq!(list.head, Some(4));
        assert_eq!(list.current_idx, Some(3));

        list.insert(5, 1);
        assert_eq!(list.to_vec(), vec![4, 3, 5]);
        assert_eq!(list.head, Some(4));
        assert_eq!(list.current_idx, Some(2)); // Node indexes should be reallocated in the order they were removed

        let item = list.remove(0);
        assert_eq!(item, 5);
        assert_eq!(list.to_vec(), vec![4, 3]);
        assert_eq!(list.head, Some(4));
        assert_eq!(list.current_idx, Some(4));

        let item = list.remove(7);
        assert_eq!(item, 3);
        assert_eq!(list.to_vec(), vec![4]);
        assert_eq!(list.head, Some(4));
        assert_eq!(list.current_idx, Some(4));

        let item = list.remove(0);
        assert_eq!(item, 4);
        assert_eq!(list.to_vec(), vec![]);
        assert_eq!(list.head, None);
        assert_eq!(list.current_idx, None);
    }
}
