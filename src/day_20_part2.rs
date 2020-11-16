/*
    --- Part Two ---
    Okay, so the facility is big.

    How many rooms have a shortest path from your current location that pass through at least 1000 doors?
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

    fn count_far_rooms(&self) -> usize {
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
        visited_steps.values().filter(|&&d| d >= 1000).count()
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

#[aoc(day20, part2)]
pub fn solve(input: &str) -> usize {
    let map = Map::from_string(input);
    // println!("{}", map);

    let result = map.count_far_rooms();
    println!("Rooms >= 1000 away: {}", result);
    assert_eq!(result, 8517);
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
}
