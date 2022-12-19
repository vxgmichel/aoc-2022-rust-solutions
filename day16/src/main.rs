use std::collections::{hash_map, HashMap, HashSet};
use std::io::{self, BufRead};

type Name = [char; 2];

#[derive(Debug, PartialEq, Eq, Clone)]
struct Valve {
    name: Name,
    flow: u32,
    valves: Vec<Name>,
}

fn parse_line(line: &str) -> Valve {
    let vec = line.split(' ').collect::<Vec<_>>();
    let name: Vec<char> = vec[1].chars().collect();
    let name = name.try_into().unwrap();
    let flow = vec[4];
    let flow = flow[5..flow.len() - 1].parse().unwrap();
    let valves = vec[9..]
        .iter()
        .map(|x| {
            x.trim_end_matches(',')
                .chars()
                .collect::<Vec<_>>()
                .try_into()
                .unwrap()
        })
        .collect();
    Valve { name, flow, valves }
}

fn all_paths_from(map: &HashMap<Name, &Valve>, name: Name) -> HashMap<Name, u32> {
    let mut result: HashMap<Name, u32> = HashMap::new();
    let mut current_names: HashSet<Name> = [name].into_iter().collect();
    for i in 0.. {
        if current_names.is_empty() {
            break;
        }
        let mut new_current_names: HashSet<Name> = HashSet::new();
        for current in current_names {
            if let hash_map::Entry::Vacant(e) = result.entry(current) {
                e.insert(i + 1);
                new_current_names.extend(map[&current].valves.iter())
            }
        }
        current_names = new_current_names;
    }
    result
        .into_iter()
        .filter(|(name, _)| map[name].flow > 0)
        .collect()
}

fn rec(
    map: &HashMap<Name, &Valve>,
    costs: &HashMap<Name, HashMap<Name, u32>>,
    state: Name,
    time: u32,
    opened: &HashSet<Name>,
) -> u32 {
    let mut result = 0;
    for (&dest, &cost) in &costs[&state] {
        if cost < time && !opened.contains(&dest) {
            let new_time = time - cost;
            let gain = new_time * map[&dest].flow;
            let mut new_opened = opened.clone();
            new_opened.insert(dest);
            result = result.max(gain + rec(map, costs, dest, new_time, &new_opened));
        }
    }
    result
}

fn solve1(xs: &[Valve]) -> u32 {
    let map: HashMap<Name, &Valve> = xs.iter().map(|x| (x.name, x)).collect();
    let costs: HashMap<Name, HashMap<Name, u32>> = xs
        .iter()
        .map(|x| (x.name, all_paths_from(&map, x.name)))
        .collect();
    let start = ['A', 'A'];
    rec(&map, &costs, start, 30, &HashSet::new())
}

fn rec2(
    destinations: &[Vec<(usize, u32)>],
    flows: &[u32],
    state0: usize,
    state1: usize,
    time0: u32,
    time1: u32,
    opened: u32,
) -> u32 {
    let mut result = 0;
    if time0 >= time1 {
        for &(dest, cost) in &destinations[state0] {
            if cost < time0 && opened & (1 << dest) == 0 {
                let new_time = time0 - cost;
                let gain = new_time * flows[dest];
                let new_opened = opened | (1 << dest);
                result = result.max(
                    gain + rec2(
                        destinations,
                        flows,
                        dest,
                        state1,
                        new_time,
                        time1,
                        new_opened,
                    ),
                );
            }
        }
    } else {
        for &(dest, cost) in &destinations[state1] {
            if cost < time1 && opened & (1 << dest) == 0 {
                let new_time = time1 - cost;
                let gain = new_time * flows[dest];
                let new_opened = opened | (1 << dest);
                result = result.max(
                    gain + rec2(
                        destinations,
                        flows,
                        state0,
                        dest,
                        time0,
                        new_time,
                        new_opened,
                    ),
                );
            }
        }
    }
    result
}

fn solve2(all_valves: &[Valve]) -> u32 {
    let start = ['A', 'A'];
    let map: HashMap<Name, &Valve> = all_valves.iter().map(|x| (x.name, x)).collect();
    let mut valves: Vec<&Valve> = all_valves.iter().filter(|x| x.flow > 0).collect();
    // Add starting point if it's not part of the actual valves
    if map[&start].flow == 0 {
        valves.push(map[&start]);
    }
    let name_to_index: HashMap<Name, usize> = valves
        .iter()
        .enumerate()
        .map(|(i, x)| (x.name, i))
        .collect();
    let destinations: Vec<Vec<(usize, u32)>> = valves
        .iter()
        .map(|x| {
            all_paths_from(&map, x.name)
                .into_iter()
                .map(|(name, value)| (name_to_index[&name], value))
                .collect()
        })
        .collect();
    let flows: Vec<u32> = valves.iter().map(|x| x.flow).collect();
    let start_index = name_to_index[&start];
    rec2(&destinations, &flows, start_index, start_index, 26, 26, 0)
}

fn main() {
    let vec: Vec<Valve> = io::stdin()
        .lock()
        .lines()
        .map(|x| parse_line(&x.unwrap()))
        .collect();
    println!("Part 1: {}", solve1(&vec));
    println!("Part 2: {}", solve2(&vec));
}
