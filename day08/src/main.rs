use std::collections::HashSet;
use std::io::{self, BufRead};

// Part 1

fn visible(row: &[u32]) -> HashSet<usize> {
    let mut current: i32 = -1;
    let mut result = HashSet::new();
    for (i, &value) in row.iter().enumerate() {
        if value as i32 > current {
            current = value as i32;
            result.insert(i);
        }
    }
    current = -1;
    for (i, &value) in row.iter().enumerate().rev() {
        if value as i32 > current {
            current = value as i32;
            result.insert(i);
        }
    }
    result
}

fn horizontal(grid: &[Vec<u32>]) -> HashSet<(usize, usize)> {
    grid.iter()
        .enumerate()
        .flat_map(|(i, x)| visible(x).iter().map(move |j| (i, *j)).collect::<Vec<_>>())
        .collect()
}

fn solve(grid: &[Vec<u32>]) -> u32 {
    let m = grid.len();
    let n = grid[0].len();
    let transposed_grid: Vec<Vec<u32>> = (0..n)
        .map(|j| (0..m).map(|i| grid[i][j]).collect())
        .collect();
    let mut set: HashSet<(usize, usize)> = horizontal(grid);
    set.extend(horizontal(&transposed_grid).iter().map(|&(i, j)| (j, i)));
    set.len() as u32
}

// Part 2

fn distance(iter: &mut dyn Iterator<Item = &u32>) -> u32 {
    let start = iter.next().unwrap();
    let mut result = 0;
    for x in iter {
        result += 1;
        if x >= start {
            break;
        }
    }
    result
}

fn score(grid: &[Vec<u32>], i: usize, j: usize) -> u32 {
    let m = grid.len();
    let n = grid[0].len();
    let transposed: Vec<Vec<u32>> = (0..n)
        .map(|j| (0..m).map(|i| grid[i][j]).collect())
        .collect();
    let mut a = grid[i][j..n].iter();
    let mut b = grid[i][0..=j].iter().rev();
    let mut c = transposed[j][i..m].iter();
    let mut d = transposed[j][0..=i].iter().rev();
    distance(&mut a) * distance(&mut b) * distance(&mut c) * distance(&mut d)
}

fn solve2(grid: &[Vec<u32>]) -> u32 {
    let m = grid.len();
    let n = grid[0].len();
    (0..m)
        .flat_map(|i| (0..n).map(move |j| score(grid, i, j)))
        .max()
        .unwrap()
}

fn main() {
    let grid: Vec<Vec<u32>> = io::stdin()
        .lock()
        .lines()
        .map(|x| {
            x.unwrap()
                .chars()
                .map(|x| x.to_digit(10).unwrap())
                .collect()
        })
        .collect();

    println!("Part 1: {}", solve(&grid));
    println!("Part 2: {}", solve2(&grid));
}
