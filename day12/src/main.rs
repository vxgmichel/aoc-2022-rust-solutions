use std::collections::{HashMap, HashSet};
use std::io::{self, BufRead};

type Node = (usize, usize);

fn neighbors(grid: &[Vec<u32>], node: Node) -> Vec<Node> {
    let m = grid.len();
    let n = grid[0].len();
    let mut result = vec![];
    let (x, y) = node;
    let hmin = grid[x][y] - 1;
    if x > 0 && grid[x - 1][y] >= hmin {
        result.push((x - 1, y));
    }
    if x < m - 1 && grid[x + 1][y] >= hmin {
        result.push((x + 1, y));
    }
    if y > 0 && grid[x][y - 1] >= hmin {
        result.push((x, y - 1));
    }
    if y < n - 1 && grid[x][y + 1] >= hmin {
        result.push((x, y + 1));
    }
    result
}

fn solve1(grid: &[Vec<u32>], start: Node, end: Node) -> u32 {
    let results = build(grid, end);
    results[&start]
}

fn build(grid: &[Vec<u32>], init: Node) -> HashMap<Node, u32> {
    let mut current_nodes: Vec<Node> = vec![init];
    let mut seen: HashSet<Node> = [init].into_iter().collect();
    let mut result = HashMap::new();
    for step in 0.. {
        if current_nodes.is_empty() {
            break;
        }
        let mut next_nodes = vec![];
        for node in current_nodes {
            result.insert(node, step);
            for neighbor in neighbors(grid, node) {
                if seen.insert(neighbor) {
                    next_nodes.push(neighbor);
                }
            }
        }
        current_nodes = next_nodes;
    }
    result
}

fn solve2(grid: &[Vec<u32>], end: Node) -> u32 {
    let results = &build(grid, end);
    grid.iter()
        .enumerate()
        .flat_map(|(i, row)| {
            row.iter().enumerate().filter_map(move |(j, x)| {
                if *x != 0 {
                    None
                } else {
                    results.get(&(i, j)).cloned()
                }
            })
        })
        .min()
        .unwrap()
}

fn main() {
    let mut start = None;
    let mut end = None;
    let vec: Vec<Vec<u32>> = io::stdin()
        .lock()
        .lines()
        .map(|x| x.unwrap())
        .enumerate()
        .map(|(i, x)| {
            x.chars()
                .enumerate()
                .map(|(j, y)| match y {
                    'S' => {
                        start = Some((i, j));
                        0
                    }
                    'E' => {
                        end = Some((i, j));
                        25
                    }
                    'a'..='z' => y as u32 - 'a' as u32,
                    _ => panic!(),
                })
                .collect()
        })
        .collect();
    let start = start.unwrap();
    let end = end.unwrap();
    println!("Part 1: {}", solve1(&vec, start, end));
    println!("Part 2: {}", solve2(&vec, end));
}
