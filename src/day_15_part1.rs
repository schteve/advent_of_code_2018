/*
    --- Day 15: Beverage Bandits ---
    Having perfected their hot chocolate, the Elves have a new problem: the Goblins that live in these caves will do anything to steal it. Looks like they're here for a fight.

    You scan the area, generating a map of the walls (#), open cavern (.), and starting position of every Goblin (G) and Elf (E) (your puzzle input).

    Combat proceeds in rounds; in each round, each unit that is still alive takes a turn, resolving all of its actions before the next unit's turn begins. On each unit's turn, it tries to move into range of an enemy (if it isn't already) and then attack (if it is in range).

    All units are very disciplined and always follow very strict combat rules. Units never move or attack diagonally, as doing so would be dishonorable. When multiple choices are equally valid, ties are broken in reading order: top-to-bottom, then left-to-right. For instance, the order in which units take their turns within a round is the reading order of their starting positions in that round, regardless of the type of unit or whether other units have moved after the round started. For example:

                    would take their
    These units:   turns in this order:
    #######           #######
    #.G.E.#           #.1.2.#
    #E.G.E#           #3.4.5#
    #.G.E.#           #.6.7.#
    #######           #######
    Each unit begins its turn by identifying all possible targets (enemy units). If no targets remain, combat ends.

    Then, the unit identifies all of the open squares (.) that are in range of each target; these are the squares which are adjacent (immediately up, down, left, or right) to any target and which aren't already occupied by a wall or another unit. Alternatively, the unit might already be in range of a target. If the unit is not already in range of a target, and there are no open squares which are in range of a target, the unit ends its turn.

    If the unit is already in range of a target, it does not move, but continues its turn with an attack. Otherwise, since it is not in range of a target, it moves.

    To move, the unit first considers the squares that are in range and determines which of those squares it could reach in the fewest steps. A step is a single movement to any adjacent (immediately up, down, left, or right) open (.) square. Units cannot move into walls or other units. The unit does this while considering the current positions of units and does not do any prediction about where units will be later. If the unit cannot reach (find an open path to) any of the squares that are in range, it ends its turn. If multiple squares are in range and tied for being reachable in the fewest steps, the square which is first in reading order is chosen. For example:

    Targets:      In range:     Reachable:    Nearest:      Chosen:
    #######       #######       #######       #######       #######
    #E..G.#       #E.?G?#       #E.@G.#       #E.!G.#       #E.+G.#
    #...#.#  -->  #.?.#?#  -->  #.@.#.#  -->  #.!.#.#  -->  #...#.#
    #.G.#G#       #?G?#G#       #@G@#G#       #!G.#G#       #.G.#G#
    #######       #######       #######       #######       #######
    In the above scenario, the Elf has three targets (the three Goblins):

    Each of the Goblins has open, adjacent squares which are in range (marked with a ? on the map).
    Of those squares, four are reachable (marked @); the other two (on the right) would require moving through a wall or unit to reach.
    Three of these reachable squares are nearest, requiring the fewest steps (only 2) to reach (marked !).
    Of those, the square which is first in reading order is chosen (+).
    The unit then takes a single step toward the chosen square along the shortest path to that square. If multiple steps would put the unit equally closer to its destination, the unit chooses the step which is first in reading order. (This requires knowing when there is more than one shortest path so that you can consider the first step of each such path.) For example:

    In range:     Nearest:      Chosen:       Distance:     Step:
    #######       #######       #######       #######       #######
    #.E...#       #.E...#       #.E...#       #4E212#       #..E..#
    #...?.#  -->  #...!.#  -->  #...+.#  -->  #32101#  -->  #.....#
    #..?G?#       #..!G.#       #...G.#       #432G2#       #...G.#
    #######       #######       #######       #######       #######
    The Elf sees three squares in range of a target (?), two of which are nearest (!), and so the first in reading order is chosen (+). Under "Distance", each open square is marked with its distance from the destination square; the two squares to which the Elf could move on this turn (down and to the right) are both equally good moves and would leave the Elf 2 steps from being in range of the Goblin. Because the step which is first in reading order is chosen, the Elf moves right one square.

    Here's a larger example of movement:

    Initially:
    #########
    #G..G..G#
    #.......#
    #.......#
    #G..E..G#
    #.......#
    #.......#
    #G..G..G#
    #########

    After 1 round:
    #########
    #.G...G.#
    #...G...#
    #...E..G#
    #.G.....#
    #.......#
    #G..G..G#
    #.......#
    #########

    After 2 rounds:
    #########
    #..G.G..#
    #...G...#
    #.G.E.G.#
    #.......#
    #G..G..G#
    #.......#
    #.......#
    #########

    After 3 rounds:
    #########
    #.......#
    #..GGG..#
    #..GEG..#
    #G..G...#
    #......G#
    #.......#
    #.......#
    #########
    Once the Goblins and Elf reach the positions above, they all are either in range of a target or cannot find any square in range of a target, and so none of the units can move until a unit dies.

    After moving (or if the unit began its turn in range of a target), the unit attacks.

    To attack, the unit first determines all of the targets that are in range of it by being immediately adjacent to it. If there are no such targets, the unit ends its turn. Otherwise, the adjacent target with the fewest hit points is selected; in a tie, the adjacent target with the fewest hit points which is first in reading order is selected.

    The unit deals damage equal to its attack power to the selected target, reducing its hit points by that amount. If this reduces its hit points to 0 or fewer, the selected target dies: its square becomes . and it takes no further turns.

    Each unit, either Goblin or Elf, has 3 attack power and starts with 200 hit points.

    For example, suppose the only Elf is about to attack:

        HP:            HP:
    G....  9       G....  9
    ..G..  4       ..G..  4
    ..EG.  2  -->  ..E..
    ..G..  2       ..G..  2
    ...G.  1       ...G.  1
    The "HP" column shows the hit points of the Goblin to the left in the corresponding row. The Elf is in range of three targets: the Goblin above it (with 4 hit points), the Goblin to its right (with 2 hit points), and the Goblin below it (also with 2 hit points). Because three targets are in range, the ones with the lowest hit points are selected: the two Goblins with 2 hit points each (one to the right of the Elf and one below the Elf). Of those, the Goblin first in reading order (the one to the right of the Elf) is selected. The selected Goblin's hit points (2) are reduced by the Elf's attack power (3), reducing its hit points to -1, killing it.

    After attacking, the unit's turn ends. Regardless of how the unit's turn ends, the next unit in the round takes its turn. If all units have taken turns in this round, the round ends, and a new round begins.

    The Elves look quite outnumbered. You need to determine the outcome of the battle: the number of full rounds that were completed (not counting the round in which combat ends) multiplied by the sum of the hit points of all remaining units at the moment combat ends. (Combat only ends when a unit finds no targets during its turn.)

    Below is an entire sample combat. Next to each map, each row's units' hit points are listed from left to right.

    Initially:
    #######
    #.G...#   G(200)
    #...EG#   E(200), G(200)
    #.#.#G#   G(200)
    #..G#E#   G(200), E(200)
    #.....#
    #######

    After 1 round:
    #######
    #..G..#   G(200)
    #...EG#   E(197), G(197)
    #.#G#G#   G(200), G(197)
    #...#E#   E(197)
    #.....#
    #######

    After 2 rounds:
    #######
    #...G.#   G(200)
    #..GEG#   G(200), E(188), G(194)
    #.#.#G#   G(194)
    #...#E#   E(194)
    #.....#
    #######

    Combat ensues; eventually, the top Elf dies:

    After 23 rounds:
    #######
    #...G.#   G(200)
    #..G.G#   G(200), G(131)
    #.#.#G#   G(131)
    #...#E#   E(131)
    #.....#
    #######

    After 24 rounds:
    #######
    #..G..#   G(200)
    #...G.#   G(131)
    #.#G#G#   G(200), G(128)
    #...#E#   E(128)
    #.....#
    #######

    After 25 rounds:
    #######
    #.G...#   G(200)
    #..G..#   G(131)
    #.#.#G#   G(125)
    #..G#E#   G(200), E(125)
    #.....#
    #######

    After 26 rounds:
    #######
    #G....#   G(200)
    #.G...#   G(131)
    #.#.#G#   G(122)
    #...#E#   E(122)
    #..G..#   G(200)
    #######

    After 27 rounds:
    #######
    #G....#   G(200)
    #.G...#   G(131)
    #.#.#G#   G(119)
    #...#E#   E(119)
    #...G.#   G(200)
    #######

    After 28 rounds:
    #######
    #G....#   G(200)
    #.G...#   G(131)
    #.#.#G#   G(116)
    #...#E#   E(113)
    #....G#   G(200)
    #######

    More combat ensues; eventually, the bottom Elf dies:

    After 47 rounds:
    #######
    #G....#   G(200)
    #.G...#   G(131)
    #.#.#G#   G(59)
    #...#.#
    #....G#   G(200)
    #######
    Before the 48th round can finish, the top-left Goblin finds that there are no targets remaining, and so combat ends. So, the number of full rounds that were completed is 47, and the sum of the hit points of all remaining units is 200+131+59+200 = 590. From these, the outcome of the battle is 47 * 590 = 27730.

    Here are a few example summarized combats:

    #######       #######
    #G..#E#       #...#E#   E(200)
    #E#E.E#       #E#...#   E(197)
    #G.##.#  -->  #.E##.#   E(185)
    #...#E#       #E..#E#   E(200), E(200)
    #...E.#       #.....#
    #######       #######

    Combat ends after 37 full rounds
    Elves win with 982 total hit points left
    Outcome: 37 * 982 = 36334
    #######       #######
    #E..EG#       #.E.E.#   E(164), E(197)
    #.#G.E#       #.#E..#   E(200)
    #E.##E#  -->  #E.##.#   E(98)
    #G..#.#       #.E.#.#   E(200)
    #..E#.#       #...#.#
    #######       #######

    Combat ends after 46 full rounds
    Elves win with 859 total hit points left
    Outcome: 46 * 859 = 39514
    #######       #######
    #E.G#.#       #G.G#.#   G(200), G(98)
    #.#G..#       #.#G..#   G(200)
    #G.#.G#  -->  #..#..#
    #G..#.#       #...#G#   G(95)
    #...E.#       #...G.#   G(200)
    #######       #######

    Combat ends after 35 full rounds
    Goblins win with 793 total hit points left
    Outcome: 35 * 793 = 27755
    #######       #######
    #.E...#       #.....#
    #.#..G#       #.#G..#   G(200)
    #.###.#  -->  #.###.#
    #E#G#G#       #.#.#.#
    #...#G#       #G.G#G#   G(98), G(38), G(200)
    #######       #######

    Combat ends after 54 full rounds
    Goblins win with 536 total hit points left
    Outcome: 54 * 536 = 28944
    #########       #########
    #G......#       #.G.....#   G(137)
    #.E.#...#       #G.G#...#   G(200), G(200)
    #..##..G#       #.G##...#   G(200)
    #...##..#  -->  #...##..#
    #...#...#       #.G.#...#   G(200)
    #.G...G.#       #.......#
    #.....G.#       #.......#
    #########       #########

    Combat ends after 20 full rounds
    Goblins win with 937 total hit points left
    Outcome: 20 * 937 = 18740
    What is the outcome of the combat described in your puzzle input?
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

struct BattleMap {
    tiles: BTreeMap<Point, Tile>,
}

impl BattleMap {
    fn from_string(input: &str) -> Self {
        let mut tiles = BTreeMap::new();
        let mut p = Point { x: 0, y: 0 };
        for line in input.trim().lines() {
            for c in line.chars() {
                tiles.insert(p, Tile::from_char(c));
                p.x += 1;
            }
            p.x = 0;
            p.y += 1;
        }

        Self { tiles }
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
                            if x <= 3 {
                                Tile::Empty // Dead goblin
                            } else {
                                Tile::Goblin(x - 3)
                            }
                        }
                        Some(&Tile::Elf(x)) => {
                            if x <= 3 {
                                Tile::Empty // Dead elf
                            } else {
                                Tile::Elf(x - 3)
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
}

impl fmt::Display for BattleMap {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.to_string(false))
    }
}

#[aoc(day15, part1)]
pub fn solve(input: &str) -> u32 {
    let mut battle_map = BattleMap::from_string(input);
    let score = battle_map.battle();
    println!("Score: {}", score);
    assert_eq!(score, 184206);
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
}
