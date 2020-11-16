/*
    --- Day 20: A Regular Map ---
    While you were learning about instruction pointers, the Elves made considerable progress. When you look up, you discover that the North Pole base construction project has completely surrounded you.

    The area you are in is made up entirely of rooms and doors. The rooms are arranged in a grid, and rooms only connect to adjacent rooms when a door is present between them.

    For example, drawing rooms as ., walls as #, doors as | or -, your current position as X, and where north is up, the area you're in might look like this:

    #####
    #.|.#
    #-###
    #.|X#
    #####
    You get the attention of a passing construction Elf and ask for a map. "I don't have time to draw out a map of this place - it's huge. Instead, I can give you directions to every room in the facility!" He writes down some directions on a piece of parchment and runs off. In the example above, the instructions might have been ^WNE$, a regular expression or "regex" (your puzzle input).

    The regex matches routes (like WNE for "west, north, east") that will take you from your current room through various doors in the facility. In aggregate, the routes will take you through every door in the facility at least once; mapping out all of these routes will let you build a proper map and find your way around.

    ^ and $ are at the beginning and end of your regex; these just mean that the regex doesn't match anything outside the routes it describes. (Specifically, ^ matches the start of the route, and $ matches the end of it.) These characters will not appear elsewhere in the regex.

    The rest of the regex matches various sequences of the characters N (north), S (south), E (east), and W (west). In the example above, ^WNE$ matches only one route, WNE, which means you can move west, then north, then east from your current position. Sequences of letters like this always match that exact route in the same order.

    Sometimes, the route can branch. A branch is given by a list of options separated by pipes (|) and wrapped in parentheses. So, ^N(E|W)N$ contains a branch: after going north, you must choose to go either east or west before finishing your route by going north again. By tracing out the possible routes after branching, you can determine where the doors are and, therefore, where the rooms are in the facility.

    For example, consider this regex: ^ENWWW(NEEE|SSE(EE|N))$

    This regex begins with ENWWW, which means that from your current position, all routes must begin by moving east, north, and then west three times, in that order. After this, there is a branch. Before you consider the branch, this is what you know about the map so far, with doors you aren't sure about marked with a ?:

    #?#?#?#?#
    ?.|.|.|.?
    #?#?#?#-#
        ?X|.?
        #?#?#
    After this point, there is (NEEE|SSE(EE|N)). This gives you exactly two options: NEEE and SSE(EE|N). By following NEEE, the map now looks like this:

    #?#?#?#?#
    ?.|.|.|.?
    #-#?#?#?#
    ?.|.|.|.?
    #?#?#?#-#
        ?X|.?
        #?#?#
    Now, only SSE(EE|N) remains. Because it is in the same parenthesized group as NEEE, it starts from the same room NEEE started in. It states that starting from that point, there exist doors which will allow you to move south twice, then east; this ends up at another branch. After that, you can either move east twice or north once. This information fills in the rest of the doors:

    #?#?#?#?#
    ?.|.|.|.?
    #-#?#?#?#
    ?.|.|.|.?
    #-#?#?#-#
    ?.?.?X|.?
    #-#-#?#?#
    ?.|.|.|.?
    #?#?#?#?#
    Once you've followed all possible routes, you know the remaining unknown parts are all walls, producing a finished map of the facility:

    #########
    #.|.|.|.#
    #-#######
    #.|.|.|.#
    #-#####-#
    #.#.#X|.#
    #-#-#####
    #.|.|.|.#
    #########
    Sometimes, a list of options can have an empty option, like (NEWS|WNSE|). This means that routes at this point could effectively skip the options in parentheses and move on immediately. For example, consider this regex and the corresponding map:

    ^ENNWSWW(NEWS|)SSSEEN(WNSE|)EE(SWEN|)NNN$

    ###########
    #.|.#.|.#.#
    #-###-#-#-#
    #.|.|.#.#.#
    #-#####-#-#
    #.#.#X|.#.#
    #-#-#####-#
    #.#.|.|.|.#
    #-###-###-#
    #.|.|.#.|.#
    ###########
    This regex has one main route which, at three locations, can optionally include additional detours and be valid: (NEWS|), (WNSE|), and (SWEN|). Regardless of which option is taken, the route continues from the position it is left at after taking those steps. So, for example, this regex matches all of the following routes (and more that aren't listed here):

    ENNWSWWSSSEENEENNN
    ENNWSWWNEWSSSSEENEENNN
    ENNWSWWNEWSSSSEENEESWENNNN
    ENNWSWWSSSEENWNSEEENNN
    By following the various routes the regex matches, a full map of all of the doors and rooms in the facility can be assembled.

    To get a sense for the size of this facility, you'd like to determine which room is furthest from you: specifically, you would like to find the room for which the shortest path to that room would require passing through the most doors.

    In the first example (^WNE$), this would be the north-east corner 3 doors away.
    In the second example (^ENWWW(NEEE|SSE(EE|N))$), this would be the south-east corner 10 doors away.
    In the third example (^ENNWSWW(NEWS|)SSSEEN(WNSE|)EE(SWEN|)NNN$), this would be the north-east corner 18 doors away.
    Here are a few more examples:

    Regex: ^ESSWWN(E|NNENN(EESS(WNSE|)SSS|WWWSSSSE(SW|NNNE)))$
    Furthest room requires passing 23 doors

    #############
    #.|.|.|.|.|.#
    #-#####-###-#
    #.#.|.#.#.#.#
    #-#-###-#-#-#
    #.#.#.|.#.|.#
    #-#-#-#####-#
    #.#.#.#X|.#.#
    #-#-#-###-#-#
    #.|.#.|.#.#.#
    ###-#-###-#-#
    #.|.#.|.|.#.#
    #############
    Regex: ^WSSEESWWWNW(S|NENNEEEENN(ESSSSW(NWSW|SSEN)|WSWWN(E|WWS(E|SS))))$
    Furthest room requires passing 31 doors

    ###############
    #.|.|.|.#.|.|.#
    #-###-###-#-#-#
    #.|.#.|.|.#.#.#
    #-#########-#-#
    #.#.|.|.|.|.#.#
    #-#-#########-#
    #.#.#.|X#.|.#.#
    ###-#-###-#-#-#
    #.|.#.#.|.#.|.#
    #-###-#####-###
    #.|.#.|.|.#.#.#
    #-#-#####-#-#-#
    #.#.|.|.|.#.|.#
    ###############
    What is the largest number of doors you would be required to pass through to reach a room? That is, find the room for which the shortest path from your starting location to that room would require passing through the most doors; what is the fewest doors you can pass through to reach it?
*/

use crate::common::Cardinal;
use crate::common::Point;
use std::collections::HashMap;
use std::fmt;

struct Room {
    door_north: bool,
    door_south: bool,
    door_east: bool,
    door_west: bool,
}

impl Room {
    fn new() -> Self {
        Self {
            door_north: false,
            door_south: false,
            door_east: false,
            door_west: false,
        }
    }

    fn has_door(&self, dir: Cardinal) -> bool {
        match dir {
            Cardinal::North => self.door_north,
            Cardinal::South => self.door_south,
            Cardinal::East => self.door_east,
            Cardinal::West => self.door_west,
        }
    }

    fn set_door(&mut self, dir: Cardinal, door: bool) {
        match dir {
            Cardinal::North => self.door_north = door,
            Cardinal::South => self.door_south = door,
            Cardinal::East => self.door_east = door,
            Cardinal::West => self.door_west = door,
        }
    }
}

struct Frame {
    start: Vec<Point>,           // Tracks the routes available at the frame start
    finished_routes: Vec<Point>, // The routes in this frame that already ended in this frame
}

struct Map {
    rooms: HashMap<Point, Room>,
}

impl Map {
    fn from_string(regex: &str) -> Self {
        let mut map = Self {
            rooms: HashMap::new(),
        };
        map.find_routes(regex);
        map
    }

    fn find_routes(&mut self, regex_str: &str) {
        let mut frames: Vec<Frame> = Vec::new(); // The frame stack
        let mut current_frame = Frame {
            start: vec![Point::new()],
            finished_routes: Vec::new(),
        };
        let mut routes = vec![Point::new()]; // The routes currently under consideration

        for c in regex_str.chars() {
            match c {
                'N' | 'S' | 'E' | 'W' => {
                    // A new step to take on every route currently under consideration
                    for p in routes.iter_mut() {
                        let dir = Cardinal::from_char(c);
                        self.add_room(*p, dir); // First add a door for the room we are leaving
                        *p = p.step(dir, 1);
                        self.add_room(*p, dir.opposite()); // Next add a door for the room we are entering
                    }
                }
                '|' => {
                    // Hit a branch, save off the newly finished routes and reset the current routes to the state at the start of frame
                    current_frame.finished_routes.append(&mut routes);
                    routes = current_frame.start.clone();
                }
                '(' => {
                    // Push this frame to the stack and start a new one. Current routes are carried forward
                    let new_frame = Frame {
                        start: routes.clone(),
                        finished_routes: Vec::new(),
                    };
                    frames.push(current_frame);
                    current_frame = new_frame;
                }
                ')' => {
                    // No data from this frame needs to be kept, the only thing carried forward are the current routes
                    // which will be built upon in the parent frame. Also deduplicate the
                    current_frame.finished_routes.append(&mut routes);
                    routes = current_frame.finished_routes;
                    routes.sort_unstable();
                    routes.dedup();

                    current_frame = frames.pop().unwrap();
                }
                '^' => (),
                '$' => return,
                _ => panic!("Unknown character: {}", c),
            }
        }

        panic!("Regex terminated unexpectedly");
    }

    fn add_route(&mut self, route: &str) {
        let mut current_point = Point::new();
        for c in route.chars() {
            let dir = Cardinal::from_char(c);
            self.add_room(current_point, dir); // First add a door for the room we are leaving
            current_point = current_point.step(dir, 1);
            self.add_room(current_point, dir.opposite()); // Next add a door for the room we are entering
        }
    }

    fn add_room(&mut self, point: Point, from_dir: Cardinal) {
        let room = self.rooms.entry(point).or_insert_with(Room::new);
        room.set_door(from_dir, true);
    }

    fn find_furthest_room(&self) -> u32 {
        let mut visited_steps: HashMap<Point, u32> = HashMap::new(); // Track which rooms were visited, indicating how many steps to each one
        let mut frontier: Vec<Point> = Vec::new();
        frontier.push(Point::new());

        let mut steps = 0;
        while frontier.is_empty() == false {
            for location in frontier.drain(..).collect::<Vec<Point>>() {
                visited_steps.insert(location, steps);

                let room = self
                    .rooms
                    .get(&location)
                    .expect("Visited room does not exist!");
                if room.door_north == true {
                    let next_location = location.step(Cardinal::North, 1);
                    if visited_steps.get(&next_location) == None {
                        frontier.push(next_location);
                    }
                }
                if room.door_south == true {
                    let next_location = location.step(Cardinal::South, 1);
                    if visited_steps.get(&next_location) == None {
                        frontier.push(next_location);
                    }
                }
                if room.door_east == true {
                    let next_location = location.step(Cardinal::East, 1);
                    if visited_steps.get(&next_location) == None {
                        frontier.push(next_location);
                    }
                }
                if room.door_west == true {
                    let next_location = location.step(Cardinal::West, 1);
                    if visited_steps.get(&next_location) == None {
                        frontier.push(next_location);
                    }
                }
            }

            // Remove any duplicates
            frontier.sort_unstable();
            frontier.dedup();

            steps += 1;
        }

        // Find the room with the furthest distance
        *visited_steps.values().max().unwrap()
    }
}

impl fmt::Display for Map {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // First, create a map of the pixels for each room (one room consists of a 3x3 grid of pixels)
        let mut pixels: HashMap<Point, char> = HashMap::new();
        for (p, r) in &self.rooms {
            let draw_p = Point {
                x: p.x * 2,
                y: p.y * 2,
            };

            // First, the four corners
            let mut next_char = '#';
            if let Some(prev) = pixels.insert(draw_p + Point { x: -1, y: -1 }, next_char) {
                assert_eq!(prev, next_char);
            }
            if let Some(prev) = pixels.insert(draw_p + Point { x: -1, y: 1 }, next_char) {
                assert_eq!(prev, next_char);
            }
            if let Some(prev) = pixels.insert(draw_p + Point { x: 1, y: -1 }, next_char) {
                assert_eq!(prev, next_char);
            }
            if let Some(prev) = pixels.insert(draw_p + Point { x: 1, y: 1 }, next_char) {
                assert_eq!(prev, next_char);
            }

            // Next, the four walls / doors
            next_char = if r.door_north == true { '-' } else { '#' };
            if let Some(prev) = pixels.insert(draw_p + Point { x: 0, y: -1 }, next_char) {
                assert_eq!(prev, next_char);
            }
            next_char = if r.door_south == true { '-' } else { '#' };
            if let Some(prev) = pixels.insert(draw_p + Point { x: 0, y: 1 }, next_char) {
                assert_eq!(prev, next_char);
            }
            next_char = if r.door_east == true { '|' } else { '#' };
            if let Some(prev) = pixels.insert(draw_p + Point { x: 1, y: 0 }, next_char) {
                assert_eq!(prev, next_char);
            }
            next_char = if r.door_west == true { '|' } else { '#' };
            if let Some(prev) = pixels.insert(draw_p + Point { x: -1, y: 0 }, next_char) {
                assert_eq!(prev, next_char);
            }

            // Last, the room itself
            next_char = if draw_p.x == 0 && draw_p.y == 0 {
                'X'
            } else {
                '.'
            };
            if let Some(prev) = pixels.insert(draw_p, next_char) {
                assert_eq!(prev, next_char);
            }
        }

        // Now write the entire pixel map
        let x_min = pixels.keys().map(|p| p.x).min().unwrap();
        let x_max = pixels.keys().map(|p| p.x).max().unwrap();
        let y_min = pixels.keys().map(|p| p.y).min().unwrap();
        let y_max = pixels.keys().map(|p| p.y).max().unwrap();
        writeln!(f)?;
        for y in y_min..=y_max {
            for x in x_min..=x_max {
                if let Some(pixel) = pixels.get(&Point { x, y }) {
                    write!(f, "{}", pixel)?;
                } else {
                    write!(f, " ")?;
                }
            }
            writeln!(f)?;
        }

        Ok(())
    }
}

#[aoc(day20, part1)]
pub fn solve(input: &str) -> u32 {
    let map = Map::from_string(input);
    // println!("{}", map);

    let result = map.find_furthest_room();
    println!("Furthest room: {}", result);
    assert_eq!(result, 3699);
    result
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_map_from_string() {
        let map = Map::from_string("^WNE$");
        assert_eq!(
            map.to_string(),
            "
#####
#.|.#
#-###
#.|X#
#####
"
        );

        let map = Map::from_string("^ENWWW(NEEE|SSE(EE|N))$");
        assert_eq!(
            map.to_string(),
            "
#########
#.|.|.|.#
#-#######
#.|.|.|.#
#-#####-#
#.#.#X|.#
#-#-#####
#.|.|.|.#
#########
"
        );

        let map = Map::from_string("^ENNWSWW(NEWS|)SSSEEN(WNSE|)EE(SWEN|)NNN$");
        assert_eq!(
            map.to_string(),
            "
###########
#.|.#.|.#.#
#-###-#-#-#
#.|.|.#.#.#
#-#####-#-#
#.#.#X|.#.#
#-#-#####-#
#.#.|.|.|.#
#-###-###-#
#.|.|.#.|.#
###########
"
        );

        let map = Map::from_string("^ESSWWN(E|NNENN(EESS(WNSE|)SSS|WWWSSSSE(SW|NNNE)))$");
        assert_eq!(
            map.to_string(),
            "
#############
#.|.|.|.|.|.#
#-#####-###-#
#.#.|.#.#.#.#
#-#-###-#-#-#
#.#.#.|.#.|.#
#-#-#-#####-#
#.#.#.#X|.#.#
#-#-#-###-#-#
#.|.#.|.#.#.#
###-#-###-#-#
#.|.#.|.|.#.#
#############
"
        );

        let map =
            Map::from_string("^WSSEESWWWNW(S|NENNEEEENN(ESSSSW(NWSW|SSEN)|WSWWN(E|WWS(E|SS))))$");
        assert_eq!(
            map.to_string(),
            "
###############
#.|.|.|.#.|.|.#
#-###-###-#-#-#
#.|.#.|.|.#.#.#
#-#########-#-#
#.#.|.|.|.|.#.#
#-#-#########-#
#.#.#.|X#.|.#.#
###-#-###-#-#-#
#.|.#.#.|.#.|.#
#-###-#####-###
#.|.#.|.|.#.#.#
#-#-#####-#-#-#
#.#.|.|.|.#.|.#
###############
"
        );
    }

    #[test]
    fn test_map_find_furthest_room() {
        let map = Map::from_string("^WNE$");
        assert_eq!(map.find_furthest_room(), 3);

        let map = Map::from_string("^ENWWW(NEEE|SSE(EE|N))$");
        assert_eq!(map.find_furthest_room(), 10);

        let map = Map::from_string("^ENNWSWW(NEWS|)SSSEEN(WNSE|)EE(SWEN|)NNN$");
        assert_eq!(map.find_furthest_room(), 18);

        let map = Map::from_string("^ESSWWN(E|NNENN(EESS(WNSE|)SSS|WWWSSSSE(SW|NNNE)))$");
        assert_eq!(map.find_furthest_room(), 23);

        let map =
            Map::from_string("^WSSEESWWWNW(S|NENNEEEENN(ESSSSW(NWSW|SSEN)|WSWWN(E|WWS(E|SS))))$");
        assert_eq!(map.find_furthest_room(), 31);
    }
}
