/*
    --- Day 7: The Sum of Its Parts ---
    You find yourself standing on a snow-covered coastline; apparently, you landed a little off course. The region is too hilly to see the North Pole from here, but you do spot some Elves that seem to be trying to unpack something that washed ashore. It's quite cold out, so you decide to risk creating a paradox by asking them for directions.

    "Oh, are you the search party?" Somehow, you can understand whatever Elves from the year 1018 speak; you assume it's Ancient Nordic Elvish. Could the device on your wrist also be a translator? "Those clothes don't look very warm; take this." They hand you a heavy coat.

    "We do need to find our way back to the North Pole, but we have higher priorities at the moment. You see, believe it or not, this box contains something that will solve all of Santa's transportation problems - at least, that's what it looks like from the pictures in the instructions." It doesn't seem like they can read whatever language it's in, but you can: "Sleigh kit. Some assembly required."

    "'Sleigh'? What a wonderful name! You must help us assemble this 'sleigh' at once!" They start excitedly pulling more parts out of the box.

    The instructions specify a series of steps and requirements about which steps must be finished before others can begin (your puzzle input). Each step is designated by a single letter. For example, suppose you have the following instructions:

    Step C must be finished before step A can begin.
    Step C must be finished before step F can begin.
    Step A must be finished before step B can begin.
    Step A must be finished before step D can begin.
    Step B must be finished before step E can begin.
    Step D must be finished before step E can begin.
    Step F must be finished before step E can begin.
    Visually, these requirements look like this:

    -->A--->B--
    /    \      \
    C      -->D----->E
    \           /
    ---->F-----
    Your first goal is to determine the order in which the steps should be completed. If more than one step is ready, choose the step which is first alphabetically. In this example, the steps would be completed as follows:

    Only C is available, and so it is done first.
    Next, both A and F are available. A is first alphabetically, so it is done next.
    Then, even though F was available earlier, steps B and D are now also available, and B is the first alphabetically of the three.
    After that, only D and F are available. E is not available because only some of its prerequisites are complete. Therefore, D is completed next.
    F is the only choice, so it is done next.
    Finally, E is completed.
    So, in this example, the correct order is CABDFE.

    In what order should the steps in your instructions be completed?
*/

use regex::Regex;
use std::collections::HashMap;

struct Instructions {
    graph: HashMap<char, Vec<char>>,
    reqs: HashMap<char, Vec<char>>,
    root: Vec<char>,
}

impl Instructions {
    fn from_string(input: &str) -> Self {
        let mut graph = HashMap::new();
        let mut reqs = HashMap::new();

        let re = Regex::new(r"Step (\w) must be finished before step (\w) can begin.").unwrap();
        for cap in re.captures_iter(input) {
            let parent = cap[1].chars().next().unwrap();
            let child = cap[2].chars().next().unwrap();

            let children = graph.entry(parent).or_insert_with(Vec::new);
            children.push(child);

            let req_list = reqs.entry(child).or_insert_with(Vec::new);
            req_list.push(parent);
        }

        // Sorting now makes later jobs easier
        for children in graph.values_mut() {
            children.sort_unstable();
        }
        for req_list in reqs.values_mut() {
            req_list.sort_unstable();
        }

        // Find the end step(s) - any children that are not also parents
        let children: Vec<char> = graph.keys().copied().collect();
        let mut parents: Vec<char> = graph.values().cloned().flatten().collect();
        parents.sort_unstable();
        parents.dedup();
        let mut root: Vec<char> = children.iter().filter(|&&p| parents.contains(&p) == false).copied().collect();
        root.sort_unstable();

        Self {
            graph,
            reqs,
            root,
        }
    }

    fn emit_order(&self) -> String {
        let mut order = Vec::new();

        let mut frontier: Vec<char> = self.root.clone();
        while frontier.is_empty() == false {
            let next = frontier.remove(0);
            order.push(next);

            if let Some(children) = self.graph.get(&next) {
                for &child in children {
                    let is_ready = self.reqs.get(&child).unwrap().iter().all(|req| order.contains(req));
                    if is_ready == true {
                        frontier.push(child);
                    }
                }
                frontier.sort_unstable();
                frontier.dedup();
            }
        }

        order.iter().collect()
    }
}

#[aoc(day7, part1)]
pub fn solve(input: &str) -> String {
    let instructions = Instructions::from_string(input);
    let order = instructions.emit_order();
    println!("Order: {}", order);
    assert_eq!(order, "HPDTNXYLOCGEQSIMABZKRUWVFJ");
    order
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_emit_order() {
        let input = "
Step C must be finished before step A can begin.
Step C must be finished before step F can begin.
Step A must be finished before step B can begin.
Step A must be finished before step D can begin.
Step B must be finished before step E can begin.
Step D must be finished before step E can begin.
Step F must be finished before step E can begin.";
        let instructions = Instructions::from_string(input);
        let order = instructions.emit_order();
        assert_eq!(order, "CABDFE");
    }
}
