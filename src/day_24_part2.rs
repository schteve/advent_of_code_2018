/*
    --- Part Two ---
    Things aren't looking good for the reindeer. The man asks whether more milk and cookies would help you think.

    If only you could give the reindeer's immune system a boost, you might be able to change the outcome of the combat.

    A boost is an integer increase in immune system units' attack damage. For example, if you were to boost the above example's immune system's units by 1570, the armies would instead look like this:

    Immune System:
    17 units each with 5390 hit points (weak to radiation, bludgeoning) with
    an attack that does 6077 fire damage at initiative 2
    989 units each with 1274 hit points (immune to fire; weak to bludgeoning,
    slashing) with an attack that does 1595 slashing damage at initiative 3

    Infection:
    801 units each with 4706 hit points (weak to radiation) with an attack
    that does 116 bludgeoning damage at initiative 1
    4485 units each with 2961 hit points (immune to radiation; weak to fire,
    cold) with an attack that does 12 slashing damage at initiative 4
    With this boost, the combat proceeds differently:

    Immune System:
    Group 2 contains 989 units
    Group 1 contains 17 units
    Infection:
    Group 1 contains 801 units
    Group 2 contains 4485 units

    Infection group 1 would deal defending group 2 185832 damage
    Infection group 1 would deal defending group 1 185832 damage
    Infection group 2 would deal defending group 1 53820 damage
    Immune System group 2 would deal defending group 1 1577455 damage
    Immune System group 2 would deal defending group 2 1577455 damage
    Immune System group 1 would deal defending group 2 206618 damage

    Infection group 2 attacks defending group 1, killing 9 units
    Immune System group 2 attacks defending group 1, killing 335 units
    Immune System group 1 attacks defending group 2, killing 32 units
    Infection group 1 attacks defending group 2, killing 84 units
    Immune System:
    Group 2 contains 905 units
    Group 1 contains 8 units
    Infection:
    Group 1 contains 466 units
    Group 2 contains 4453 units

    Infection group 1 would deal defending group 2 108112 damage
    Infection group 1 would deal defending group 1 108112 damage
    Infection group 2 would deal defending group 1 53436 damage
    Immune System group 2 would deal defending group 1 1443475 damage
    Immune System group 2 would deal defending group 2 1443475 damage
    Immune System group 1 would deal defending group 2 97232 damage

    Infection group 2 attacks defending group 1, killing 8 units
    Immune System group 2 attacks defending group 1, killing 306 units
    Infection group 1 attacks defending group 2, killing 29 units
    Immune System:
    Group 2 contains 876 units
    Infection:
    Group 2 contains 4453 units
    Group 1 contains 160 units

    Infection group 2 would deal defending group 2 106872 damage
    Immune System group 2 would deal defending group 2 1397220 damage
    Immune System group 2 would deal defending group 1 1397220 damage

    Infection group 2 attacks defending group 2, killing 83 units
    Immune System group 2 attacks defending group 2, killing 427 units
    After a few fights...

    Immune System:
    Group 2 contains 64 units
    Infection:
    Group 2 contains 214 units
    Group 1 contains 19 units

    Infection group 2 would deal defending group 2 5136 damage
    Immune System group 2 would deal defending group 2 102080 damage
    Immune System group 2 would deal defending group 1 102080 damage

    Infection group 2 attacks defending group 2, killing 4 units
    Immune System group 2 attacks defending group 2, killing 32 units
    Immune System:
    Group 2 contains 60 units
    Infection:
    Group 1 contains 19 units
    Group 2 contains 182 units

    Infection group 1 would deal defending group 2 4408 damage
    Immune System group 2 would deal defending group 1 95700 damage
    Immune System group 2 would deal defending group 2 95700 damage

    Immune System group 2 attacks defending group 1, killing 19 units
    Immune System:
    Group 2 contains 60 units
    Infection:
    Group 2 contains 182 units

    Infection group 2 would deal defending group 2 4368 damage
    Immune System group 2 would deal defending group 2 95700 damage

    Infection group 2 attacks defending group 2, killing 3 units
    Immune System group 2 attacks defending group 2, killing 30 units
    After a few more fights...

    Immune System:
    Group 2 contains 51 units
    Infection:
    Group 2 contains 40 units

    Infection group 2 would deal defending group 2 960 damage
    Immune System group 2 would deal defending group 2 81345 damage

    Infection group 2 attacks defending group 2, killing 0 units
    Immune System group 2 attacks defending group 2, killing 27 units
    Immune System:
    Group 2 contains 51 units
    Infection:
    Group 2 contains 13 units

    Infection group 2 would deal defending group 2 312 damage
    Immune System group 2 would deal defending group 2 81345 damage

    Infection group 2 attacks defending group 2, killing 0 units
    Immune System group 2 attacks defending group 2, killing 13 units
    Immune System:
    Group 2 contains 51 units
    Infection:
    No groups remain.
    This boost would allow the immune system's armies to win! It would be left with 51 units.

    You don't even know how you could boost the reindeer's immune system or what effect it might have, so you need to be cautious and find the smallest boost that would allow the immune system to win.

    How many units does the immune system have left after getting the smallest boost it needs to win?
*/

use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{alpha1, char, digit1, multispace0},
    combinator::{map, map_res, opt, success},
    multi::{many1, separated_list1},
    sequence::{delimited, pair, preceded, terminated, tuple},
    IResult,
};
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
    fn from_string(input: &str) -> Self {
        Self::parser(input).unwrap().1
    }

    fn parser(input: &str) -> IResult<&str, Self> {
        let (input, (units, hp, modifiers, atk_dmg, atk_type, initiative)) = tuple((
            delimited(
                multispace0,
                map_res(digit1, |x: &str| x.parse::<u32>()),
                tag(" units each with "),
            ),
            terminated(
                map_res(digit1, |x: &str| x.parse::<u32>()),
                tag(" hit points"),
            ),
            opt(delimited(
                tag(" ("),
                separated_list1(
                    tag("; "),
                    alt((
                        pair(
                            success(false),
                            preceded(tag("immune to "), separated_list1(tag(", "), alpha1)),
                        ),
                        pair(
                            success(true),
                            preceded(tag("weak to "), separated_list1(tag(", "), alpha1)),
                        ),
                    )),
                ),
                char(')'),
            )),
            preceded(
                tag(" with an attack that does "),
                map_res(digit1, |x: &str| x.parse::<u32>()),
            ),
            preceded(char(' '), map(alpha1, |x: &str| x.to_owned())),
            preceded(
                tag(" damage at initiative "),
                map_res(digit1, |x: &str| x.parse::<u32>()),
            ),
        ))(input)?;

        let mut weaknesses: Vec<String> = Vec::new();
        let mut immunities: Vec<String> = Vec::new();
        if let Some(m) = modifiers {
            for (is_weakness, values) in m {
                let values_iter = values.iter().map(|s| s.to_string());
                if is_weakness == true {
                    weaknesses.extend(values_iter);
                } else {
                    immunities.extend(values_iter);
                }
            }
        }

        Ok((
            input,
            Self {
                units,
                hp,
                atk_dmg,
                atk_type,
                initiative,
                weaknesses,
                immunities,
            },
        ))
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

#[derive(Clone)]
struct System {
    immune: Vec<Group>,
    infection: Vec<Group>,
}

impl System {
    fn from_string(input: &str) -> Self {
        Self::parser(input).unwrap().1
    }

    fn parser(input: &str) -> IResult<&str, Self> {
        let (input, (immune, infection)) = pair(
            preceded(
                pair(multispace0, tag("Immune System:")),
                many1(Group::parser),
            ),
            preceded(pair(multispace0, tag("Infection:")), many1(Group::parser)),
        )(input)?;

        Ok((input, Self { immune, infection }))
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
                let immune_units_after = self.immune_count();
                let infection_units_after = self.infection_count();
                if immune_units == immune_units_after && infection_units == infection_units_after {
                    // Stalemate! Count this as a loss by forcing the immune system to 0.
                    self.immune.iter_mut().for_each(|g| g.units = 0);
                    return 0;
                }
            } else {
                // Fighting has concluded, return the final score
                return immune_units + infection_units;
            }
        }
    }

    fn boost(&mut self, boost: u32) {
        self.immune.iter_mut().for_each(|g| g.atk_dmg += boost);
    }

    fn battle_boost(&self) -> u32 {
        let mut boost_value = 1;
        let mut bounds = (0, None);

        loop {
            let mut system = self.clone();
            system.boost(boost_value);
            system.battle();

            if system.immune_count() > 0 {
                // Immune system won! But this might not be the lowest boost value.
                bounds = (bounds.0, Some(boost_value));
            } else {
                bounds = (boost_value, bounds.1);
            }

            if let Some(upper) = bounds.1 {
                if upper == bounds.0 + 1 {
                    // Re-test the upper bound since the latest test may have been the lower bound
                    let mut system = self.clone();
                    system.boost(upper);
                    system.battle();
                    return system.immune_count() + system.infection_count();
                } else {
                    boost_value = (upper + bounds.0) / 2;
                }
            } else {
                boost_value *= 2;
            }
        }
    }
}

#[aoc(day24, part2)]
pub fn solve(input: &str) -> u32 {
    let system = System::from_string(input);
    let units = system.battle_boost();
    println!("Units remaining: {}", units);
    assert_eq!(units, 5252);
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

    #[test]
    fn test_battle_boost() {
        let input = "\
Immune System:
17 units each with 5390 hit points (weak to radiation, bludgeoning) with an attack that does 4507 fire damage at initiative 2
989 units each with 1274 hit points (immune to fire; weak to bludgeoning, slashing) with an attack that does 25 slashing damage at initiative 3

Infection:
801 units each with 4706 hit points (weak to radiation) with an attack that does 116 bludgeoning damage at initiative 1
4485 units each with 2961 hit points (immune to radiation; weak to fire, cold) with an attack that does 12 slashing damage at initiative 4";
        let system = System::from_string(input);

        // Check that the example boost gives the expected result
        let mut boosted = system.clone();
        boosted.boost(1570);
        boosted.battle();
        assert_eq!(boosted.immune_count(), 51);

        // Check that searching for the boost gives the same result as the example
        let result = system.battle_boost();
        assert_eq!(result, 51);
    }
}
