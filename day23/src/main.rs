use std::collections::{HashMap, HashSet};
use std::env;
use std::io::{self, BufRead};
use std::thread::sleep;
use std::time::Duration;

type Position = (i32, i32);

fn next_position(set: &HashSet<Position>, position: Position, step: u8) -> Position {
    let (x, y) = position;
    let nw = set.contains(&(x - 1, y - 1));
    let nn = set.contains(&(x - 1, y));
    let ne = set.contains(&(x - 1, y + 1));
    let ww = set.contains(&(x, y - 1));
    let ee = set.contains(&(x, y + 1));
    let sw = set.contains(&(x + 1, y - 1));
    let sn = set.contains(&(x + 1, y));
    let se = set.contains(&(x + 1, y + 1));
    match (step, nw, nn, ne, ww, ee, sw, sn, se) {
        (_, false, false, false, false, false, false, false, false) => position,
        (0, false, false, false, _, _, _, _, _) => (x - 1, y),
        (0, _, _, _, _, _, false, false, false) => (x + 1, y),
        (0, false, _, _, false, _, false, _, _) => (x, y - 1),
        (0, _, _, false, _, false, _, _, false) => (x, y + 1),
        (1, _, _, _, _, _, false, false, false) => (x + 1, y),
        (1, false, _, _, false, _, false, _, _) => (x, y - 1),
        (1, _, _, false, _, false, _, _, false) => (x, y + 1),
        (1, false, false, false, _, _, _, _, _) => (x - 1, y),
        (2, false, _, _, false, _, false, _, _) => (x, y - 1),
        (2, _, _, false, _, false, _, _, false) => (x, y + 1),
        (2, false, false, false, _, _, _, _, _) => (x - 1, y),
        (2, _, _, _, _, _, false, false, false) => (x + 1, y),
        (3, _, _, false, _, false, _, _, false) => (x, y + 1),
        (3, false, false, false, _, _, _, _, _) => (x - 1, y),
        (3, _, _, _, _, _, false, false, false) => (x + 1, y),
        (3, false, _, _, false, _, false, _, _) => (x, y - 1),
        (_, _, _, _, _, _, _, _, _) => position,
    }
}

fn make_plan(set: &HashSet<Position>, step: u8) -> HashMap<Position, Position> {
    let mut map = HashMap::new();
    for &position in set {
        map.insert(position, next_position(set, position, step));
    }
    map
}

fn print_grid(set: &HashSet<Position>) {
    let mut result = String::new();
    let size = termsize::get().unwrap();
    let minx = -18;
    let miny = -18;
    let maxx = minx + size.rows as i32 - 2;
    let maxy = miny + size.cols as i32 / 2 - 2;
    result.push_str("\x1b[1;1H");
    for x in minx..=maxx {
        for y in miny..=maxy {
            if set.contains(&(x, y)) {
                result.push_str("██");
            } else {
                result.push_str("  ");
            }
        }
        result.push('\n');
    }
    print!("{}", result);
}

fn solve1(grid: &[Vec<char>]) -> i32 {
    let mut set = HashSet::new();
    for (i, row) in grid.iter().enumerate() {
        for (j, &c) in row.iter().enumerate() {
            if c == '#' {
                set.insert((i as i32, j as i32));
            }
        }
    }
    for i in 0..10 {
        let plan = make_plan(&set, i % 4);
        let mut count_positions = HashMap::new();
        for position in plan.values() {
            *count_positions.entry(position).or_insert(0) += 1;
        }
        set.clear();
        for (&source, &dest) in &plan {
            set.insert(if count_positions[&dest] == 1 {
                dest
            } else {
                source
            });
        }
    }
    let minx = set.iter().map(|(x, _)| x).min().unwrap();
    let miny = set.iter().map(|(_, y)| y).min().unwrap();
    let maxx = set.iter().map(|(x, _)| x).max().unwrap();
    let maxy = set.iter().map(|(_, y)| y).max().unwrap();
    (maxx - minx + 1) * (maxy - miny + 1) - set.len() as i32
}

fn solve2(grid: &[Vec<char>], debug: bool) -> i32 {
    let mut set = HashSet::new();
    for (i, row) in grid.iter().enumerate() {
        for (j, &c) in row.iter().enumerate() {
            if c == '#' {
                set.insert((i as i32, j as i32));
            }
        }
    }
    if debug {
        print!("\x1b[?1049h\x1b[?25l");
        print_grid(&set);
    }
    for i in 0.. {
        let plan = make_plan(&set, i as u8 % 4);
        let mut count_positions = HashMap::new();
        for position in plan.values() {
            *count_positions.entry(position).or_insert(0) += 1;
        }
        let new_set = plan
            .iter()
            .map(|(source, dest)| {
                if count_positions[&dest] == 1 {
                    dest
                } else {
                    source
                }
            })
            .cloned()
            .collect();
        if set == new_set {
            if debug {
                sleep(Duration::from_secs(1));
                print!("\x1b[?1049l\x1b[?25h");
            }
            return i as i32 + 1;
        }
        set = new_set;
        if debug {
            sleep(Duration::from_millis(1000 / 30));
            print_grid(&set);
        }
    }
    panic!()
}

fn main() {
    let debug = env::args().any(|x| x == "--debug" || x == "-d");
    let vec: Vec<Vec<char>> = io::stdin()
        .lock()
        .lines()
        .map(|x| x.unwrap().chars().collect())
        .collect();
    println!("Part 1: {}", solve1(&vec));
    println!("Part 1: {}", solve2(&vec, debug));
}
