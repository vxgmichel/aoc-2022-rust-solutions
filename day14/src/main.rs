use std::env;
use std::io::{self, BufRead};
use std::thread::sleep;
use std::time::Duration;
extern crate termsize;

type Position = (usize, usize);
type Grid = Vec<Vec<char>>;

const EMPTY: char = ' ';
const ROCK: char = '\u{2588}';
const SAND: char = '\u{2591}';

fn parse_line(line: &str) -> Vec<Position> {
    line.split(" -> ")
        .map(|x| {
            let (a, b) = x.split_once(',').unwrap();
            (a.parse().unwrap(), b.parse().unwrap())
        })
        .collect()
}

fn draw_segment(grid: &mut Grid, start: Position, stop: Position, offset_x: usize) {
    let (start_x, start_y) = start;
    let (stop_x, stop_y) = stop;
    if (start_x as i32 - stop_x as i32).abs() + (start_y as i32 - stop_y as i32).abs() <= 1 {
        grid[start_x - offset_x][start_y] = ROCK;
        grid[stop_x - offset_x][stop_y] = ROCK;
        return;
    }
    let mid = ((start_x + stop_x) / 2, (start_y + stop_y) / 2);
    draw_segment(grid, start, mid, offset_x);
    draw_segment(grid, mid, stop, offset_x);
}

fn add_sand(grid: &mut Grid, init: Position) -> Option<Position> {
    let (mut x, mut y) = init;
    if grid[x][y] != EMPTY {
        return None;
    }
    loop {
        let a = *grid.get(x - 1)?.get(y + 1)?;
        let b = *grid.get(x)?.get(y + 1)?;
        let c = *grid.get(x + 1)?.get(y + 1)?;
        match (a, b, c) {
            (_, EMPTY, _) => y += 1,
            (EMPTY, ROCK | SAND, _) => {
                y += 1;
                x -= 1
            }
            (ROCK | SAND, ROCK | SAND, EMPTY) => {
                y += 1;
                x += 1
            }
            (ROCK | SAND, ROCK | SAND, ROCK | SAND) => {
                grid[x][y] = SAND;
                return Some((x, y));
            }
            _ => panic!(),
        }
    }
}

fn print_grid(grid: &Grid, init: Position) {
    let size = termsize::get().unwrap();
    let m = grid.len();
    let n = grid[0].len();
    let mut s = String::new();
    let start: usize = (init.0 as i32 - size.cols as i32 / 4)
        .try_into()
        .unwrap_or_default();
    let stop = m.min(init.0 + size.cols as usize / 4);
    s.push_str("\x1b[1;1H");
    for j in 0..n.min(size.rows as usize - 1) {
        for row in grid.iter().take(stop).skip(start) {
            s.push(row[j]);
            s.push(row[j]);
        }
        s.push('\n');
    }
    print!("{}", s);
}

fn run_grid(grid: &mut Grid, init: Position, debug: bool) -> u32 {
    if debug {
        print!("\x1b[?1049h\x1b[?25l");
    }
    for step in 0.. {
        if add_sand(grid, init).is_none() {
            if debug {
                sleep(Duration::from_secs(1));
                print!("\x1b[?1049l\x1b[?25h");
            }
            return step;
        };
        if debug {
            print_grid(grid, init);
            //sleep(Duration::from_millis(1))
        }
    }
    panic!()
}

fn make_grid(m: usize, n: usize, paths: &[Vec<Position>], offset_x: usize) -> Grid {
    let mut grid: Grid = (0..m).map(|_| (0..n).map(|_| ' ').collect()).collect();
    for path in paths {
        for points in path.windows(2) {
            let start = points[0];
            let stop = points[1];
            draw_segment(&mut grid, start, stop, offset_x);
        }
    }
    grid
}

fn solve1(paths: &[Vec<Position>], debug: bool) -> u32 {
    let minx = paths
        .iter()
        .flat_map(|x| x.iter().map(|(x, _)| x))
        .min()
        .unwrap();
    let maxx = paths
        .iter()
        .flat_map(|x| x.iter().map(|(x, _)| x))
        .max()
        .unwrap();
    let maxy = paths
        .iter()
        .flat_map(|x| x.iter().map(|(_, y)| y))
        .max()
        .unwrap();
    let offset_x = minx - 2;
    let m = maxx - offset_x + 1;
    let n = maxy + 1;
    let mut grid = make_grid(m, n, paths, offset_x);
    run_grid(&mut grid, (500 - offset_x, 0), debug)
}

fn solve2(paths: &[Vec<Position>], debug: bool) -> u32 {
    let maxy = paths
        .iter()
        .flat_map(|x| x.iter().map(|(_, y)| y))
        .max()
        .unwrap();
    let m = 1000;
    let n = maxy + 3;
    let mut grid = make_grid(m, n, paths, 0);
    for row in grid.iter_mut().take(m) {
        row[maxy + 2] = ROCK
    }
    run_grid(&mut grid, (500, 0), debug)
}

fn main() {
    let debug = env::args().any(|x| x == "--debug" || x == "-d");
    let vec: Vec<Vec<Position>> = io::stdin()
        .lock()
        .lines()
        .map(|x| parse_line(&x.unwrap()))
        .collect();
    println!("Part 1: {}", solve1(&vec, debug));
    println!("Part 2: {}", solve2(&vec, debug));
}
