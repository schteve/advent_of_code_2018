/*
    --- Part Two ---
    As you're about to begin construction, four of the Elves offer to help. "The sun will set soon; it'll go faster if we work together." Now, you need to account for multiple people working on steps simultaneously. If multiple steps are available, workers should still begin them in alphabetical order.

    Each step takes 60 seconds plus an amount corresponding to its letter: A=1, B=2, C=3, and so on. So, step A takes 60+1=61 seconds, while step Z takes 60+26=86 seconds. No time is required between steps.

    To simplify things for the example, however, suppose you only have help from one Elf (a total of two workers) and that each step takes 60 fewer seconds (so that step A takes 1 second and step Z takes 26 seconds). Then, using the same instructions as above, this is how each second would be spent:

    Second   Worker 1   Worker 2   Done
    0        C          .
    1        C          .
    2        C          .
    3        A          F       C
    4        B          F       CA
    5        B          F       CA
    6        D          F       CAB
    7        D          F       CAB
    8        D          F       CAB
    9        D          .       CABF
    10        E          .       CABFD
    11        E          .       CABFD
    12        E          .       CABFD
    13        E          .       CABFD
    14        E          .       CABFD
    15        .          .       CABFDE
    Each row represents one second of time. The Second column identifies how many seconds have passed as of the beginning of that second. Each worker column shows the step that worker is currently doing (or . if they are idle). The Done column shows completed steps.

    Note that the order of the steps has changed; this is because steps now take time to finish and multiple workers can begin multiple steps simultaneously.

    In this example, it would take 15 seconds for two workers to complete these steps.

    With 5 workers and the 60+ second step durations described above, how long will it take to complete all of the steps?
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
        let mut root: Vec<char> = children
            .iter()
            .filter(|&&p| parents.contains(&p) == false)
            .copied()
            .collect();
        root.sort_unstable();

        Self { graph, reqs, root }
    }

    fn emit_order_and_time(&self, num_workers: u8, ascii_offset: u8) -> (String, u32) {
        let mut total_time = 0;
        let mut order = Vec::new();

        let mut workers = vec![Option::<(char, u8)>::None; num_workers as usize];
        let mut frontier = self.root.clone();
        while frontier.is_empty() == false || workers.iter().any(|w| w.is_some()) {
            // Allocate jobs to any idle workers
            for worker in workers.iter_mut() {
                if worker.is_none() && frontier.is_empty() == false {
                    let next = frontier.remove(0);
                    let time = next as u8 - ascii_offset;
                    *worker = Some((next, time));
                }
            }

            // Fast forward to the next time a job is done
            let shortest_job = workers
                .iter()
                .filter_map(|&w| w)
                .map(|w| w.1)
                .min()
                .unwrap();
            for worker in workers.iter_mut() {
                if worker.is_some() == true {
                    let (job, time) = worker.unwrap();
                    let new_time = time - shortest_job;
                    if new_time == 0 {
                        *worker = None;
                        order.push(job);

                        // Add any children to the frontier if they haven't already been visited
                        if let Some(children) = self.graph.get(&job) {
                            for &child in children {
                                let is_ready = self
                                    .reqs
                                    .get(&child)
                                    .unwrap()
                                    .iter()
                                    .all(|req| order.contains(req));
                                if is_ready == true {
                                    frontier.push(child);
                                }
                            }
                        }
                    } else {
                        *worker = Some((job, new_time));
                    }
                }
            }
            total_time += shortest_job as u32;

            // In case the frontier changed, organize it again.
            frontier.sort_unstable();
            frontier.dedup();
        }

        (order.iter().collect(), total_time)
    }
}

#[aoc(day7, part2)]
pub fn solve(input: &str) -> u32 {
    let instructions = Instructions::from_string(input);
    let (_order, total_time) = instructions.emit_order_and_time(5, 4); // 5 workers, and ASCII 'A' is 65 but we want it to be 61

    //println!("Order: {}", _order);
    println!("Total time: {}", total_time);
    assert_eq!(total_time, 908);
    total_time
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_emit_order_and_time() {
        let input = "
Step C must be finished before step A can begin.
Step C must be finished before step F can begin.
Step A must be finished before step B can begin.
Step A must be finished before step D can begin.
Step B must be finished before step E can begin.
Step D must be finished before step E can begin.
Step F must be finished before step E can begin.";
        let instructions = Instructions::from_string(input);
        let (order, total_time) = instructions.emit_order_and_time(2, 64); // A = 1
        assert_eq!(order, "CABFDE");
        assert_eq!(total_time, 15);
    }
}
