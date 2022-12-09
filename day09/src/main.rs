use std::collections::HashSet;
use std::io::{self, BufRead};

type Position = (i32, i32);

fn solve(xs: &[(char, u32)], length: usize) -> u32 {
    let mut body: Vec<Position> = vec![(0, 0); length];
    let mut set: HashSet<Position> = [*body.last().unwrap()].into_iter().collect();
    for &(dir, value) in xs {
        for _ in 0..value {
            match dir {
                'U' => body[0] = (body[0].0 + 1, body[0].1),
                'D' => body[0] = (body[0].0 - 1, body[0].1),
                'L' => body[0] = (body[0].0, body[0].1 - 1),
                'R' => body[0] = (body[0].0, body[0].1 + 1),
                _ => panic!(),
            }
            for i in 0..(length - 1) {
                let head = body[i];
                let tail = body.get_mut(i + 1).unwrap();
                let dx = (head.0 - tail.0).abs();
                let dy = (head.1 - tail.1).abs();
                match (dx, dy) {
                    (2, 2) => *tail = ((head.0 + tail.0) / 2, (head.1 + tail.1) / 2),
                    (2, 0 | 1) => *tail = ((head.0 + tail.0) / 2, head.1),
                    (0 | 1, 2) => *tail = (head.0, (head.1 + tail.1) / 2),
                    (0 | 1, 0 | 1) => {}
                    (a, b) => panic!("{} {}", a, b),
                }
            }
            set.insert(*body.last().unwrap());
        }
    }
    set.len() as u32
}

fn main() {
    let vec: Vec<(char, u32)> = io::stdin()
        .lock()
        .lines()
        .map(|x| x.unwrap())
        .map(|x| {
            let (a, b) = x.split_once(' ').unwrap();
            let dir = a.chars().next().unwrap();
            let value = b.parse().unwrap();
            (dir, value)
        })
        .collect();
    println!("Part 1: {}", solve(&vec, 2));
    println!("Part 2: {}", solve(&vec, 10));
}
