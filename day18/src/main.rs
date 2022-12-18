use std::collections::HashSet;
use std::io::{self, BufRead};

type Position = (i32, i32, i32);

fn update_set(s: &mut HashSet<Position>, position: Position) {
    if !s.insert(position) {
        s.remove(&position);
    }
}

fn solve1(rocks: &[Position]) -> usize {
    let mut x_sides = HashSet::new();
    let mut y_sides = HashSet::new();
    let mut z_sides = HashSet::new();
    for &(x, y, z) in rocks {
        update_set(&mut x_sides, (x, y, z));
        update_set(&mut y_sides, (x, y, z));
        update_set(&mut z_sides, (x, y, z));
        update_set(&mut x_sides, (x + 1, y, z));
        update_set(&mut y_sides, (x, y + 1, z));
        update_set(&mut z_sides, (x, y, z + 1));
    }
    x_sides.len() + y_sides.len() + z_sides.len()
}

fn walk_rec(
    rocks: &HashSet<Position>,
    position: Position,
    min: i32,
    max: i32,
    result: &mut HashSet<Position>,
) {
    let (x, y, z) = position;
    if x < min
        || x > max
        || y < min
        || y > max
        || z < min
        || z > max
        || rocks.contains(&position)
        || result.contains(&position)
    {
        return;
    }
    result.insert(position);
    walk_rec(rocks, (x + 1, y, z), min, max, result);
    walk_rec(rocks, (x - 1, y, z), min, max, result);
    walk_rec(rocks, (x, y + 1, z), min, max, result);
    walk_rec(rocks, (x, y - 1, z), min, max, result);
    walk_rec(rocks, (x, y, z + 1), min, max, result);
    walk_rec(rocks, (x, y, z - 1), min, max, result);
}

fn solve2(rocks: &[Position]) -> usize {
    let min = rocks
        .iter()
        .cloned()
        .flat_map(|(a, b, c)| [a, b, c])
        .min()
        .unwrap()
        - 1;
    let max = rocks
        .iter()
        .cloned()
        .flat_map(|(a, b, c)| [a, b, c])
        .max()
        .unwrap()
        + 1;
    let rock_set: HashSet<Position> = rocks.iter().cloned().collect();
    let mut empty_sets: Vec<HashSet<Position>> = vec![];
    for x in min..=max {
        for y in min..=max {
            for z in min..=max {
                let position = (x, y, z);
                if rock_set.contains(&position) || empty_sets.iter().any(|x| x.contains(&position))
                {
                    continue;
                }
                let mut new_empty_set: HashSet<Position> = HashSet::new();
                walk_rec(&rock_set, position, min, max, &mut new_empty_set);
                empty_sets.push(new_empty_set);
            }
        }
    }

    let outside_ref = (min, min, min);
    assert!(!rock_set.contains(&outside_ref));
    let mut new_rocks: Vec<Position> = rocks.to_vec();
    for empty_set in empty_sets {
        if !empty_set.contains(&outside_ref) {
            new_rocks.extend(empty_set);
        }
    }
    solve1(&new_rocks)
}

fn main() {
    let vec: Vec<Position> = io::stdin()
        .lock()
        .lines()
        .map(|x| {
            let line = x.unwrap();
            let mut iter = line.split(',').map(|x| x.parse().unwrap());
            (
                iter.next().unwrap(),
                iter.next().unwrap(),
                iter.next().unwrap(),
            )
        })
        .collect();
    println!("Part 1: {}", solve1(&vec));
    println!("Part 2: {}", solve2(&vec));
}
