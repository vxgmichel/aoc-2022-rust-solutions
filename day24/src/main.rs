use std::collections::HashSet;
use std::io::{self, BufRead};

type Position = (usize, usize);

fn is_position_available(grid: &[Vec<char>], (x, y): Position, time: usize) -> bool {
    let m = grid.len() as i32 - 2;
    let n = grid[0].len() as i32 - 2;
    let a = x as i32 - 1;
    let b = y as i32 - 1;
    let up = ((a + time as i32).rem_euclid(m), b);
    let down = ((a - time as i32).rem_euclid(m), b);
    let left = (a, (b + time as i32).rem_euclid(n));
    let right = (a, (b - time as i32).rem_euclid(n));
    let up = grid[up.0 as usize + 1][up.1 as usize + 1];
    let down = grid[down.0 as usize + 1][down.1 as usize + 1];
    let left = grid[left.0 as usize + 1][left.1 as usize + 1];
    let right = grid[right.0 as usize + 1][right.1 as usize + 1];
    up != '^' && down != 'v' && left != '<' && right != '>'
}

fn neighbors(grid: &[Vec<char>], (x, y): Position) -> Vec<Position> {
    let mut result = vec![(x, y)];
    let m = grid.len();
    let n = grid[0].len();
    if 1 < x {
        result.push((x - 1, y))
    }
    if x < m - 2 {
        result.push((x + 1, y))
    }
    if 1 < y {
        result.push((x, y - 1))
    }
    if y < n - 2 {
        result.push((x, y + 1))
    }
    result
}

fn solve(
    grid: &[Vec<char>],
    start_position: Position,
    stop_position: Position,
    start_time: usize,
) -> usize {
    let mut positions: HashSet<Position> = HashSet::new();

    for time in (start_time + 1).. {
        let mut new_positions: HashSet<Position> = HashSet::new();
        if is_position_available(grid, start_position, time) {
            new_positions.insert(start_position);
        }
        for position in positions {
            for neighbor in neighbors(grid, position) {
                if is_position_available(grid, neighbor, time) {
                    new_positions.insert(neighbor);
                }
            }
        }
        positions = new_positions;
        if positions.contains(&stop_position) {
            return time + 1;
        }
    }
    panic!()
}

fn solve1(grid: &[Vec<char>]) -> usize {
    let start_position = (1, 1);
    let stop_position = (grid.len() - 2, grid[0].len() - 2);
    solve(grid, start_position, stop_position, 0)
}

fn solve2(grid: &[Vec<char>]) -> usize {
    let start_position = (1, 1);
    let stop_position = (grid.len() - 2, grid[0].len() - 2);
    let t1 = solve(grid, start_position, stop_position, 0);
    let t2 = solve(grid, stop_position, start_position, t1);
    solve(grid, start_position, stop_position, t2)
}

fn main() {
    let vec: Vec<Vec<char>> = io::stdin()
        .lock()
        .lines()
        .map(|x| x.unwrap().chars().collect())
        .collect();
    println!("Part 1: {}", solve1(&vec));
    println!("Part 2: {}", solve2(&vec));
}
