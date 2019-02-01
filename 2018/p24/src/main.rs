use lazy_static::lazy_static;
use regex::Regex;
use std::collections::hash_map::HashMap;
use std::collections::hash_set::HashSet;
use std::env;
use std::error::Error;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

fn a() -> Result<(), Box<dyn Error>> {
    let mut reader = BufReader::new(File::open("24.input")?);
    let mut contents = String::new();
    reader.read_to_string(&mut contents)?;
    let lines: Vec<_> = contents.lines().collect();
    let mut immune_groups = vec![];
    let mut infection_groups = vec![];
    for &line in lines[1..11].iter() {
        immune_groups.push(Group::parse(line, Army::Immune));
    }
    for &line in lines[13..].iter() {
        infection_groups.push(Group::parse(line, Army::Infection));
    }
    println!(
        "{}",
        simulate(&immune_groups, &infection_groups, 0).unwrap().1
    );
    Ok(())
}

fn b() -> Result<(), Box<dyn Error>> {
    let mut reader = BufReader::new(File::open("24.input")?);
    let mut contents = String::new();
    reader.read_to_string(&mut contents)?;
    let lines: Vec<_> = contents.lines().collect();
    let mut immune_groups = vec![];
    let mut infection_groups = vec![];
    for &line in lines[1..11].iter() {
        immune_groups.push(Group::parse(line, Army::Immune));
    }
    for &line in lines[13..].iter() {
        infection_groups.push(Group::parse(line, Army::Infection));
    }
    for atk_boost in 1.. {
        println!("Trying {}", atk_boost);
        //println!(
        //    "{:?}",
        //    simulate(&immune_groups, &infection_groups, atk_boost)
        //);
        if let Some((Army::Immune, n)) = simulate(&immune_groups, &infection_groups, atk_boost) {
            println!("{}", n);
            break;
        }
    }
    Ok(())
}

fn simulate(
    immune_groups: &Vec<Group>,
    infection_groups: &Vec<Group>,
    atk_boost: isize,
) -> Option<(Army, isize)> {
    let mut immune_groups = Vec::clone(immune_groups);
    let mut infection_groups = Vec::clone(infection_groups);
    for group in immune_groups.iter_mut() {
        group.dmg += atk_boost;
    }
    while !immune_groups.is_empty() && !infection_groups.is_empty() {
        let mut all_groups = vec![];
        let mut targeted = HashSet::new();
        let mut targets = HashMap::new();
        all_groups.extend(immune_groups.iter_mut());
        all_groups.extend(infection_groups.iter_mut());
        all_groups.sort_by_key(|g| (-g.units * g.dmg, -g.initiative));
        // Target selection
        for (group_idx, group) in all_groups.iter().enumerate() {
            let mut optimal_target = None;
            let mut optimal_target_idx = 0;
            let mut optimal_dmg = 0;
            let mut optimal_effective_power = 0;
            let mut optimal_initiative = 0;
            for (target_idx, target) in all_groups.iter().enumerate() {
                if (target.army == group.army) || targeted.contains(&target_idx) {
                    continue;
                }
                let dmg = calc_dmg(&group, &target);
                let target_effective_power = target.units * target.dmg;
                let target_initiative = target.initiative;
                if (optimal_target.is_none() && dmg != 0)
                    || (optimal_target.is_some()
                        && (dmg > optimal_dmg
                            || (dmg == optimal_dmg
                                && target_effective_power > optimal_effective_power)
                            || (dmg == optimal_dmg
                                && target_effective_power == optimal_effective_power
                                && target_initiative > optimal_initiative)))
                {
                    optimal_target = Some(target);
                    optimal_target_idx = target_idx;
                    optimal_dmg = dmg;
                    optimal_effective_power = target_effective_power;
                    optimal_initiative = target_initiative;
                }
            }
            if optimal_target.is_some() {
                targeted.insert(optimal_target_idx);
                targets.insert(group_idx, optimal_target_idx);
            }
        }
        // Attack phase
        let mut attack_order: Vec<_> = (0..all_groups.len()).collect();
        attack_order.sort_by_key(|&x| -all_groups[x].initiative);
        let mut killed = 0;
        for group_idx in attack_order {
            let group = &all_groups[group_idx];
            // Eliminated groups cannot attack; filter for groups that have targets
            if group.units <= 0 || !targets.contains_key(&group_idx) {
                continue;
            }
            let target_idx = targets[&group_idx];
            let target = &all_groups[target_idx];
            let dmg = calc_dmg(&group, target);
            let units_killed = dmg / target.hp;
            all_groups[target_idx].units -= units_killed;
            killed += units_killed;
        }
        // Tie if no units are killed
        if killed == 0 {
            return None;
        }
        // Reap eliminated groups
        infection_groups = infection_groups
            .into_iter()
            .filter(|g| g.units > 0)
            .collect();
        immune_groups = immune_groups
            .into_iter()
            .filter(|g| g.units > 0)
            .collect();
    }
    if !infection_groups.is_empty() {
        Some((Army::Infection, infection_groups.iter().map(|g| g.units).sum()))
    } else {
        Some((Army::Immune, immune_groups.iter().map(|g| g.units).sum()))
    }
}

fn calc_dmg(attacker: &Group, target: &Group) -> isize {
    let mut dmg = attacker.units * attacker.dmg;
    if target.immunities.contains(&attacker.atk_type) {
        dmg = 0;
    } else if target.weaknesses.contains(&attacker.atk_type) {
        dmg *= 2;
    }
    dmg
}

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
struct Group {
    units: isize,
    hp: isize,
    immunities: Vec<String>,
    weaknesses: Vec<String>,
    dmg: isize,
    atk_type: String,
    initiative: isize,
    army: Army,
}

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
enum Army {
    Infection,
    Immune,
}

impl Group {
    fn parse(desc: &str, army: Army) -> Group {
        lazy_static! {
            static ref GROUP_REGEX: Regex = Regex::new(r"(?P<units>.*) units each with (?P<hp>.*) hit points (?P<typing>\(.*\) )?with an attack that does (?P<dmg>.*) (?P<atk_type>.*) damage at initiative (?P<initiative>.*)").unwrap();
        }
        let cap = &GROUP_REGEX.captures(desc).unwrap();
        let mut immunities = vec![];
        let mut weaknesses = vec![];
        if let Some(typing) = cap.name("typing") {
            let s = typing.as_str();
            // Strip leading and trailing parentheses
            let s = &s[1..s.len() - 2];
            let parts = s.split("; ");
            for part in parts {
                if part.contains("weak to") {
                    for weakness in part["weak to ".len()..].split(", ") {
                        weaknesses.push(String::from(weakness));
                    }
                } else if part.contains("immune to") {
                    for immunity in part["immune to ".len()..].split(", ") {
                        immunities.push(String::from(immunity));
                    }
                }
            }
        }
        Group {
            units: parse_isize(cap.name("units").unwrap().as_str()),
            hp: parse_isize(cap.name("hp").unwrap().as_str()),
            immunities,
            weaknesses,
            dmg: parse_isize(cap.name("dmg").unwrap().as_str()),
            atk_type: String::from(cap.name("atk_type").unwrap().as_str()),
            initiative: parse_isize(cap.name("initiative").unwrap().as_str()),
            army,
        }
    }
}

fn parse_isize(s: &str) -> isize {
    s.parse::<isize>().unwrap()
}

fn main() -> Result<(), Box<dyn Error>> {
    let part = env::args()
        .nth(1)
        .expect("Expected argument specifying problem part `a` or `b`");
    if part == "a" {
        a()
    } else {
        b()
    }
}
