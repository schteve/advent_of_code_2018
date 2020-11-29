/*
    --- Part Two ---
    According to your calculations, the Elves are going to lose badly. Surely, you won't mess up the timeline too much if you give them just a little advanced technology, right?

    You need to make sure the Elves not only win, but also suffer no losses: even the death of a single Elf is unacceptable.

    However, you can't go too far: larger changes will be more likely to permanently alter spacetime.

    So, you need to find the outcome of the battle in which the Elves have the lowest integer attack power (at least 4) that allows them to win without a single death. The Goblins always have an attack power of 3.

    In the first summarized example above, the lowest attack power the Elves need to win without losses is 15:

    #######       #######
    #.G...#       #..E..#   E(158)
    #...EG#       #...E.#   E(14)
    #.#.#G#  -->  #.#.#.#
    #..G#E#       #...#.#
    #.....#       #.....#
    #######       #######

    Combat ends after 29 full rounds
    Elves win with 172 total hit points left
    Outcome: 29 * 172 = 4988
    In the second example above, the Elves need only 4 attack power:

    #######       #######
    #E..EG#       #.E.E.#   E(200), E(23)
    #.#G.E#       #.#E..#   E(200)
    #E.##E#  -->  #E.##E#   E(125), E(200)
    #G..#.#       #.E.#.#   E(200)
    #..E#.#       #...#.#
    #######       #######

    Combat ends after 33 full rounds
    Elves win with 948 total hit points left
    Outcome: 33 * 948 = 31284
    In the third example above, the Elves need 15 attack power:

    #######       #######
    #E.G#.#       #.E.#.#   E(8)
    #.#G..#       #.#E..#   E(86)
    #G.#.G#  -->  #..#..#
    #G..#.#       #...#.#
    #...E.#       #.....#
    #######       #######

    Combat ends after 37 full rounds
    Elves win with 94 total hit points left
    Outcome: 37 * 94 = 3478
    In the fourth example above, the Elves need 12 attack power:

    #######       #######
    #.E...#       #...E.#   E(14)
    #.#..G#       #.#..E#   E(152)
    #.###.#  -->  #.###.#
    #E#G#G#       #.#.#.#
    #...#G#       #...#.#
    #######       #######

    Combat ends after 39 full rounds
    Elves win with 166 total hit points left
    Outcome: 39 * 166 = 6474
    In the last example above, the lone Elf needs 34 attack power:

    #########       #########
    #G......#       #.......#
    #.E.#...#       #.E.#...#   E(38)
    #..##..G#       #..##...#
    #...##..#  -->  #...##..#
    #...#...#       #...#...#
    #.G...G.#       #.......#
    #.....G.#       #.......#
    #########       #########

    Combat ends after 30 full rounds
    Elves win with 38 total hit points left
    Outcome: 30 * 38 = 1140
    After increasing the Elves' attack power until it is just barely enough for them to win without any Elves dying, what is the outcome of the combat described in your puzzle input?
*/

use crate::common::Point;
use std::collections::BTreeMap;
use std::collections::BTreeSet;
use std::fmt;
use std::fmt::Write;
use std::iter::FromIterator;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum Tile {
    Empty,
    Wall,
    Goblin(u32),
    Elf(u32),
}

impl Tile {
    fn from_char(c: char) -> Self {
        match c {
            '.' => Self::Empty,
            '#' => Self::Wall,
            'G' => Self::Goblin(200),
            'E' => Self::Elf(200),
            _ => panic!("Unknown tile: {}", c),
        }
    }

    fn to_char(&self) -> char {
        match *self {
            Self::Empty => '.',
            Self::Wall => '#',
            Self::Goblin(_) => 'G',
            Self::Elf(_) => 'E',
        }
    }
}

impl fmt::Display for Tile {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.to_char())?;
        match self {
            Self::Empty => (),
            Self::Wall => (),
            Self::Goblin(x) => write!(f, "({})", x)?,
            Self::Elf(x) => write!(f, "({})", x)?,
        };
        Ok(())
    }
}

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
struct Path {
    first: Point,
    last: Point,
    length: usize,
}

#[derive(Clone)]
struct BattleMap {
    tiles: BTreeMap<Point, Tile>,
    goblin_atk: u32,
    elf_atk: u32,
}

impl BattleMap {
    fn from_string(input: &str) -> Self {
        let mut tiles = BTreeMap::new();
        let mut p = Point::new();
        for line in input.trim().lines() {
            for c in line.chars() {
                tiles.insert(p, Tile::from_char(c));
                p.x += 1;
            }
            p.x = 0;
            p.y += 1;
        }

        Self {
            tiles,
            goblin_atk: 3,
            elf_atk: 3,
        }
    }

    fn get_range(&self) -> ((i32, i32), (i32, i32)) {
        let mut tiles_iter = self.tiles.iter();
        if let Some((point, _tile)) = tiles_iter.next() {
            tiles_iter.fold(
                ((point.x, point.x), (point.y, point.y)),
                |(acc_x, acc_y), (p, _)| {
                    (
                        (acc_x.0.min(p.x), acc_x.1.max(p.x)),
                        (acc_y.0.min(p.y), acc_y.1.max(p.y)),
                    )
                },
            )
        } else {
            ((0, 0), (0, 0))
        }
    }

    fn to_string(&self, with_details: bool) -> String {
        let mut output = String::new();
        let range = self.get_range();
        for y in (range.1).0..=(range.1).1 {
            let mut unit_strings = Vec::new();
            for x in (range.0).0..=(range.0).1 {
                if let Some(tile) = self.tiles.get(&Point { x, y }) {
                    write!(output, "{}", tile.to_char()).unwrap();
                    if with_details == true {
                        match tile {
                            Tile::Goblin(_) | Tile::Elf(_) => unit_strings.push(tile.to_string()),
                            _ => (),
                        }
                    }
                } else {
                    write!(output, ".").unwrap();
                }
            }
            if with_details == true && unit_strings.is_empty() == false {
                write!(output, "   {}", unit_strings.join(", ")).unwrap();
            }
            writeln!(output).unwrap();
        }
        output
    }

    // Returns in reading order
    fn identify_units(&self) -> Vec<Point> {
        let mut units: Vec<Point> = self
            .tiles
            .iter()
            .filter(|&(_point, tile)| matches!(tile, Tile::Goblin(_) | Tile::Elf(_)))
            .map(|(&point, &_tile)| point)
            .collect();
        units.sort_by(Point::cmp_y_x);
        units
    }

    fn identify_enemies(&self, am_goblin: bool) -> Vec<Point> {
        let units: Vec<Point> = self
            .tiles
            .iter()
            .filter(|&(_point, tile)| match tile {
                Tile::Goblin(_) => am_goblin == false,
                Tile::Elf(_) => am_goblin == true,
                _ => false,
            })
            .map(|(&point, &_tile)| point)
            .collect();
        units
    }

    fn identify_adjacent_empty(&self, unit: &Point) -> Vec<Point> {
        unit.orthogonals()
            .into_iter()
            .filter(|&p| self.tiles.get(&p) == Some(&Tile::Empty))
            .collect()
    }

    fn identify_adjacent_enemies(&self, unit: &Point, am_goblin: bool) -> Vec<Point> {
        unit.orthogonals()
            .into_iter()
            .filter(|&point| match self.tiles.get(&point) {
                Some(Tile::Goblin(_)) => am_goblin == false,
                Some(Tile::Elf(_)) => am_goblin == true,
                _ => false,
            })
            .collect()
    }

    fn find_paths(&self, from: &Point, to: &[Point]) -> Vec<Path> {
        let mut paths: BTreeSet<Path> = BTreeSet::new();
        let mut shortest_path = None;
        let to_set: BTreeSet<Point> = BTreeSet::from_iter(to.iter().cloned());

        let starts: Vec<Point> = self.identify_adjacent_empty(from);
        for start in starts {
            let mut visited: BTreeSet<Point> = BTreeSet::new();
            visited.insert(*from);
            visited.insert(start);

            let mut frontier: Vec<Point> = Vec::new();
            frontier.push(start);
            let mut distance = 0;
            while frontier.is_empty() == false {
                distance += 1;
                if let Some(shortest) = shortest_path {
                    if distance > shortest {
                        break; // As an optimization, quit searching for paths if already exceeded the shortest we've seen
                    }
                }

                let mut found_a_target = false;
                for frontier_point in frontier.drain(..).collect::<Vec<Point>>() {
                    if to_set.contains(&frontier_point) == true {
                        found_a_target = true;
                        let path = Path {
                            first: start,
                            last: frontier_point,
                            length: distance,
                        };
                        paths.insert(path);
                    }
                    visited.insert(frontier_point);

                    let next: Vec<Point> = self
                        .identify_adjacent_empty(&frontier_point)
                        .into_iter()
                        .filter(|point| visited.get(&point) == None)
                        .collect();
                    for &n in &next {
                        if frontier.contains(&n) == false {
                            frontier.push(n);
                        }
                    }
                }

                // As an optimization we quit searching once we found any target (finishing the current frontier). It is
                // guaranteed that we won't need any longer paths since the next step after finding the paths would be
                // culling all but the shortest ones.
                if found_a_target == true {
                    if let Some(shortest) = shortest_path {
                        if distance < shortest {
                            shortest_path = Some(distance);
                        }
                    } else {
                        shortest_path = Some(distance);
                    }
                    break;
                }
            }
        }

        // Convert from BTreeSet to Vec, removing all but the shortest paths
        let paths_vec: Vec<Path> = Vec::from_iter(
            paths
                .iter()
                .filter(|path| path.length == shortest_path.unwrap())
                .cloned(),
        );
        paths_vec
    }

    fn tick(&mut self) -> bool {
        let units = self.identify_units();
        for unit in units {
            let mut unit_location = unit;

            // Is this unit a goblin or an elf?
            let am_goblin = match self.tiles.get(&unit_location) {
                Some(Tile::Goblin(_)) => true,
                Some(Tile::Elf(_)) => false,
                _ => continue, // Unit was killed in an earlier iteration
            };

            // If there are no adjacent enemies, move towards one
            let adjacent_enemies = self.identify_adjacent_enemies(&unit_location, am_goblin);
            if adjacent_enemies.is_empty() == true {
                // Find all spaces adjacent to all enemies on the map
                let enemies = self.identify_enemies(am_goblin);
                if enemies.is_empty() == true {
                    // No more enemies, end immediately
                    return false;
                }
                let mut adjacents: Vec<Point> = enemies
                    .into_iter()
                    .flat_map(|enemy| self.identify_adjacent_empty(&enemy))
                    .collect();
                adjacents.sort_by(Point::cmp_y_x);
                adjacents.dedup();
                if adjacents.is_empty() == true {
                    // No possible targets
                    continue;
                }

                // Find the shortest path(s) to each reachable space
                let mut paths: Vec<Path> = self.find_paths(&unit_location, &adjacents);
                paths.sort_by_key(|path| path.last); // The paths are usually, but not always, in reading order by target. Ensure it by sorting.
                if paths.is_empty() == true {
                    // No paths to a target
                    continue;
                }

                // Keep only paths to the first target in reading order
                let target = paths[0].last;
                let paths_to_target: Vec<Path> = paths
                    .into_iter()
                    .filter(|path| path.last == target)
                    .collect();

                // Take the first step in reading order
                let mut steps: Vec<Point> =
                    paths_to_target.into_iter().map(|path| path.first).collect();
                steps.sort_by(Point::cmp_y_x);
                unit_location = steps[0];

                // Step by overwriting current space and new space
                let unit_data = self.tiles.insert(unit, Tile::Empty).unwrap();
                let previous = self.tiles.insert(unit_location, unit_data).unwrap();
                assert_eq!(previous, Tile::Empty);
            }

            // Check again if there are adjacent enemies (we may have moved). Attack if possible.
            let enemies_can_attack: Vec<Point> =
                self.identify_adjacent_enemies(&unit_location, am_goblin);
            if enemies_can_attack.is_empty() == false {
                // Find enemies with lowest HP
                let least_hp = enemies_can_attack
                    .iter()
                    .map(|&enemy| match self.tiles.get(&enemy) {
                        Some(&Tile::Goblin(x)) => x,
                        Some(&Tile::Elf(x)) => x,
                        _ => panic!("Unexpected tile"),
                    })
                    .min()
                    .unwrap();
                let mut enemies_least_hp: Vec<Point> = enemies_can_attack
                    .into_iter()
                    .filter(|&enemy| match self.tiles.get(&enemy) {
                        Some(&Tile::Goblin(x)) => x == least_hp,
                        Some(&Tile::Elf(x)) => x == least_hp,
                        _ => panic!("Unexpected tile"),
                    })
                    .collect();
                enemies_least_hp.sort_by(Point::cmp_y_x);

                // Attack first enemy in the list
                if enemies_least_hp.is_empty() == false {
                    let enemy = enemies_least_hp[0];
                    let enemy_adjusted = match self.tiles.get(&enemy) {
                        Some(&Tile::Goblin(x)) => {
                            if x <= self.elf_atk {
                                Tile::Empty // Dead goblin
                            } else {
                                Tile::Goblin(x - self.elf_atk)
                            }
                        }
                        Some(&Tile::Elf(x)) => {
                            if x <= self.goblin_atk {
                                Tile::Empty // Dead elf
                            } else {
                                Tile::Elf(x - self.goblin_atk)
                            }
                        }
                        _ => panic!("Unexpected tile"),
                    };
                    self.tiles.insert(enemy, enemy_adjusted);
                }
            }
        }

        true
    }

    fn count_goblins(&self) -> u32 {
        self.tiles
            .iter()
            .filter(|&(_point, tile)| matches!(tile, Tile::Goblin(_)))
            .count() as u32
    }

    fn count_elves(&self) -> u32 {
        self.tiles
            .iter()
            .filter(|&(_point, tile)| matches!(tile, Tile::Elf(_)))
            .count() as u32
    }

    fn calculate_score(&self, rounds: u32) -> u32 {
        let total_hit_points: u32 = self
            .tiles
            .iter()
            .map(|(_point, tile)| match tile {
                &Tile::Goblin(x) | &Tile::Elf(x) => x,
                _ => 0,
            })
            .sum();
        rounds * total_hit_points
    }

    fn battle(&mut self) -> u32 {
        let mut rounds = 0;
        loop {
            if self.count_goblins() == 0 || self.count_elves() == 0 {
                // Simulation is over. Calculate final score.
                return self.calculate_score(rounds);
            }

            if self.tick() == true {
                rounds += 1;
            }
        }
    }

    fn power_up_elves(&mut self) -> (u32, u32) {
        let mut elf_power = 4; // One more than default of 3
        loop {
            let mut parallel_reality = self.clone();
            parallel_reality.elf_atk = elf_power;

            let start_elf_count = parallel_reality.count_elves();
            let score = parallel_reality.battle();
            let end_elf_count = parallel_reality.count_elves();

            if start_elf_count == end_elf_count {
                // No elves died
                *self = parallel_reality; // Not strictly necessary to copy this over, but it makes it possible to externally test the result
                return (score, elf_power);
            }

            elf_power += 1;
        }
    }
}

impl fmt::Display for BattleMap {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.to_string(false))
    }
}

#[aoc(day15, part2)]
pub fn solve(input: &str) -> u32 {
    let mut battle_map = BattleMap::from_string(input);
    let (score, _power) = battle_map.power_up_elves();
    println!("Score: {}", score);
    assert_eq!(score, 41804);
    score
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_tick1() {
        let input = "
#########
#G..G..G#
#.......#
#.......#
#G..E..G#
#.......#
#.......#
#G..G..G#
#########";
        let mut battle_map = BattleMap::from_string(input);

        let results = vec![
            "
#########
#.G...G.#
#...G...#
#...E..G#
#.G.....#
#.......#
#G..G..G#
#.......#
#########",
            "
#########
#..G.G..#
#...G...#
#.G.E.G.#
#.......#
#G..G..G#
#.......#
#.......#
#########",
            "
#########
#.......#
#..GGG..#
#..GEG..#
#G..G...#
#......G#
#.......#
#.......#
#########",
        ];
        for (i, result) in results.iter().enumerate() {
            println!("Round: {}", i);
            battle_map.tick();
            println!("{}", battle_map);
            assert_eq!(battle_map.to_string(false).trim(), result.trim());
        }
    }

    #[test]
    fn test_tick2() {
        let input = "
#######
#.G...#
#...EG#
#.#.#G#
#..G#E#
#.....#
#######";
        let mut battle_map = BattleMap::from_string(input);

        let results: Vec<(usize, &str)> = vec![
            (
                1,
                "
#######
#..G..#   G(200)
#...EG#   E(197), G(197)
#.#G#G#   G(200), G(197)
#...#E#   E(197)
#.....#
#######",
            ),
            (
                2,
                "
#######
#...G.#   G(200)
#..GEG#   G(200), E(188), G(194)
#.#.#G#   G(194)
#...#E#   E(194)
#.....#
#######",
            ),
            (
                23,
                "
#######
#...G.#   G(200)
#..G.G#   G(200), G(131)
#.#.#G#   G(131)
#...#E#   E(131)
#.....#
#######",
            ),
            (
                24,
                "
#######
#..G..#   G(200)
#...G.#   G(131)
#.#G#G#   G(200), G(128)
#...#E#   E(128)
#.....#
#######",
            ),
            (
                25,
                "
#######
#.G...#   G(200)
#..G..#   G(131)
#.#.#G#   G(125)
#..G#E#   G(200), E(125)
#.....#
#######",
            ),
            (
                26,
                "
#######
#G....#   G(200)
#.G...#   G(131)
#.#.#G#   G(122)
#...#E#   E(122)
#..G..#   G(200)
#######",
            ),
            (
                27,
                "
#######
#G....#   G(200)
#.G...#   G(131)
#.#.#G#   G(119)
#...#E#   E(119)
#...G.#   G(200)
#######",
            ),
            (
                28,
                "
#######
#G....#   G(200)
#.G...#   G(131)
#.#.#G#   G(116)
#...#E#   E(113)
#....G#   G(200)
#######",
            ),
            (
                47,
                "
#######
#G....#   G(200)
#.G...#   G(131)
#.#.#G#   G(59)
#...#.#
#....G#   G(200)
#######",
            ),
        ];
        let mut counter = 0;
        for &(round, result) in results.iter() {
            while counter < round {
                println!("Round: {}", counter);
                battle_map.tick();
                println!("{}", battle_map);
                counter += 1;
            }
            println!("End round: {}", counter);
            assert_eq!(battle_map.to_string(true).trim(), result.trim());
        }
    }

    #[test]
    fn test_battle() {
        let inputs_results_scores: Vec<(&str, &str, u32)> = vec![
            (
                "
#######
#.G...#
#...EG#
#.#.#G#
#..G#E#
#.....#
#######",
                "
#######
#G....#   G(200)
#.G...#   G(131)
#.#.#G#   G(59)
#...#.#
#....G#   G(200)
#######",
                27730,
            ),
            (
                "
#######
#G..#E#
#E#E.E#
#G.##.#
#...#E#
#...E.#
#######",
                "
#######
#...#E#   E(200)
#E#...#   E(197)
#.E##.#   E(185)
#E..#E#   E(200), E(200)
#.....#
#######",
                36334,
            ),
            (
                "
#######
#E..EG#
#.#G.E#
#E.##E#
#G..#.#
#..E#.#
#######",
                "
#######
#.E.E.#   E(164), E(197)
#.#E..#   E(200)
#E.##.#   E(98)
#.E.#.#   E(200)
#...#.#
#######",
                39514,
            ),
            (
                "
#######
#E.G#.#
#.#G..#
#G.#.G#
#G..#.#
#...E.#
#######",
                "
#######
#G.G#.#   G(200), G(98)
#.#G..#   G(200)
#..#..#
#...#G#   G(95)
#...G.#   G(200)
#######",
                27755,
            ),
            (
                "
#######
#.E...#
#.#..G#
#.###.#
#E#G#G#
#...#G#
#######",
                "
#######
#.....#
#.#G..#   G(200)
#.###.#
#.#.#.#
#G.G#G#   G(98), G(38), G(200)
#######",
                28944,
            ),
            (
                "
#########
#G......#
#.E.#...#
#..##..G#
#...##..#
#...#...#
#.G...G.#
#.....G.#
#########",
                "
#########
#.G.....#   G(137)
#G.G#...#   G(200), G(200)
#.G##...#   G(200)
#...##..#
#.G.#...#   G(200)
#.......#
#.......#
#########",
                18740,
            ),
        ];

        for (input, result, score) in inputs_results_scores {
            let mut battle_map = BattleMap::from_string(input);
            let battle_score = battle_map.battle();
            assert_eq!(battle_score, score);
            assert_eq!(battle_map.to_string(true).trim(), result.trim());
        }
    }

    #[test]
    fn test_power_up_elves() {
        let inputs_results_scores_power: Vec<(&str, &str, u32, u32)> = vec![
            (
                "
#######
#.G...#
#...EG#
#.#.#G#
#..G#E#
#.....#
#######",
                "
#######
#..E..#   E(158)
#...E.#   E(14)
#.#.#.#
#...#.#
#.....#
#######",
                4988,
                15,
            ),
            (
                "
#######
#E..EG#
#.#G.E#
#E.##E#
#G..#.#
#..E#.#
#######",
                "
#######
#.E.E.#   E(200), E(23)
#.#E..#   E(200)
#E.##E#   E(125), E(200)
#.E.#.#   E(200)
#...#.#
#######",
                31284,
                4,
            ),
            (
                "
#######
#E.G#.#
#.#G..#
#G.#.G#
#G..#.#
#...E.#
#######",
                "
#######
#.E.#.#   E(8)
#.#E..#   E(86)
#..#..#
#...#.#
#.....#
#######",
                3478,
                15,
            ),
            (
                "
#######
#.E...#
#.#..G#
#.###.#
#E#G#G#
#...#G#
#######",
                "
#######
#...E.#   E(14)
#.#..E#   E(152)
#.###.#
#.#.#.#
#...#.#
#######",
                6474,
                12,
            ),
            (
                "
#########
#G......#
#.E.#...#
#..##..G#
#...##..#
#...#...#
#.G...G.#
#.....G.#
#########",
                "
#########
#.......#
#.E.#...#   E(38)
#..##...#
#...##..#
#...#...#
#.......#
#.......#
#########",
                1140,
                34,
            ),
        ];

        for (input, result, score, power) in inputs_results_scores_power {
            let mut battle_map = BattleMap::from_string(input);
            let (battle_score, battle_power) = battle_map.power_up_elves();
            assert_eq!(battle_score, score);
            assert_eq!(battle_power, power);
            assert_eq!(battle_map.to_string(true).trim(), result.trim());
        }
    }
}
