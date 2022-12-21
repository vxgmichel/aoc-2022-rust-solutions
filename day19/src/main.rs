use hashbrown::HashMap;
use std::env;
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
    const STORE: u8 = 25;

    fn tick(&mut self) -> Option<u8> {
        if self.time == 0 {
            return None;
        }
        self.time -= 1;
        self.ore = (self.ore + self.ore_robots).min(State::STORE);
        self.clay = (self.clay + self.clay_robots).min(State::STORE);
        self.obsidian = (self.obsidian + self.obsidian_robots).min(State::STORE);
        Some(0)
    }

    fn build_ore_robot(&mut self, blueprint: &Blueprint) -> Option<u8> {
        if self.ore + self.ore_robots * (self.time - 2) < blueprint.0 {
            return None;
        }
        while self.ore < blueprint.0 {
            self.tick()?;
        }
        self.ore -= blueprint.0;
        self.tick()?;
        self.ore_robots += 1;
        Some(0)
    }

    fn build_clay_robot(&mut self, blueprint: &Blueprint) -> Option<u8> {
        if self.ore + self.ore_robots * (self.time - 2) < blueprint.1 {
            return None;
        }
        while self.ore < blueprint.1 {
            self.tick()?;
        }
        self.ore -= blueprint.1;
        self.tick()?;
        self.clay_robots += 1;
        Some(0)
    }

    fn build_obsidian_robot(&mut self, blueprint: &Blueprint) -> Option<u8> {
        if self.ore + self.ore_robots * (self.time - 2) < blueprint.2
            || self.clay + self.clay_robots * (self.time - 2) < blueprint.3
        {
            return None;
        }
        while self.ore < blueprint.2 || self.clay < blueprint.3 {
            self.tick()?;
        }
        self.ore -= blueprint.2;
        self.clay -= blueprint.3;
        self.tick()?;
        self.obsidian_robots += 1;
        Some(0)
    }

    fn build_geode_robot(&mut self, blueprint: &Blueprint) -> Option<u8> {
        if self.ore + self.ore_robots * (self.time - 2) < blueprint.4
            || self.obsidian + self.obsidian_robots * (self.time - 2) < blueprint.5
        {
            return None;
        }
        while self.ore < blueprint.4 || self.obsidian < blueprint.5 {
            self.tick()?;
        }
        self.ore -= blueprint.4;
        self.obsidian -= blueprint.5;
        self.tick()?;
        Some(self.time)
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
    if state.time < 2 {
        return 0;
    }
    let mut result = 0;
    let mut new_state = state;
    if let Some(x) = new_state.build_geode_robot(blueprint) {
        result = result.max(x + rec(blueprint, new_state, cache, cache_hit))
    }
    let mut new_state = state;
    if let Some(x) = new_state.build_obsidian_robot(blueprint) {
        result = result.max(x + rec(blueprint, new_state, cache, cache_hit));
    }
    let mut new_state = state;
    if let Some(x) = new_state.build_clay_robot(blueprint) {
        result = result.max(x + rec(blueprint, new_state, cache, cache_hit))
    }
    let mut new_state = state;
    if let Some(x) = new_state.build_ore_robot(blueprint) {
        result = result.max(x + rec(blueprint, new_state, cache, cache_hit))
    }
    cache.insert(state, result);
    result
}

fn solve1(blueprints: &[Blueprint], time: u8, debug: bool) -> u64 {
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
            if debug {
                println!(
                    "[{}] {:?} -> {} (miss={}, hit={})",
                    i + 1,
                    x,
                    result,
                    cache.len(),
                    cache_hit
                );
            }
            (i as u64 + 1) * result as u64
        })
        .sum()
}

fn solve2(blueprints: &[Blueprint], time: u8, debug: bool) -> u64 {
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
            if debug {
                println!("[{}] {:?} -> {}", i + 1, x, result);
            }
            result as u64
        })
        .product()
}

fn main() {
    let debug = env::args().any(|x| x == "--debug" || x == "-d");
    let vec: Vec<Blueprint> = io::stdin()
        .lock()
        .lines()
        .map(|x| parse_line(&x.unwrap()))
        .collect();
    println!("Part 1: {}", solve1(&vec, 24, debug));
    println!("Part 2: {}", solve2(&vec[..3], 32, debug));
}
