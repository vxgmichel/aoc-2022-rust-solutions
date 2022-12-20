use hashbrown::HashMap;
use std::io::{self, BufRead};

type Blueprint = (u8, u8, u8, u8, u8, u8);

#[derive(Hash, Clone, Copy, PartialEq, Eq, Debug)]
struct State {
    time: u8,
    ore_robots: u8,
    clay_robots: u8,
    obsidian_robots: u8,
    ore: u8,
    clay: u8,
    obsidian: u8,
}

impl State {
    fn tick(&self) -> State {
        State {
            time: self.time - 1,
            ore_robots: self.ore_robots,
            clay_robots: self.clay_robots,
            obsidian_robots: self.obsidian_robots,
            ore: self.ore + self.ore_robots,
            clay: self.clay + self.clay_robots,
            obsidian: self.obsidian + self.obsidian_robots,
        }
    }

    fn can_build_ore_robot(&self, blueprint: &Blueprint) -> bool {
        self.ore >= blueprint.0
    }

    fn build_ore_robot(&self, blueprint: &Blueprint) -> State {
        State {
            time: self.time,
            ore_robots: self.ore_robots + 1,
            clay_robots: self.clay_robots,
            obsidian_robots: self.obsidian_robots,
            ore: self.ore - blueprint.0,
            clay: self.clay,
            obsidian: self.obsidian,
        }
    }

    fn can_build_clay_robot(&self, blueprint: &Blueprint) -> bool {
        self.ore >= blueprint.1
    }

    fn build_clay_robot(&self, blueprint: &Blueprint) -> State {
        State {
            time: self.time,
            ore_robots: self.ore_robots,
            clay_robots: self.clay_robots + 1,
            obsidian_robots: self.obsidian_robots,
            ore: self.ore - blueprint.1,
            clay: self.clay,
            obsidian: self.obsidian,
        }
    }

    fn can_build_obsidian_robot(&self, blueprint: &Blueprint) -> bool {
        self.ore >= blueprint.2 && self.clay >= blueprint.3
    }

    fn build_obsidian_robot(&self, blueprint: &Blueprint) -> State {
        State {
            time: self.time,
            ore_robots: self.ore_robots,
            clay_robots: self.clay_robots,
            obsidian_robots: self.obsidian_robots + 1,
            ore: self.ore - blueprint.2,
            clay: self.clay - blueprint.3,
            obsidian: self.obsidian,
        }
    }

    fn can_build_geode_robot(&self, blueprint: &Blueprint) -> bool {
        self.ore >= blueprint.4 && self.obsidian >= blueprint.5
    }

    fn build_geode_robot(&self, blueprint: &Blueprint) -> State {
        State {
            time: self.time,
            ore_robots: self.ore_robots,
            clay_robots: self.clay_robots,
            obsidian_robots: self.obsidian_robots,
            ore: self.ore - blueprint.4,
            clay: self.clay,
            obsidian: self.obsidian - blueprint.5,
        }
    }
}

fn parse_line(line: &str) -> Blueprint {
    let mut iter = line.split(' ');
    iter.next().unwrap();
    let a = iter.next().unwrap().parse().unwrap();
    let b = iter.next().unwrap().parse().unwrap();
    let c = iter.next().unwrap().parse().unwrap();
    let d = iter.next().unwrap().parse().unwrap();
    let e = iter.next().unwrap().parse().unwrap();
    let f = iter.next().unwrap().parse().unwrap();
    (a, b, c, d, e, f)
}

fn rec(
    blueprint: &Blueprint,
    state: State,
    cache: &mut HashMap<State, u8>,
    cache_hit: &mut u32,
) -> u8 {
    if let Some(&x) = cache.get(&state) {
        *cache_hit += 1;
        return x;
    }
    if state.time == 0 {
        return 0;
    }
    let new_state = state.tick();
    if state.can_build_geode_robot(blueprint) {
        let result = new_state.time
            + rec(
                blueprint,
                new_state.build_geode_robot(blueprint),
                cache,
                cache_hit,
            );
        cache.insert(state, result);
        return result;
    }
    let mut result = rec(blueprint, new_state, cache, cache_hit);
    if state.can_build_ore_robot(blueprint) {
        result = result.max(rec(
            blueprint,
            new_state.build_ore_robot(blueprint),
            cache,
            cache_hit,
        ))
    }
    if state.can_build_clay_robot(blueprint) {
        result = result.max(rec(
            blueprint,
            new_state.build_clay_robot(blueprint),
            cache,
            cache_hit,
        ))
    }
    if state.can_build_obsidian_robot(blueprint) {
        result = result.max(rec(
            blueprint,
            new_state.build_obsidian_robot(blueprint),
            cache,
            cache_hit,
        ))
    }
    cache.insert(state, result);
    result
}

fn solve1(blueprints: &[Blueprint], time: u8) -> u64 {
    let state = State {
        time,
        ore_robots: 1,
        clay_robots: 0,
        obsidian_robots: 0,
        ore: 0,
        clay: 0,
        obsidian: 0,
    };
    blueprints
        .iter()
        .enumerate()
        .map(|(i, x)| {
            let mut cache_hit = 0;
            let mut cache = HashMap::new();
            let result = rec(x, state, &mut cache, &mut cache_hit);
            println!(
                "[{}] {:?} -> {} (miss={}, hit={})",
                i + 1,
                x,
                result,
                cache.len(),
                cache_hit
            );
            (i as u64 + 1) * result as u64
        })
        .sum()
}

fn solve2(blueprints: &[Blueprint], time: u8) -> u64 {
    let state = State {
        time,
        ore_robots: 1,
        clay_robots: 0,
        obsidian_robots: 0,
        ore: 0,
        clay: 0,
        obsidian: 0,
    };
    blueprints
        .iter()
        .enumerate()
        .map(|(i, x)| {
            let mut cache_hit = 0;
            let mut cache = HashMap::new();
            let result = rec(x, state, &mut cache, &mut cache_hit);
            println!("[{}] {:?} -> {}", i + 1, x, result);
            result as u64
        })
        .product()
}

fn main() {
    let vec: Vec<Blueprint> = io::stdin()
        .lock()
        .lines()
        .map(|x| parse_line(&x.unwrap()))
        .collect();
    println!("Part 1: {}", solve1(&vec, 24));
    println!("Part 2: {}", solve2(&vec[..3], 32));
}
