use core::panic;
use std::borrow::BorrowMut;
use std::collections::hash_map::Entry;
use std::collections::HashMap;
use std::io::{self, BufRead};
use std::thread::sleep;
use std::time::Duration;

type Row = [char; 7];
type Position = (usize, usize);
const CHECK_DEPTH: usize = 64;

fn print_grid(grid: &[Row]) {
    let term_rows = termsize::get().unwrap().rows as usize;
    let mut string = String::new();
    string.push_str("\x1b[1;1H");
    let mut iter = grid.iter().take(term_rows * 2).rev();
    while let (Some(top), Some(bottom)) = (iter.next(), iter.next()) {
        string.push('█');
        for (top_char, bottom_char) in top.iter().zip(bottom.iter()) {
            string.push(match (top_char, bottom_char) {
                ('#', '#') => '█',
                (' ', '#') => '▄',
                ('#', ' ') => '▀',
                (' ', ' ') => ' ',
                _ => panic!(),
            })
        }
        string.push('█');
        string.push('\n');
    }
    string.push_str("▀▀▀▀▀▀▀▀▀\n\n");
    print!("{}", string);
}

fn collide(grid: &[Row], piece: &[Position], position: Position) -> bool {
    let (x, y) = position;
    piece
        .iter()
        .any(|(a, b)| grid[a + x].get(b + y).cloned().unwrap_or('#') != ' ')
}

fn solve(xs: &[char], n: u64, debug: bool) -> u64 {
    let mut grid = [[' '; 7]; 8000];
    let pieces = [
        vec![(0, 0), (0, 1), (0, 2), (0, 3)],
        vec![(0, 1), (1, 0), (1, 1), (1, 2), (2, 1)],
        vec![(0, 0), (0, 1), (0, 2), (1, 2), (2, 2)],
        vec![(0, 0), (1, 0), (2, 0), (3, 0)],
        vec![(0, 0), (0, 1), (1, 0), (1, 1)],
    ];
    let mut height = 0;
    let mut dir_iter = xs.iter().enumerate().cycle();
    let mut cache = HashMap::new();
    if debug {
        print!("\x1b[?1049h\x1b[?25l");
    }
    let mut extra_height = 0;
    let mut fallen_pieces = 0;
    for (piece_id, piece) in pieces.iter().enumerate().cycle() {
        // Loop over rock positions
        let mut position: Position = (height + 3, 2);
        for (i, (dir_id, dir)) in dir_iter.borrow_mut().enumerate() {
            // Check for existing state
            if i == 0 && height > CHECK_DEPTH && extra_height == 0 {
                let last_rows: [Row; CHECK_DEPTH] =
                    grid[height - CHECK_DEPTH..height].try_into().unwrap();
                let key = (piece_id, dir_id, last_rows);
                match cache.entry(key) {
                    Entry::Occupied(x) => {
                        let (old_fallen_pieces, old_height) = x.get();
                        let remaining = n - fallen_pieces - 1;
                        let step = fallen_pieces - old_fallen_pieces;
                        let steps = remaining / step;
                        fallen_pieces += steps * step;
                        extra_height = steps * (height - old_height) as u64;
                    }
                    Entry::Vacant(e) => {
                        e.insert((fallen_pieces, height));
                    }
                };
            }

            // Display
            if debug && height < 100 {
                for (x, y) in piece {
                    grid[position.0 + x][position.1 + y] = '#';
                }
                print_grid(&grid);
                for (x, y) in piece {
                    grid[position.0 + x][position.1 + y] = ' ';
                }
                sleep(Duration::from_millis(50));
            }

            // Move left/right
            //let (dir_id, dir) = dir_iter.next().unwrap();
            let new_position = match dir {
                '<' if position.1 > 0 => (position.0, position.1 - 1),
                '>' if position.1 < 6 => (position.0, position.1 + 1),
                '<' | '>' => (position.0, position.1),
                _ => panic!(),
            };
            if !collide(&grid, piece, new_position) {
                position = new_position
            }

            // Fall down
            if position.0 == 0 {
                break;
            }
            let new_position = (position.0 - 1, position.1);
            if collide(&grid, piece, new_position) {
                break;
            }
            position = new_position;
        }

        // Set the rock
        for (x, y) in piece {
            grid[position.0 + x][position.1 + y] = '#';
            height = height.max(position.0 + x + 1);
        }
        fallen_pieces += 1;

        // Check break condition
        if fallen_pieces == n {
            break;
        }
    }
    if debug {
        print!("\x1b[?1049l\x1b[?25h");
    }
    height as u64 + extra_height
}

fn main() {
    let vec: Vec<char> = io::stdin()
        .lock()
        .lines()
        .next()
        .unwrap()
        .unwrap()
        .chars()
        .collect();
    println!("Part 1: {}", solve(&vec, 2022, false));
    println!("Part 2: {}", solve(&vec, 1_000_000_000_000, false));
}
