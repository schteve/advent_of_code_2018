/*
    --- Day 24: Immune System Simulator 20XX ---
    After a weird buzzing noise, you appear back at the man's cottage. He seems relieved to see his friend, but quickly notices that the little reindeer caught some kind of cold while out exploring.

    The portly man explains that this reindeer's immune system isn't similar to regular reindeer immune systems:

    The immune system and the infection each have an army made up of several groups; each group consists of one or more identical units. The armies repeatedly fight until only one army has units remaining.

    Units within a group all have the same hit points (amount of damage a unit can take before it is destroyed), attack damage (the amount of damage each unit deals), an attack type, an initiative (higher initiative units attack first and win ties), and sometimes weaknesses or immunities. Here is an example group:

    18 units each with 729 hit points (weak to fire; immune to cold, slashing)
    with an attack that does 8 radiation damage at initiative 10
    Each group also has an effective power: the number of units in that group multiplied by their attack damage. The above group has an effective power of 18 * 8 = 144. Groups never have zero or negative units; instead, the group is removed from combat.

    Each fight consists of two phases: target selection and attacking.

    During the target selection phase, each group attempts to choose one target. In decreasing order of effective power, groups choose their targets; in a tie, the group with the higher initiative chooses first. The attacking group chooses to target the group in the enemy army to which it would deal the most damage (after accounting for weaknesses and immunities, but not accounting for whether the defending group has enough units to actually receive all of that damage).

    If an attacking group is considering two defending groups to which it would deal equal damage, it chooses to target the defending group with the largest effective power; if there is still a tie, it chooses the defending group with the highest initiative. If it cannot deal any defending groups damage, it does not choose a target. Defending groups can only be chosen as a target by one attacking group.

    At the end of the target selection phase, each group has selected zero or one groups to attack, and each group is being attacked by zero or one groups.

    During the attacking phase, each group deals damage to the target it selected, if any. Groups attack in decreasing order of initiative, regardless of whether they are part of the infection or the immune system. (If a group contains no units, it cannot attack.)

    The damage an attacking group deals to a defending group depends on the attacking group's attack type and the defending group's immunities and weaknesses. By default, an attacking group would deal damage equal to its effective power to the defending group. However, if the defending group is immune to the attacking group's attack type, the defending group instead takes no damage; if the defending group is weak to the attacking group's attack type, the defending group instead takes double damage.

    The defending group only loses whole units from damage; damage is always dealt in such a way that it kills the most units possible, and any remaining damage to a unit that does not immediately kill it is ignored. For example, if a defending group contains 10 units with 10 hit points each and receives 75 damage, it loses exactly 7 units and is left with 3 units at full health.

    After the fight is over, if both armies still contain units, a new fight begins; combat only ends once one army has lost all of its units.

    For example, consider the following armies:

    Immune System:
    17 units each with 5390 hit points (weak to radiation, bludgeoning) with
    an attack that does 4507 fire damage at initiative 2
    989 units each with 1274 hit points (immune to fire; weak to bludgeoning,
    slashing) with an attack that does 25 slashing damage at initiative 3

    Infection:
    801 units each with 4706 hit points (weak to radiation) with an attack
    that does 116 bludgeoning damage at initiative 1
    4485 units each with 2961 hit points (immune to radiation; weak to fire,
    cold) with an attack that does 12 slashing damage at initiative 4
    If these armies were to enter combat, the following fights, including details during the target selection and attacking phases, would take place:

    Immune System:
    Group 1 contains 17 units
    Group 2 contains 989 units
    Infection:
    Group 1 contains 801 units
    Group 2 contains 4485 units

    Infection group 1 would deal defending group 1 185832 damage
    Infection group 1 would deal defending group 2 185832 damage
    Infection group 2 would deal defending group 2 107640 damage
    Immune System group 1 would deal defending group 1 76619 damage
    Immune System group 1 would deal defending group 2 153238 damage
    Immune System group 2 would deal defending group 1 24725 damage

    Infection group 2 attacks defending group 2, killing 84 units
    Immune System group 2 attacks defending group 1, killing 4 units
    Immune System group 1 attacks defending group 2, killing 51 units
    Infection group 1 attacks defending group 1, killing 17 units
    Immune System:
    Group 2 contains 905 units
    Infection:
    Group 1 contains 797 units
    Group 2 contains 4434 units

    Infection group 1 would deal defending group 2 184904 damage
    Immune System group 2 would deal defending group 1 22625 damage
    Immune System group 2 would deal defending group 2 22625 damage

    Immune System group 2 attacks defending group 1, killing 4 units
    Infection group 1 attacks defending group 2, killing 144 units
    Immune System:
    Group 2 contains 761 units
    Infection:
    Group 1 contains 793 units
    Group 2 contains 4434 units

    Infection group 1 would deal defending group 2 183976 damage
    Immune System group 2 would deal defending group 1 19025 damage
    Immune System group 2 would deal defending group 2 19025 damage

    Immune System group 2 attacks defending group 1, killing 4 units
    Infection group 1 attacks defending group 2, killing 143 units
    Immune System:
    Group 2 contains 618 units
    Infection:
    Group 1 contains 789 units
    Group 2 contains 4434 units

    Infection group 1 would deal defending group 2 183048 damage
    Immune System group 2 would deal defending group 1 15450 damage
    Immune System group 2 would deal defending group 2 15450 damage

    Immune System group 2 attacks defending group 1, killing 3 units
    Infection group 1 attacks defending group 2, killing 143 units
    Immune System:
    Group 2 contains 475 units
    Infection:
    Group 1 contains 786 units
    Group 2 contains 4434 units

    Infection group 1 would deal defending group 2 182352 damage
    Immune System group 2 would deal defending group 1 11875 damage
    Immune System group 2 would deal defending group 2 11875 damage

    Immune System group 2 attacks defending group 1, killing 2 units
    Infection group 1 attacks defending group 2, killing 142 units
    Immune System:
    Group 2 contains 333 units
    Infection:
    Group 1 contains 784 units
    Group 2 contains 4434 units

    Infection group 1 would deal defending group 2 181888 damage
    Immune System group 2 would deal defending group 1 8325 damage
    Immune System group 2 would deal defending group 2 8325 damage

    Immune System group 2 attacks defending group 1, killing 1 unit
    Infection group 1 attacks defending group 2, killing 142 units
    Immune System:
    Group 2 contains 191 units
    Infection:
    Group 1 contains 783 units
    Group 2 contains 4434 units

    Infection group 1 would deal defending group 2 181656 damage
    Immune System group 2 would deal defending group 1 4775 damage
    Immune System group 2 would deal defending group 2 4775 damage

    Immune System group 2 attacks defending group 1, killing 1 unit
    Infection group 1 attacks defending group 2, killing 142 units
    Immune System:
    Group 2 contains 49 units
    Infection:
    Group 1 contains 782 units
    Group 2 contains 4434 units

    Infection group 1 would deal defending group 2 181424 damage
    Immune System group 2 would deal defending group 1 1225 damage
    Immune System group 2 would deal defending group 2 1225 damage

    Immune System group 2 attacks defending group 1, killing 0 units
    Infection group 1 attacks defending group 2, killing 49 units
    Immune System:
    No groups remain.
    Infection:
    Group 1 contains 782 units
    Group 2 contains 4434 units
    In the example above, the winning army ends up with 782 + 4434 = 5216 units.

    You scan the reindeer's condition (your puzzle input); the white-bearded man looks nervous. As it stands now, how many units would the winning army have?
*/

use lazy_static::lazy_static;
use regex::Regex;
use std::cmp;

#[derive(Clone, Debug, PartialEq)]
struct Group {
    units: u32,
    hp: u32,
    atk_dmg: u32,
    atk_type: String,
    initiative: u32,
    weaknesses: Vec<String>,
    immunities: Vec<String>,
}

impl Group {
    fn from_string(input: &str) -> Option<Self> {
        lazy_static! {
            static ref RE1: Regex = Regex::new(r"^(\d+) units each with (\d+) hit points (?:\(.*?\) )?with an attack that does (\d+) (\w+) damage at initiative (\d+)").unwrap();
            static ref RE2: Regex = Regex::new(r"weak to (.*?)(;|\))").unwrap();
            static ref RE3: Regex = Regex::new(r"immune to (.*?)(;|\))").unwrap();
        }
        let caps_raw = RE1.captures(input.trim());
        if caps_raw.is_none() == true {
            return None;
        }

        let caps = caps_raw.unwrap();
        let units = caps[1].parse::<u32>().unwrap();
        let hp = caps[2].parse::<u32>().unwrap();
        let atk_dmg = caps[3].parse::<u32>().unwrap();
        let atk_type = caps[4].to_owned();
        let initiative = caps[5].parse::<u32>().unwrap();

        let caps_raw = RE2.captures(input);
        let weaknesses = if let Some(caps) = caps_raw {
            caps[1].split(", ").map(|s| s.to_owned()).collect()
        } else {
            Vec::new()
        };

        let caps_raw = RE3.captures(input);
        let immunities = if let Some(caps) = caps_raw {
            caps[1].split(", ").map(|s| s.to_owned()).collect()
        } else {
            Vec::new()
        };

        Some(Self {
            units,
            hp,
            atk_dmg,
            atk_type,
            initiative,
            weaknesses,
            immunities,
        })
    }

    fn effective_power(&self) -> u32 {
        self.units * self.atk_dmg
    }

    fn damage_to_enemy(&self, enemy: &Self) -> u32 {
        if enemy.immunities.contains(&self.atk_type) == true {
            0
        } else if enemy.weaknesses.contains(&self.atk_type) == true {
            self.effective_power() * 2
        } else {
            self.effective_power()
        }
    }

    fn apply_damage(&self, enemy: &mut Self) {
        let dmg = self.damage_to_enemy(enemy);
        let units_lost = dmg / enemy.hp;
        enemy.units -= cmp::min(enemy.units, units_lost);
    }
}

struct System {
    immune: Vec<Group>,
    infection: Vec<Group>,
}

impl System {
    fn from_string(input: &str) -> Self {
        let mut immune: Vec<Group> = Vec::new();
        let mut infection: Vec<Group> = Vec::new();
        let mut is_immune = true;
        for line in input.lines() {
            if line.starts_with("Immune System:") == true {
                is_immune = true;
            } else if line.starts_with("Infection:") == true {
                is_immune = false;
            } else if let Some(group) = Group::from_string(line) {
                if is_immune == true {
                    immune.push(group);
                } else {
                    infection.push(group);
                }
            }
        }

        Self { immune, infection }
    }

    fn select_targets(friendly: &[Group], enemy: &[Group]) -> Vec<Option<usize>> {
        // First create a list showing the index order of friendly groups based on power, with initiative as a tie breaker
        let mut friendly_sorted: Vec<(usize, Group)> =
            friendly.iter().cloned().enumerate().collect();
        friendly_sorted.sort_unstable_by(|a, b| {
            (b.1.effective_power(), b.1.initiative).cmp(&(a.1.effective_power(), a.1.initiative))
        });

        // Then select a target (if possible) for each group based on the most possible damage done, with effective power as a tie breaker
        let mut targets_by_priority: Vec<(usize, Option<usize>)> = Vec::new();
        for (f_idx, f) in friendly_sorted {
            let target_idx = enemy
                .iter()
                .enumerate()
                .filter(|&(i, e)| {
                    targets_by_priority.iter().any(|(_idx, t)| t == &Some(i)) == false
                        && e.units > 0
                }) // Skip enemies that have already been targeted, or that have no units remaining
                .max_by(|(_ai, ae), (_bi, be)| {
                    (f.damage_to_enemy(ae), ae.effective_power(), ae.initiative).cmp(&(
                        f.damage_to_enemy(be),
                        be.effective_power(),
                        be.initiative,
                    ))
                })
                .map(|(i, _e)| i);

            if let Some(idx) = target_idx {
                if f.damage_to_enemy(&enemy[idx]) == 0 {
                    // If it cannot deal any damage, do not choose a target
                    targets_by_priority.push((f_idx, None));
                } else {
                    targets_by_priority.push((f_idx, target_idx));
                }
            } else {
                targets_by_priority.push((f_idx, target_idx));
            }
        }

        // Last reorder the targets, since they were written in priority order but must be returned in the same order as the friendly army groups
        targets_by_priority.sort_unstable_by_key(|(idx, _target_idx)| *idx);
        targets_by_priority
            .into_iter()
            .map(|(_idx, target_idx)| target_idx)
            .collect()
    }

    fn fight(&mut self) {
        // Target selection phase.
        let immune_targets: Vec<Option<usize>> =
            System::select_targets(&self.immune, &self.infection);
        let infection_targets: Vec<Option<usize>> =
            System::select_targets(&self.infection, &self.immune);

        // Attacking phase. Create a master list of all groups (index and initiative only), their target, and an indicator of which army
        // it is (true = immune is attacking and infection is defending).
        let mut all_attackers: Vec<(usize, u32, Option<usize>, bool)> = Vec::new();
        all_attackers.extend(
            self.immune
                .iter()
                .enumerate()
                .zip(immune_targets)
                .map(|((g_idx, g), t)| (g_idx, g.initiative, t, true)),
        );
        all_attackers.extend(
            self.infection
                .iter()
                .enumerate()
                .zip(infection_targets)
                .map(|((g_idx, g), t)| (g_idx, g.initiative, t, false)),
        );
        all_attackers.sort_unstable_by_key(|(_g_idx, g_init, _t, _a)| *g_init); // Sort master list by initiative, highest initiative last

        // Iterate from highest initiative to lowest
        for (group_idx, _group_init, target, army) in all_attackers.into_iter().rev() {
            if let Some(target_idx) = target {
                let (attacker, defender) = if army == true {
                    // Immune is attacking, infection is defending
                    if self.immune[group_idx].units == 0 {
                        // Target has no units left, skip this attack
                        continue;
                    }
                    (&self.immune, &mut self.infection)
                } else {
                    // Infection is attacking, immune is defending
                    if self.infection[group_idx].units == 0 {
                        // Target has no units left, skip this attack
                        continue;
                    }
                    (&self.infection, &mut self.immune)
                };
                attacker[group_idx].apply_damage(&mut defender[target_idx]);
            }
        }
    }

    fn immune_count(&self) -> u32 {
        self.immune.iter().map(|g| g.units).sum()
    }

    fn infection_count(&self) -> u32 {
        self.infection.iter().map(|g| g.units).sum()
    }

    fn battle(&mut self) -> u32 {
        loop {
            let immune_units = self.immune_count();
            let infection_units = self.infection_count();
            if immune_units > 0 && infection_units > 0 {
                self.fight();
            } else {
                // Fighting has concluded, return the final score
                return immune_units + infection_units;
            }
        }
    }
}

#[aoc(day24, part1)]
pub fn solve(input: &str) -> u32 {
    let mut system = System::from_string(input);
    let units = system.battle();
    println!("Units remaining: {}", units);
    assert_eq!(units, 18717);
    units
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_system_from_string() {
        let input = "\
Immune System:
17 units each with 5390 hit points (weak to radiation, bludgeoning) with an attack that does 4507 fire damage at initiative 2
989 units each with 1274 hit points (immune to fire; weak to bludgeoning, slashing) with an attack that does 25 slashing damage at initiative 3

Infection:
801 units each with 4706 hit points (weak to radiation) with an attack that does 116 bludgeoning damage at initiative 1
4485 units each with 2961 hit points (immune to radiation; weak to fire, cold) with an attack that does 12 slashing damage at initiative 4";
        let system = System::from_string(input);
        assert_eq!(
            system.immune[0],
            Group {
                units: 17,
                hp: 5390,
                atk_dmg: 4507,
                atk_type: "fire".into(),
                initiative: 2,
                weaknesses: vec!["radiation".into(), "bludgeoning".into()],
                immunities: vec![],
            }
        );
        assert_eq!(
            system.immune[1],
            Group {
                units: 989,
                hp: 1274,
                atk_dmg: 25,
                atk_type: "slashing".into(),
                initiative: 3,
                weaknesses: vec!["bludgeoning".into(), "slashing".into()],
                immunities: vec!["fire".into()],
            }
        );
        assert_eq!(
            system.infection[0],
            Group {
                units: 801,
                hp: 4706,
                atk_dmg: 116,
                atk_type: "bludgeoning".into(),
                initiative: 1,
                weaknesses: vec!["radiation".into()],
                immunities: vec![],
            }
        );
        assert_eq!(
            system.infection[1],
            Group {
                units: 4485,
                hp: 2961,
                atk_dmg: 12,
                atk_type: "slashing".into(),
                initiative: 4,
                weaknesses: vec!["fire".into(), "cold".into()],
                immunities: vec!["radiation".into()],
            }
        );
    }

    #[test]
    fn test_effective_power() {
        let group = Group {
            units: 18,
            hp: 729,
            atk_dmg: 8,
            atk_type: "radiation".into(),
            initiative: 10,
            weaknesses: vec!["fire".into()],
            immunities: vec!["cold".into(), "slashing".into()],
        };
        assert_eq!(group.effective_power(), 144);
    }

    #[test]
    fn test_damage_to_enemy() {
        let input = "\
Immune System:
17 units each with 5390 hit points (weak to radiation, bludgeoning) with an attack that does 4507 fire damage at initiative 2
989 units each with 1274 hit points (immune to fire; weak to bludgeoning, slashing) with an attack that does 25 slashing damage at initiative 3

Infection:
801 units each with 4706 hit points (weak to radiation) with an attack that does 116 bludgeoning damage at initiative 1
4485 units each with 2961 hit points (immune to radiation; weak to fire, cold) with an attack that does 12 slashing damage at initiative 4";
        let system = System::from_string(input);
        assert_eq!(
            system.infection[0].damage_to_enemy(&system.immune[0]),
            185832
        );
        assert_eq!(
            system.infection[0].damage_to_enemy(&system.immune[1]),
            185832
        );
        assert_eq!(
            system.infection[1].damage_to_enemy(&system.immune[1]),
            107640
        );
        assert_eq!(
            system.immune[0].damage_to_enemy(&system.infection[0]),
            76619
        );
        assert_eq!(
            system.immune[0].damage_to_enemy(&system.infection[1]),
            153238
        );
        assert_eq!(
            system.immune[1].damage_to_enemy(&system.infection[0]),
            24725
        );
    }

    #[test]
    fn test_select_targets() {
        let input = "\
Immune System:
17 units each with 5390 hit points (weak to radiation, bludgeoning) with an attack that does 4507 fire damage at initiative 2
989 units each with 1274 hit points (immune to fire; weak to bludgeoning, slashing) with an attack that does 25 slashing damage at initiative 3

Infection:
801 units each with 4706 hit points (weak to radiation) with an attack that does 116 bludgeoning damage at initiative 1
4485 units each with 2961 hit points (immune to radiation; weak to fire, cold) with an attack that does 12 slashing damage at initiative 4";
        let system = System::from_string(input);
        let immune_targets = System::select_targets(&system.immune, &system.infection);
        assert_eq!(immune_targets, vec![Some(1), Some(0)]);
        let infection_targets = System::select_targets(&system.infection, &system.immune);
        assert_eq!(infection_targets, vec![Some(0), Some(1)]);
    }

    #[test]
    fn test_fight() {
        fn get_units_list(army: &[Group]) -> Vec<u32> {
            army.iter().map(|g| g.units).collect::<Vec<u32>>()
        }

        let input = "\
Immune System:
17 units each with 5390 hit points (weak to radiation, bludgeoning) with an attack that does 4507 fire damage at initiative 2
989 units each with 1274 hit points (immune to fire; weak to bludgeoning, slashing) with an attack that does 25 slashing damage at initiative 3

Infection:
801 units each with 4706 hit points (weak to radiation) with an attack that does 116 bludgeoning damage at initiative 1
4485 units each with 2961 hit points (immune to radiation; weak to fire, cold) with an attack that does 12 slashing damage at initiative 4";
        let mut system = System::from_string(input);
        assert_eq!(get_units_list(&system.immune), vec![17, 989]);
        assert_eq!(get_units_list(&system.infection), vec![801, 4485]);

        system.fight();
        assert_eq!(get_units_list(&system.immune), vec![0, 905]);
        assert_eq!(get_units_list(&system.infection), vec![797, 4434]);

        system.fight();
        assert_eq!(get_units_list(&system.immune), vec![0, 761]);
        assert_eq!(get_units_list(&system.infection), vec![793, 4434]);

        system.fight();
        assert_eq!(get_units_list(&system.immune), vec![0, 618]);
        assert_eq!(get_units_list(&system.infection), vec![789, 4434]);

        system.fight();
        assert_eq!(get_units_list(&system.immune), vec![0, 475]);
        assert_eq!(get_units_list(&system.infection), vec![786, 4434]);

        system.fight();
        assert_eq!(get_units_list(&system.immune), vec![0, 333]);
        assert_eq!(get_units_list(&system.infection), vec![784, 4434]);

        system.fight();
        assert_eq!(get_units_list(&system.immune), vec![0, 191]);
        assert_eq!(get_units_list(&system.infection), vec![783, 4434]);

        system.fight();
        assert_eq!(get_units_list(&system.immune), vec![0, 49]);
        assert_eq!(get_units_list(&system.infection), vec![782, 4434]);

        system.fight();
        assert_eq!(get_units_list(&system.immune), vec![0, 0]);
        assert_eq!(get_units_list(&system.infection), vec![782, 4434]);
    }

    #[test]
    fn test_battle() {
        let input = "\
Immune System:
17 units each with 5390 hit points (weak to radiation, bludgeoning) with an attack that does 4507 fire damage at initiative 2
989 units each with 1274 hit points (immune to fire; weak to bludgeoning, slashing) with an attack that does 25 slashing damage at initiative 3

Infection:
801 units each with 4706 hit points (weak to radiation) with an attack that does 116 bludgeoning damage at initiative 1
4485 units each with 2961 hit points (immune to radiation; weak to fire, cold) with an attack that does 12 slashing damage at initiative 4";
        let mut system = System::from_string(input);
        let result = system.battle();
        assert_eq!(result, 5216);
    }
}
