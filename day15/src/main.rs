use std::collections::HashSet;
use std::io::{self, BufRead};

type Position = (i32, i32);
type Sensor = (Position, Position);

fn parse_line(line: &str) -> Sensor {
    let vec: Vec<&str> = line.split(' ').collect();
    let x1 = vec[2];
    let x1 = x1[2..x1.len() - 1].parse().unwrap();
    let y1 = vec[3];
    let y1 = y1[2..y1.len() - 1].parse().unwrap();
    let x2 = vec[8];
    let x2 = x2[2..x2.len() - 1].parse().unwrap();
    let y2 = vec[9][2..].parse().unwrap();
    ((x1, y1), (x2, y2))
}

fn build_segments(xs: &[Sensor], row: i32) -> Vec<(i32, i32)> {
    // List segments
    let mut segments: Vec<(i32, i32)> = vec![];
    for &((x1, y1), (x2, y2)) in xs {
        let d = (x1 - x2).abs() + (y1 - y2).abs();
        let extra = d - (row - y1).abs();
        if extra < 0 {
            continue;
        }
        let left = x1 - extra;
        let right = x1 + extra;
        segments.push((left, right + 1));
    }
    segments.sort();
    segments
}

fn solve1(xs: &[Sensor], row: i32) -> i32 {
    let segments = build_segments(xs, row);

    // Flatten segments
    let mut result = 0;
    let mut current = -100_000_000;
    for (a, b) in segments {
        let a = a.max(current);
        if a >= b {
            continue;
        }
        result += b - a;
        current = b;
    }

    // Remove beacons
    let mut beacons: HashSet<Position> = HashSet::new();
    for &((_, y1), beacon) in xs {
        if y1 == row {
            result -= 1;
        }
        beacons.insert(beacon);
    }
    for (_, y) in beacons {
        if y == row {
            result -= 1
        }
    }
    result
}

fn detect_gap(xs: &[Sensor], lower: i32, upper: i32, row: i32) -> Option<i32> {
    // List segments
    let segments = build_segments(xs, row);

    // Flatten segments
    let mut current = lower;
    for (a, b) in segments {
        if upper < current {
            break;
        }
        if current < a {
            return Some(current);
        }
        if current < b {
            current = b;
        }
    }
    None
}

fn solve2(xs: &[Sensor], lower: i32, upper: i32) -> i64 {
    for row in lower..=upper {
        if let Some(x) = detect_gap(xs, lower, upper, row) {
            return upper as i64 * x as i64 + row as i64;
        }
    }
    panic!()
}

fn main() {
    let vec: Vec<Sensor> = io::stdin()
        .lock()
        .lines()
        .map(|x| parse_line(&x.unwrap()))
        .collect();
    println!("Part 1: {}", solve1(&vec, 2_000_000));
    println!("Part 2: {}", solve2(&vec, 0, 4_000_000));
}
